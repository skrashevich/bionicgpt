use crate::app_layout::{Layout, SideBar};
use assets::files::button_plus_svg;
use assets::files::*;
use db::queries::datasets::Dataset;
use dioxus::prelude::*;
use primer_rsx::*;

struct Props {
    organisation_id: i32,
    datasets: Vec<Dataset>,
}

pub fn index(organisation_id: i32, datasets: Vec<Dataset>) -> String {
    fn app(cx: Scope<Props>) -> Element {
        cx.render(rsx! {
            Layout {
                section_class: "normal",
                selected_item: SideBar::Datasets,
                team_id: cx.props.organisation_id,
                title: "Datasets",
                header: cx.render(rsx!(
                    h3 { "Datasets" }
                    Button {
                        prefix_image_src: "{button_plus_svg.name}",
                        drawer_trigger: "new-dataset-form",
                        button_scheme: ButtonScheme::Primary,
                        "Add Dataset"
                    }
                )),

                if cx.props.datasets.is_empty() {
                    cx.render(rsx! {
                        BlankSlate {
                            heading: "Looks like you don't have any datasets yet",
                            visual: nav_ccsds_data_svg.name,
                            description: "Datasets allow you to organize your documents like folders",
                            //primary_action: ("New Prompt Template", &cx.props.new_route)
                        }
                    })
                } else {
                    cx.render(rsx! {
                        Box {
                            class: "has-data-table",
                            BoxHeader {
                                title: "Datasets"
                            }
                            BoxBody {
                                DataTable {
                                    table {
                                        thead {
                                            th { "Name" }
                                            th { "Document Count" }
                                            th {
                                                class: "text-right",
                                                "Action"
                                            }
                                        }
                                        tbody {

                                            cx.props.datasets.iter().map(|dataset| {
                                                cx.render(rsx!(
                                                    tr {
                                                        td {
                                                            a {
                                                                href: "{crate::routes::documents::index_route(cx.props.organisation_id, dataset.id)}",
                                                                "{dataset.name}" 
                                                            }
                                                        }
                                                        td { "{dataset.count}" }
                                                        td {
                                                            class: "text-right",
                                                            DropDown {
                                                                direction: Direction::West,
                                                                button_text: "...",
                                                                DropDownLink {
                                                                    href: "{crate::routes::documents::index_route(cx.props.organisation_id, dataset.id)}",
                                                                    target: "_top",
                                                                    "View"
                                                                }
                                                            }
                                                        }
                                                    }
                                                ))
                                            })
                                        }
                                    }
                                }
                            }
                        }
                    })
                }


                // The form to create an invitation
                super::new::New {
                    organisation_id: cx.props.organisation_id
                }
            }
        })
    }

    crate::render(VirtualDom::new_with_props(
        app,
        Props {
            organisation_id,
            datasets,
        },
    ))
}
