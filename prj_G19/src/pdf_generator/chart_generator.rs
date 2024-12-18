use charts_rs::PieChart;
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