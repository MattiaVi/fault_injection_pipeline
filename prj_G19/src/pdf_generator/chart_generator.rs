use charts_rs::{BarChart, PieChart};
use crate::analyzer::Faults;
use crate::pdf_generator::encoder::svg_to_png;
pub fn rose_pie_chart(faults: &Faults, file_name: &str,target: &str) {
    let pie_chart_json = format!(
        r###"{{
            "title_text": "Faults",
            "sub_title_text": "Risultato iniezione {} errori su {}",
            "legend_show": false,
            "radius": 130,
            "inner_radius": 30,
            "series_list": [
                {{
                    "name": "Silent",
                    "data": [{}]
                }},
                {{
                    "name": "Assignment",
                    "data": [{}]
                }},
                {{
                    "name": "Multiplication",
                    "data": [{}]
                }},
                {{
                    "name": "Addition",
                    "data": [{}]
                }},
                {{
                    "name": "Ord",
                    "data": [{}]
                }},
                {{
                    "name": "PartialOrd",
                    "data": [{}]
                }},
                {{
                    "name": "PartialEq",
                    "data": [{}]
                }},
                {{
                    "name": "Generic",
                    "data": [{}]
                }},
                {{
                    "name": "Index",
                    "data": [{}]
                }},
                {{
                    "name": "IndexMut",
                    "data": [{}]
                }}
            ]
        }}"###,
         faults.total_fault,
         target,
         faults.n_silent_fault,
         faults.n_assign_fault,
         faults.n_mul_fault,
         faults.n_add_fault,
         faults.n_ord_fault,
         faults.n_partialord_fault,
         faults.n_partialeq_fault,
         faults.n_generic_fault,
         faults.n_index_fault,
         faults.n_indexmut_fault,
    );
    let pie_chart = PieChart::from_json(&pie_chart_json).unwrap();
    let res = pie_chart.svg().unwrap();
    let dest_path = "src/pdf_generator/images/";
    svg_to_png(&res, dest_path, file_name).expect("Impossibile convertire SVG in PNG");
}

pub fn not_rose_pie_chart(faults: &Faults, file_name: &str,target: &str) {
    let pie_chart_json = format!(
        r###"{{
            "title_text": "Faults",
            "sub_title_text": "Risultato iniezione {} errori su {}",
            "legend_show": false,
            "radius": 130,
            "inner_radius": 30,
            "rose_type": false,
            "series_list": [
                {{
                    "name": "Silent",
                    "data": [{}]
                }},
                {{
                    "name": "Assignment",
                    "data": [{}]
                }},
                {{
                    "name": "Multiplication",
                    "data": [{}]
                }},
                {{
                    "name": "Addition",
                    "data": [{}]
                }},
                {{
                    "name": "Ord",
                    "data": [{}]
                }},
                {{
                    "name": "PartialOrd",
                    "data": [{}]
                }},
                {{
                    "name": "PartialEq",
                    "data": [{}]
                }},
                {{
                    "name": "Generic",
                    "data": [{}]
                }},
                {{
                    "name": "Index",
                    "data": [{}]
                }},
                {{
                    "name": "IndexMut",
                    "data": [{}]
                }}
            ]
        }}"###,
        faults.total_fault,
        target,
        faults.n_silent_fault,
        faults.n_assign_fault,
        faults.n_mul_fault,
        faults.n_add_fault,
        faults.n_ord_fault,
        faults.n_partialord_fault,
        faults.n_partialeq_fault,
        faults.n_generic_fault,
        faults.n_index_fault,
        faults.n_indexmut_fault,
    );
    let pie_chart = PieChart::from_json(&pie_chart_json).unwrap();
    let res = pie_chart.svg().unwrap();
    let dest_path = "src/pdf_generator/images/";
    svg_to_png(&res, dest_path, file_name).expect("Impossibile convertire SVG in PNG");
}
pub fn bar_chart(data: Vec<f64>, x_axis_data: &Vec<&str>){
    let bar_chart_json = format!(r###"{{
            "width": 630,
            "height": 410,
            "margin": {{
                "left": 10,
                "top": 5,
                "right": 10
            }},
            "title_text": "DETECTED",
            "title_font_color": "#345",
            "title_align": "center",
            "sub_title_text": "Percentuale di fault detected rispetto al totale, confronto tra diverse esecuzioni",
            "sub_title_align": "center",
            "sub_title_font_weight": "bold",
            "y_axis_configs": [
                {{
                    "axis_font_weight": "bold"
                }}
            ],
            "series_label_font_weight": "bold",
            "series_list": [
                {{
                    "label_show": true,
                    "data": [{}, {}, {}]
                }}
            ],
            "x_axis_data": [
                "{}",
                "{}",
                "{}"
            ],
            "x_axis_margin": {{
                "left": 1,
                "top": 0,
                "right": 0,
                "bottom": 0
            }},
            "x_axis_font_weight": "bold"
        }}"###, data[0], data[1], data[2], x_axis_data[0], x_axis_data[1], x_axis_data[2]);

    let bar_chart = BarChart::from_json(&bar_chart_json).unwrap();
    let res = bar_chart.svg().unwrap();
    let dest_path = "src/pdf_generator/images/";
    let file_name = "percentage_detected.png";
    svg_to_png(&res, dest_path, file_name).expect("Impossibile convertire SVG in PNG");
}

#[cfg(test)]
mod tests {
    use charts_rs::{BarChart, PieChart};
    use crate::pdf_generator::encoder::svg_to_png;

    #[test]
    fn bar_chart() {
        let bar_chart_json = format!(r###"{{
            "width": 630,
            "height": 410,
            "margin": {{
                "left": 10,
                "top": 5,
                "right": 10
            }},
            "title_text": "DETECTED",
            "title_font_color": "#345",
            "title_align": "right",
            "sub_title_text": "Percentuale di fault detected rispetto al totale, confronto tra diverse esecuzioni",
            "sub_title_align": "right",
            "sub_title_font_weight": "bold",
            "legend_align": "left",
            "legend_font_weight": "bold",
            "y_axis_configs": [
                {{
                    "axis_font_weight": "bold"
                }}
            ],
            "series_label_font_weight": "bold",
            "series_list": [
                {{
                    "name": "Email",
                    "label_show": true,
                    "data": [{}, {}, {}]
                }}
            ],
            "x_axis_data": [
                "{}",
                "{}",
                "{}"
            ],
            "x_axis_margin": {{
                "left": 1,
                "top": 0,
                "right": 0,
                "bottom": 0
            }},
            "x_axis_font_weight": "bold"
        }}"###,
                                         3,
                                         3,
                                         2,
                "SELECTION","BUBBLE","MATRIX"
        );
        println!("{}", bar_chart_json);
        let bar_chart = BarChart::from_json(&bar_chart_json).unwrap();
        println!("{:#?}", bar_chart);
        let res = bar_chart.svg().unwrap();
        let dest_path = "src/pdf_generator/images/";
        let file_name = "test_bar_chart.png";
        svg_to_png(&res, dest_path, file_name).expect("Impossibile convertire SVG in PNG");
    }
}