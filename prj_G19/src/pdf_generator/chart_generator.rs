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
        anl.total_fault,
        anl.n_silent_fail,
        anl.n_assign_fail,
        anl.n_mul_fail,
        anl.n_add_fail,
        anl.n_ord_fail,
        anl.n_partialord_fail,
        anl.n_partialeq_fail,
        anl.n_generic_fail,
        anl.n_index_fail,
        anl.n_indexmut_fail,
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
        anl.total_fault,
        anl.n_silent_fail,
        anl.n_assign_fail,
        anl.n_mul_fail,
        anl.n_add_fail,
        anl.n_ord_fail,
        anl.n_partialord_fail,
        anl.n_partialeq_fail,
        anl.n_generic_fail,
        anl.n_index_fail,
        anl.n_indexmut_fail,
    );
    let pie_chart = PieChart::from_json(&pie_chart_json).unwrap();
    let res = pie_chart.svg().unwrap();
    let dest_path = "src/pdf_generator/images/";
    let file_name = "pie_chart2.png";
    svg_to_png(&res, dest_path, file_name).expect("Impossibile convertire SVG in PNG");
}