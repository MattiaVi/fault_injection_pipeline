use charts_rs::PieChart;
use crate::analyzer::Analyzer;
use crate::pdf_generator::encoder::svg_to_png;
pub fn pie_chart(anl: Analyzer) {
    let pie_chart_json = format!(
        r###"{{
            "title_text": "Faults",
            "sub_title_text": "Risultato iniezione {} errori su sistema irrobustito",
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
        anl.faults.total_fault,
        anl.faults.n_silent_fault,
        anl.faults.n_assign_fault,
        anl.faults.n_mul_fault,
        anl.faults.n_add_fault,
        anl.faults.n_ord_fault,
        anl.faults.n_partialord_fault,
        anl.faults.n_partialeq_fault,
        anl.faults.n_generic_fault,
        anl.faults.n_index_fault,
        anl.faults.n_indexmut_fault,
    );
    let pie_chart = PieChart::from_json(&pie_chart_json).unwrap();
    let res = pie_chart.svg().unwrap();
    let dest_path = "src/pdf_generator/images/";
    let file_name = "pie_chart1.png";
    svg_to_png(&res, dest_path, file_name).expect("Impossibile convertire SVG in PNG");
}

pub fn not_rose_radius_pie_chart(anl: Analyzer) {
    let pie_chart_json = format!(
        r###"{{
            "title_text": "Faults",
            "sub_title_text": "Risultato iniezione {} errori su sistema irrobustito",
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
        anl.faults.total_fault,
        anl.faults.n_silent_fault,
        anl.faults.n_assign_fault,
        anl.faults.n_mul_fault,
        anl.faults.n_add_fault,
        anl.faults.n_ord_fault,
        anl.faults.n_partialord_fault,
        anl.faults.n_partialeq_fault,
        anl.faults.n_generic_fault,
        anl.faults.n_index_fault,
        anl.faults.n_indexmut_fault,
    );
    let pie_chart = PieChart::from_json(&pie_chart_json).unwrap();
    let res = pie_chart.svg().unwrap();
    let dest_path = "src/pdf_generator/images/";
    let file_name = "pie_chart2.png";
    svg_to_png(&res, dest_path, file_name).expect("Impossibile convertire SVG in PNG");
}