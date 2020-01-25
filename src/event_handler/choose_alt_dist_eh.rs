use super::*;
use crate::{event::choose_alt_dist::ChooseAltDist, IChooseAltDist};
use pbgui_tree::tree;
use qt_widgets::{cpp_core::MutPtr, QTableWidget, QWiget};
use std::cell::RefCell;
use std::rc::Rc;

pub fn match_choose_alt_dist<'a>(
    event: ChooseAltDist,
    r: i32,
    vpin_tablewidget_ptr: MutPtr<QTableWidget>,
    root_widget_ptr: MutPtr<QWidget>,
    mut pinchanges_ptr: MutPtr<QTableWidget>,
    pinchange_cache: Rc<PinChangesCache>,
    receiver: &Receiver<IMsg>,
) {
    match event {
        ChooseAltDist::GetDistributions => {
            if let Ok(IMsg::ChooseAltDist(IChooseAltDist::Distributions(distributions))) =
                receiver.recv()
            {
                let dists_ref = distributions.iter().map(|x| x.as_str()).collect::<Vec<_>>();

                let (versions_list, idx, dist_versions) =
                    build_qstring_list_and_map(version, results);
                let mut ok_or_cancel = false;
                let ok_or_cancel_ptr = MutPtr::from_raw(&mut ok_or_cancel);
                // Get New version by popping up a Dialog
                let new_version = QInputDialog::get_item_7a(
                    root_widget_ptr,
                    &qs("Pick Version"),
                    &qs(package),
                    &versions_list,
                    idx,
                    false,
                    ok_or_cancel_ptr,
                );
                if ok_or_cancel_ptr.is_null() {
                    log::error!("ok_or_cancel_ptr is null. Problem on QT side. Returning");
                    return;
                }
                if *ok_or_cancel_ptr == false {
                    log::info!("cancelled");
                } else {
                    let new_version_string = new_version.to_std_string();
                    let new_dist_id = match dist_versions.get(new_version_string.as_str()) {
                        Some(id) => id,
                        // TODO: handle this more appropriately
                        None => {
                            log::error!("ERROR: Unable to get dist id.");
                            return;
                        }
                    };
                    let new_distribution = format!("{}-{}", package, new_version_string);
                    if orig_vpin_table_distribution == new_distribution {
                        log::info!("new value and old value match. Skipping");
                        return;
                    }
                    // retrieve the value of the versionpin row
                    let vpin_row = VersionPinRow::<CppBox<QString>>::from_table_at_row(
                        &vpin_tablewidget_ptr,
                        r,
                    )
                    .unwrap();
                    // cache the change. we will use this later to update the db. The rest of
                    // the code is for updating the ui
                    let new_value_qstr = QString::from_std_str(new_distribution);
                    // build up new string
                    distribution.set_text(&new_value_qstr);
                    if pinchange_cache.has_key(vpin_row.pkgcoord_id) {
                        let row = match pinchange_cache.index(vpin_row.pkgcoord_id) {
                            Some(r) => r,
                            None => {
                                log::error!("ERROR: Problem retrieving row from QT");
                                return;
                            }
                        };
                        let mut item = pinchanges_ptr.item(row, COL_PC_NEW_VALUE);
                        if item.is_null() {
                            log::error!("problem retreiving row from pinchanges_ptr using cached row number. item is null");
                            return;
                        }
                        item.set_text(&new_version);
                        let change = Change::ChangeDistribution {
                            vpin_id: vpin_row.id,
                            new_dist_id: *new_dist_id,
                        };
                        pinchange_cache.cache_change_at(change, row);
                    } else {
                        let vpc_row = VersionPinChangesRow::<CppBox<QString>>::new(
                            ChangeType::ChangeDistribution,
                            vpin_row.pkgcoord(),
                            qs(version),
                            new_version,
                        );
                        pinchange_cache.cache_original_version(vpin_row.id, version);
                        let row_cnt = pinchanges_ptr.row_count() + 1;
                        pinchanges_ptr.set_row_count(row_cnt);

                        vpc_row.set_table_row(&mut pinchanges_ptr, row_cnt - 1);
                        let update_color = qcolor_blue!();
                        distribution.set_foreground(&QBrush::from_q_color(update_color.as_ref()));
                        distribution.table_widget().clear_selection();
                        let idx = pinchange_cache.row_count();
                        pinchange_cache.cache_dist(vpin_row.pkgcoord_id, idx);
                        let change = Change::ChangeDistribution {
                            vpin_id: vpin_row.id,
                            new_dist_id: *new_dist_id,
                        };
                        pinchange_cache.cache_change(change);
                    }
                }
            } else {
                log::error!("PackagesTree::GetPackages IMsg does not match event state");
            }
        }
    }
}

// Construct a qstringlist of versions, identify the index of the currently selected version,
// and provide a hasmap mapping the version to the id
fn build_qstring_list_and_map(
    version: &str,
    results: Vec<FindAllDistributionsRow>,
) -> (CppBox<QStringList>, i32, HashMap<String, IdType>) {
    unsafe {
        let mut versions_list = QStringList::new();
        let mut idx = 0;
        let mut cnt = 0;
        let mut dist_versions = HashMap::new();
        for r in results {
            if r.version == version {
                idx = cnt;
            }
            cnt += 1;
            dist_versions.insert(r.version.clone(), r.id);
            versions_list.append_q_string(&QString::from_std_str(r.version));
        }
        (versions_list, idx, dist_versions)
    }
}
