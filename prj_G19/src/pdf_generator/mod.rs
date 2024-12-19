
// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: CC0-1.0

//! This example generates a demo PDF document and writes it to the path that was passed as the
//! first command-line argument.  You may have to adapt the `FONT_DIRS`, `DEFAULT_FONT_NAME` and
//! `MONO_FONT_NAME` constants for your system so that these files exist:
//! - `{FONT_DIR}/{name}-Regular.ttf`
//! - `{FONT_DIR}/{name}-Bold.ttf`
//! - `{FONT_DIR}/{name}-Italic.ttf`
//! - `{FONT_DIR}/{name}-BoldItalic.ttf`
//! for `name` in {`DEFAULT_FONT_NAME`, `MONO_FONT_NAME`}.
//!
//! The generated document using the latest `genpdf-rs` release is available
//! [here](https://genpdf-rs.ireas.org/examples/demo.pdf).

mod encoder;
mod chart_generator;

use std::{env, fs};
use std::time::Instant;
use genpdf::{Alignment, Margins,Document};
use genpdf::Element as _;
use genpdf::{elements, fonts};
use genpdf::elements::{CellDecorator, FrameCellDecorator, LinearLayout, Paragraph, TableLayout};
use genpdf::style::{Color, Effect,Style};
use crate::hardened::{bubble_sort_hardened, matrix_multiplication_hardened, selection_sort_hardened, Hardened};

use crate::fault_list_manager::file_fault_list::{bubble_sort,
                                                 matrix_multiplication,
                                                 selection_sort};
use crate::analyzer::{Analyzer, Faults};
use crate::fault_env::Data;

const FONT_DIRS: &[&str] = &[
    "src/pdf_generator/fonts/times_new_roman",
    "src/pdf_generator/fonts/times_new_roman",
];
const DEFAULT_FONT_NAME: &'static str = "TimesNewRoman";
const MONO_FONT_NAME: &'static str = "TimesNewRoman";
pub fn print_pdf_all(file_path: &String, data_list: Vec<Analyzer>){
    let text_margins = Margins::trbl(0, 65, 0, 5);
    let mut doc = setup_document();
    let top_headers =  vec!["SILENT","ASSIGN","MUL","GENERIC","ADD","IND_MUT","INDEX","ORD","PAR_ORD","PAR_EQ"];
    let side_headers = vec!["SELECTION SORT","BUBBLE SORT","MATRIX MULTIPLICATION"];
    let images_paths = gen_pie_chart(&data_list,&side_headers);
    add_image_to_pdf(images_paths,&mut doc);
    let fault_table = gen_table_faults(&data_list,&top_headers,&side_headers);
    doc.push(fault_table);
    let top_headers =  vec!["NOT HARDENED(Byte)", "HARDENED(Byte)", "HARD / NOT HARD","NOT HARDENED(uS)","HARDENED(uS)","HARD / NOT HARD"];

    let dim_time_table = gen_table_dim_time(&data_list,&top_headers,&side_headers);
    doc.push(elements::Break::new(1.5));
    doc.push(dim_time_table);

    doc.push(elements::Break::new(1.5));
    let path = gen_bar_chart(&data_list,&side_headers);
    doc.push(elements::Image::from_path(path).expect("Unable to load image").with_alignment(Alignment::Center));

    doc.render_to_file(file_path)
        .expect("Failed to write output file");


}


pub fn print_pdf_diffcard(file_path: &String, data_list: Vec<Analyzer>){
    let text_margins = Margins::trbl(0, 65, 0, 5);
    let mut doc = setup_document();

    let mut chart_headers:Vec<&str> = Vec::new();
    match data_list[0].target_program.as_str() {
        "sel_sort"=> for _ in 0..3 {chart_headers.push("SELECTION SORT")} ,
        "bubble_sort"=> for _ in 0..3 {chart_headers.push("BUBBLE SORT")},
        "matrix_multiplication"=> for _ in 0..3 {chart_headers.push("MATRIX MULTIPLICATION")},
        _ => {}
    }
    let images_paths = gen_pie_chart(&data_list, &chart_headers);
    add_image_to_pdf(images_paths,&mut doc);

    let top_headers =  vec!["SILENT","ASSIGN","MUL","GENERIC","ADD","IND_MUT","INDEX","ORD","PAR_ORD","PAR_EQ"];
    let side_headers = vec!["1000 FAULTS","2000 FAULTS","3000 FAULTS"];
    let fault_table = gen_table_faults(&data_list,&top_headers,&side_headers);
    doc.push(fault_table);

    let top_headers =  vec!["NOT HARDENED(Byte)", "HARDENED(Byte)", "HARD / NOT HARD","NOT HARDENED(uS)","HARDENED(uS)","HARD / NOT HARD"];
    let dim_time_table = gen_table_dim_time(&data_list,&top_headers,&side_headers);
    doc.push(elements::Break::new(1.5));
    doc.push(dim_time_table);

    doc.render_to_file(file_path)
        .expect("Failed to write output file");
}
pub fn print_pdf(file_path: &String, analyzer: Analyzer) {
    let text_margins = Margins::trbl(0, 65, 0, 5);
    let mut doc = setup_document();
    let top_headers =  vec!["SILENT","ASSIGN","MUL","GENERIC","ADD","IND_MUT","INDEX","ORD","PAR_ORD","PAR_EQ"];
    let mut data_list:Vec<Analyzer> = Vec::new();
    let mut side_headers:Vec<&str> = Vec::new();
    match analyzer.n_esecuzione {
        0=> side_headers.push("SELECTION SORT"),
        1=> side_headers.push("BUBBLE SORT"),
        2=> side_headers.push("MATRIX MULTIPLICATION"),
        _ => {}
    }
    data_list.push(analyzer.clone());

    let important_style =  Style::new().bold();
    let red = Color::Rgb(255, 0, 0);
    let text_margins= Margins::trbl(0, 65,0,0);

    doc.push(elements::Break::new(0.3));
    doc.push(elements::Paragraph::new("Tipologia di esperimento: SINGLE ANALYSIS ").padded(text_margins).styled(important_style));
    doc.push(elements::Paragraph::new(format!("Algortimo scelto: {}", side_headers[0])).padded(text_margins).styled(important_style));
    doc.push(elements::Paragraph::new(format!("Numero di faults:  {}",analyzer.faults.total_fault)).padded(text_margins).styled(important_style));
    doc.push(elements::Break::new(0.5));
    doc.push(elements::Paragraph::new("Report finale dell'esperimento condotto sulla Fault Injection Pipeline:").padded(text_margins).styled(red));
    doc.push(elements::Break::new(0.5));
    let images_paths = gen_pie_chart(&data_list, &side_headers);
    doc.push(elements::Image::from_path(images_paths[0]).expect("Unable to load image").with_alignment(Alignment::Center));
    let fault_table = gen_table_faults(&data_list,&top_headers,&side_headers);
    doc.push(fault_table);
    doc.push(elements::Break::new(0.1));
    doc.push(elements::Paragraph::new(format!("Tempo esecuzione Fault Injection Pipeline: {} microsec", analyzer.time_experiment)).padded(text_margins));

    doc.render_to_file(file_path)
        .expect("Failed to write output file");

    /*println!("{:?}",analyzer);
    let title_style:Style =  Style::new().bold().with_font_size(20);
    let title_margins= Margins::trbl(0, 0,0,5);
    let text_style = Style::new().with_font_size(10);
    let text_margins = Margins::trbl(0, 65, 0, 5);


    let font_dir = FONT_DIRS
        .iter()
        .filter(|path| std::path::Path::new(path).exists())
        .next()
        .expect("Could not find font directory");

    let default_font =
        fonts::from_files(font_dir, DEFAULT_FONT_NAME, Some(fonts::Builtin::Times))
            .expect("Failed to load the default font family");
    let monospace_font = fonts::from_files(font_dir, MONO_FONT_NAME, Some(fonts::Builtin::Times))
        .expect("Failed to load the monospace font family");

    let mut doc = genpdf::Document::new(default_font);
    doc.set_title("genpdf Demo Document");
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.25);


    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    decorator.set_header(|page| {
        let mut layout = elements::LinearLayout::vertical();
        if page > 1 {
            layout.push(
                elements::Paragraph::new(format!("Page {}", page)).aligned(Alignment::Right),
            );
            layout.push(elements::Break::new(1));
        }
        layout.styled(Style::new().with_font_size(12))
    });
    doc.set_page_decorator(decorator);

    let monospace = doc.add_font_family(monospace_font);
    let code = Style::from(monospace).bold();
    let red = Color::Rgb(255, 0, 0);
    let blue = Color::Rgb(0, 0, 255);

    doc.push(
        elements::Paragraph::new("Risultati Analizzatore")
            .padded(title_margins)
            .styled(title_style)
            .styled(red),
    );
    doc.push(elements::Break::new(1.5));
    doc.push(elements::Paragraph::new(
        "Questo è un documento di prova per mostrare come sarebbe possibile mostrare i dati \
              estratti dall'analizzatore direttamente in un pdf.",
    ).padded(text_margins).styled(text_style));

    doc.push(elements::Break::new(0.5));

    doc.push(elements::Paragraph::new(
        "Grafico a torta contenente il totale di errori iniettati divisi per categoria",
    ).padded(text_margins).styled(text_style));
    // Generazione dei grafici
    //chart_generator::not_rose_radius_pie_chart(analyzer.clone());
    //chart_generator::pie_chart(analyzer);

    let image_path1: &'static str = "src/pdf_generator/images/pie_chart1.png";
    let image_path2: &'static str = "src/pdf_generator/images/pie_chart2.png";

    // Metto una sola immagine
    doc.push(elements::Image::from_path(image_path2).expect("Unable to load image").
        with_alignment(Alignment::Center));

    //Metto due immagini una di fianco all'altra
    let mut table = elements::TableLayout::new(vec![5, 5]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(false, false, false));
    let mut linear_layout = elements::LinearLayout::vertical();

    table
        .row()
        .element(
            elements::PaddedElement::new(
                elements::Image::from_path(image_path1)
                    .expect("Unable to load image")
                    .with_alignment(Alignment::Center), Margins::trbl(0,40,0,0)),
        )
        .element(
            elements::PaddedElement::new(
                elements::Image::from_path(image_path2)
                    .expect("Unable to load image")
                    .with_alignment(Alignment::Center), Margins::trbl(0,0,0,0)),
        )
        .push()
        .expect("Invalid table row");
    doc.push(table.padded(Margins::trbl(5,0,0,20)));



    let mut list = elements::UnorderedList::new();

    list.push(
        elements::Paragraph::default()
            .styled_string("Text", code)
            .string(", a single line of formatted text without wrapping."),
    );
    list.push(
        elements::Paragraph::default()
            .styled_string("Paragraph", code)
            .string(
                ", one or more lines of formatted text with wrapping and an alignment (left, \
                 center, right).",
            ),
    );
    list.push(
        elements::Paragraph::default()
            .styled_string("FramedElement", code)
            .string(", a frame drawn around other elements."),
    );
    list.push(
        elements::Paragraph::default()
            .styled_string("PaddedElement", code)
            .string(", an element with an additional padding."),
    );
    list.push(
        elements::Paragraph::default()
            .styled_string("StyledElement", code)
            .string(", an element with new default style."),
    );

    list.push(
        elements::Paragraph::default()
            .styled_string("UnorderedList", code)
            .string(", an unordered list of bullet points."),
    );

    list.push(
        elements::LinearLayout::vertical()
            .element(
                elements::Paragraph::default()
                    .styled_string("OrderedList", code)
                    .string(", an ordered list of bullet points."),
            )
            .element(
                elements::OrderedList::new()
                    .element(elements::Paragraph::new("Just like this."))
                    .element(elements::Paragraph::new("And this.")),
            ),
    );

    list.push(
        elements::LinearLayout::vertical()
            .element(
                elements::Paragraph::default()
                    .styled_string("BulletPoint", code)
                    .string(", an element with a bullet point, just like in this list."),
            )
            .element(elements::BulletPoint::new(elements::Paragraph::new(
                "Of course, lists can also be nested.",
            )))
            .element(
                elements::BulletPoint::new(elements::Paragraph::new(
                    "And you can change the bullet symbol.",
                ))
                    .with_bullet("•"),
            ),
    );

    list.push(
        elements::Paragraph::default()
            .styled_string("LinearLayout", code)
            .string(
                ", a container that vertically stacks its elements. The root element of a \
                 document is always a LinearLayout.",
            ),
    );
    list.push(
        elements::Paragraph::default()
            .styled_string("TableLayout", code)
            .string(", a container that arranges its elements in rows and columns."),
    );
    list.push(elements::Paragraph::new("And some more utility elements …"));
    doc.push(list);
    doc.push(elements::Break::new(1.5));

    doc.push(elements::Paragraph::new(
        "You already saw lists and formatted centered text. Here are some other examples:",
    ));
    doc.push(elements::Paragraph::new("This is right-aligned text.").aligned(Alignment::Right));
    doc.push(
        elements::Paragraph::new("And this paragraph has a frame drawn around it and is colored.")
            .padded(genpdf::Margins::vh(0, 1))
            .framed()
            .styled(red),
    );
    doc.push(
        elements::Paragraph::new("You can also use other fonts if you want to.").styled(monospace),
    );
    doc.push(
        elements::Paragraph::default()
            .string("You can also ")
            .styled_string("combine ", red)
            .styled_string("multiple ", Style::from(blue).italic())
            .styled_string("formats", code)
            .string(" in one paragraph.")
            .styled(Style::new().with_font_size(16)),
    );
    doc.push(elements::Break::new(1.5));

    doc.push(elements::Paragraph::new(
        "Embedding images also works using the 'images' feature.",
    ));
    println!("Test image");


    doc.push(elements::Paragraph::new("Here is an example table:"));

    let mut table = elements::TableLayout::new(vec![1, 2]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(false, false, false));
    table
        .row()
        .element(
            elements::Paragraph::new("Header 1")
                .styled(Effect::Bold)
                .padded(1),
        )
        .element(elements::Paragraph::new("Value 2").padded(1))
        .push()
        .expect("Invalid table row");
    table
        .row()
        .element(
            elements::Paragraph::new("Header 2")
                .styled(Effect::Bold)
                .padded(1),
        )
        .element(
            elements::Paragraph::new(
                "A long paragraph to demonstrate how wrapping works in tables.  Nice, right?",
            )
                .padded(1),
        )
        .push()
        .expect("Invalid table row");

    let list_layout = elements::LinearLayout::vertical()
        .element(elements::Paragraph::new(
            "Of course, you can use all other elements inside a table.",
        ))
        .element(
            elements::UnorderedList::new()
                .element(elements::Paragraph::new("Even lists!"))
                .element(
                    elements::Paragraph::new("And frames!")
                        .padded(genpdf::Margins::vh(0, 1))
                        .framed(),
                ),
        );
    table
        .row()
        .element(
            elements::Paragraph::new("Header 3")
                .styled(Effect::Bold)
                .padded(1),
        )
        .element(list_layout.padded(1))
        .push()
        .expect("Invalid table row");
    doc.push(table);
    doc.push(elements::Break::new(1.5));

    doc.push(elements::Paragraph::new(
        "Now let’s print a long table to demonstrate how page wrapping works:",
    ));
    doc.render_to_file(file_path)
        .expect("Failed to write output file");
     */
}
pub fn gen_bar_chart(data_list: &Vec<Analyzer>, side_headers:&Vec<&str>)-> &'static str {
    let mut percentages = Vec::new();
    for anl in data_list{
        percentages.push(f64::trunc(((anl.faults.total_fault as f64 - anl.faults.n_silent_fault as f64)/anl.faults.total_fault as f64)*10000.0)/100.0);

    }
    chart_generator::bar_chart(percentages,side_headers);
    "src/pdf_generator/images/percentage_detected.png"
}
pub fn gen_table_dim_time(data_list: &Vec<Analyzer> , top_headers: &Vec<&str>, side_headers: &Vec<&str>)-> TableLayout{
    let mut column_weights = vec![6; top_headers.len()+1];
    column_weights[0] = 10;
    let header_style = Style::new().with_font_size(7).bold();
    let mut table = TableLayout::new(column_weights);
    table.set_cell_decorator(FrameCellDecorator::new(true, true, false));

    let mut row = table.row().element(
        Paragraph::new("")
    );

    //Costruisco l'header
    for header in top_headers{
        row = row.element(
            Paragraph::default()
                .styled_string(*header, header_style)
                .aligned(Alignment::Center)
                .padded( Margins::trbl(0,4.5,0,0)),
        );
    }

    row.push().expect("Invalid table row");
    for i in 0..data_list.len(){
        let anl = &data_list[i];
        let info_vec = vec![  anl.byte_not_hardened,
                                        anl.byte_hardened,
                                        f64::trunc((anl.byte_hardened/anl.byte_not_hardened)*100.0)/100.0,
                                        anl.time_alg_not_hardened,
                                        anl.time_alg_hardened,
                                        f64::trunc((anl.time_alg_hardened/anl.time_alg_not_hardened)*100.0)/100.0
                                    ];
        let mut row = table.row().element(
            Paragraph::new(side_headers[i])
                .styled(header_style)
                .padded(1),
        );

        for info in info_vec{
            println!("{}",info.to_string());
            row = row.element(
                Paragraph::default()
                    .styled_string(&info.to_string(), Style::new().with_font_size(7).italic())
                    .aligned(Alignment::Center)
            );
        }
        row.push().expect("Invalid table row");
    }
    table


}

pub fn gen_pie_chart(data: &Vec<Analyzer>, target: &Vec <&str>)->Vec<&'static str> {
    for i in 0..data.len(){
        let anl = &data[i];
        let mut file_name = "pie_chart".to_string();
        file_name.push_str(&i.to_string());
        file_name.push_str(".png");
        chart_generator::not_rose_pie_chart(&anl.faults, file_name.as_str(),target[i]);
    }
    let mut image_paths = Vec::new();
    if data.len()>1 {
        image_paths.push("src/pdf_generator/images/pie_chart0.png");
        image_paths.push("src/pdf_generator/images/pie_chart1.png");
        image_paths.push("src/pdf_generator/images/pie_chart2.png");
    } else {
        image_paths.push("src/pdf_generator/images/pie_chart0.png");
    }
    image_paths
}

pub fn add_image_to_pdf(images_paths: Vec<&str>,doc: &mut Document){
    doc.push(elements::Image::from_path(images_paths[0]).expect("Unable to load image").
        with_alignment(Alignment::Center));

    //Metto due immagini una di fianco all'altra
    let mut table = TableLayout::new(vec![5, 5]);
    table.set_cell_decorator(FrameCellDecorator::new(false, false, false));
    table
        .row()
        .element(
            elements::PaddedElement::new(
                elements::Image::from_path(images_paths[1])
                    .expect("Unable to load image")
                    .with_alignment(Alignment::Center), Margins::trbl(0,40,0,0)),
        )
        .element(
            elements::PaddedElement::new(
                elements::Image::from_path(images_paths[2])
                    .expect("Unable to load image")
                    .with_alignment(Alignment::Center), Margins::trbl(0,0,0,0)),
        )
        .push()
        .expect("Invalid table row");
    doc.push(table.padded(Margins::trbl(5,0,0,20)));
}

pub fn gen_table_faults(data: &Vec<Analyzer>, top_headers: &Vec<&str>, side_headers: &Vec<&str>)-> TableLayout {
    let mut column_weights = vec![6; top_headers.len()+1];
    column_weights[0] = 10;
    let header_style = Style::new().with_font_size(7).bold();
    let mut table = TableLayout::new(column_weights);
    table.set_cell_decorator(FrameCellDecorator::new(true, true, false));
    //Costruisco l'header
    let mut row = table.row().element(
        Paragraph::new("")
    );

    for header in top_headers{
        row = row.element(
            Paragraph::default()
                .styled_string(*header, header_style)
                .aligned(Alignment::Center)
                .padded( Margins::trbl(0,4.5,0,0)),
        );
    }
    row.push().expect("Invalid table row");

    for i in 0..data.len(){
        let anl = &data[i];
        let f = &anl.faults;
        let mut row = table.row().element(
                    Paragraph::new(side_headers[i])
                    .styled(header_style)
                    .padded(1),
            );

            for info in f.iter(){
                println!("{}",info.1);
                row = row.element(
                    Paragraph::default()
                        .styled_string(&info.1.to_string(), Style::new().with_font_size(7).italic())
                        .aligned(Alignment::Center)
                );
            }
            row.push().expect("Invalid table row");

    }
    table

 }


// Only import the images if the feature is enabled. This helps verify our handling of feature toggles.
mod images {
    use super::*;

    pub fn one_(doc: &mut genpdf::Document) {


        /*
        doc.push(elements::Paragraph::new(
            "and here is one that is centered, rotated, and scaled some.",
        ));

        doc.push(
            elements::Image::from_path(IMAGE_PATH_JPG)
                .expect("Unable to load image")
                .with_alignment(Alignment::Center)
                .with_scale(genpdf::Scale::new(0.5, 2))
                .with_clockwise_rotation(45.0),
        );
        doc.push(elements::Paragraph::new(
            "For a full example of image functionality, please see images.pdf.",
        ));
        doc.push(elements::Break::new(1.5));
         */
   }

}
fn setup_document()->Document{
    let title_style =  Style::new().bold().with_font_size(20);
    let title_margins= Margins::trbl(0, 0,0,5);
    let red = Color::Rgb(255, 0, 0);

    let font_dir = FONT_DIRS
        .iter()
        .filter(|path| std::path::Path::new(path).exists())
        .next()
        .expect("Could not find font directory");

    let default_font =
        fonts::from_files(font_dir, DEFAULT_FONT_NAME, Some(fonts::Builtin::Times))
            .expect("Failed to load the default font family");
    let monospace_font = fonts::from_files(font_dir, MONO_FONT_NAME, Some(fonts::Builtin::Times))
        .expect("Failed to load the monospace font family");

    let mut doc = genpdf::Document::new(default_font);
    doc.set_title("genpdf Demo Document");
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.25);


    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    decorator.set_header(|page| {
        let mut layout = LinearLayout::vertical();
        if page > 1 {
            layout.push(
                elements::Paragraph::new(format!("Page {}", page)).aligned(Alignment::Right),
            );
            layout.push(elements::Break::new(1));
        }
        layout.styled(Style::new().with_font_size(12))
    });
    doc.set_page_decorator(decorator);
    doc.push(
       Paragraph::new("Risultati Analizzatore")
            .padded(title_margins)
            .styled(title_style)
            .styled(red),
    );
    doc

}
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn try_gen_table_faults(){
        let file_path = "results/prova_table.pdf";
        let anl = Analyzer::new(Faults {
            n_silent_fault: 10,
            n_assign_fault: 20,
            n_mul_fault: 30,
            n_generic_fault: 40,
            n_add_fault: 50,
            n_indexmut_fault: 60,
            n_index_fault: 70,
            n_ord_fault: 80,
            n_partialord_fault: 90,
            n_partialeq_fault: 100,
            total_fault: 550,
        }, vec![100.5, 23.9, 3.4 ], vec![322.4,323.9,111.4], 111.0, 1);
        let mut anl2 = anl.clone();
        anl2.n_esecuzione=1;
        anl2.faults.n_add_fault=1;
        let mut anl3 = anl.clone();
        anl3.n_esecuzione = 2;
        anl3.faults.n_generic_fault=2;
        let data = vec![anl,anl2,anl3];
        let top_headers =  vec!["SILENT","ASSIGN","MUL","GENERIC","ADD","IND_MUT","INDEX","ORD","PAR_ORD","PAR_EQ"];
        let side_headers = vec!["SELECTION SORT","BUBBLE SORT","MATRIX MULTIPLICATION"];
        let table = gen_table_faults(&data,&top_headers,&side_headers);
        let mut doc = setup_document();
        doc.push(table);
        doc.render_to_file(file_path)
            .expect("Failed to write output file");

    }
    #[test]
    fn try_gen_dim_time_table(){
        let file_path = "results/prova_table.pdf";
        let top_headers =  vec!["HARDENED(Byte)", "NOT HARDENED(Byte)", "HARD / NOT HARD","HARDENED(nS)","NOT HARDENED(nS)","HARD / NOT HARD"];
        let side_headers = vec!["SELECTION SORT","BUBBLE SORT","MATRIX MULTIPLICATION"];
        let anl = Analyzer::new(Faults {
            n_silent_fault: 10,
            n_assign_fault: 20,
            n_mul_fault: 30,
            n_generic_fault: 40,
            n_add_fault: 50,
            n_indexmut_fault: 60,
            n_index_fault: 70,
            n_ord_fault: 80,
            n_partialord_fault: 90,
            n_partialeq_fault: 100,
            total_fault: 550,
        }, vec![100.5, 23.9, 3.4 ], vec![322.4,323.9,111.4], 111.0, 1);
        let mut anl2 = anl.clone();
        anl2.n_esecuzione=1;
        anl2.faults.n_add_fault=1;
        let mut anl3 = anl.clone();
        anl3.n_esecuzione = 2;
        anl3.faults.n_generic_fault=2;
        let data = vec![anl,anl2,anl3];
        let table = gen_table_dim_time(&data,&top_headers,&side_headers);
        let mut doc = setup_document();
        doc.push(table);
        doc.render_to_file(file_path)
            .expect("Failed to write output file");

    }

    #[test]
    fn try_gen_pie_chart(){
        let mut table = TableLayout::new(vec![1, 5]);
        let image_path1: &'static str = "src/pdf_generator/images/pie_chart1.png";
        let image_path2: &'static str = "src/pdf_generator/images/pie_chart2.png";
        let mut doc = setup_document();
        // Metto una sola immagine
        doc.push(elements::Image::from_path(image_path2).expect("Unable to load image").
            with_alignment(Alignment::Center));

        //Metto due immagini una di fianco all'altra
        let mut table = elements::TableLayout::new(vec![5, 5]);
        table.set_cell_decorator(elements::FrameCellDecorator::new(false, false, false));
        let mut linear_layout = elements::LinearLayout::vertical();

        table
            .row()
            .element(
                elements::PaddedElement::new(
                    elements::Image::from_path(image_path1)
                        .expect("Unable to load image")
                        .with_alignment(Alignment::Center), Margins::trbl(0,40,0,0)),
            )
            .element(
                elements::PaddedElement::new(
                    elements::Image::from_path(image_path2)
                        .expect("Unable to load image")
                        .with_alignment(Alignment::Center), Margins::trbl(0,0,0,0)),
            )
            .push()
            .expect("Invalid table row");
        doc.push(table.padded(Margins::trbl(5,0,0,20)));
        let file_path = "results/prova_pie_chart.pdf";
        doc.render_to_file(file_path)
            .expect("Failed to write output file");
    }
}
