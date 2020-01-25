use super::*;

/// perform a submatch against the OVpinDialog msg
pub(crate) fn match_packages_tree(
    msg: OPackagesTree,
    db: &mut packybara::db::packrat::PackratDb,
    conductor: &mut qt_thread_conductor::conductor::Conductor<Event>,
    sender: &Sender<IMsg>,
) {
    match msg {
        OChooseAltDist::GetDistributions { package } => {
            let distributions = match db.find_all_distributions().package(package).query() {
                Ok(distributions) => distributions,
                Err(err) => {
                    sender
                        .send(IMsg::Error(format!(
                            "Unable to get distributions from db: {}",
                            err
                        )))
                        .expect("unable to send error msg");
                    conductor.signal(Event::Error);
                    return;
                }
            };
            let distributions = distributions
                .into_iter()
                .map(|mut x| std::mem::replace(&mut x.name, String::new()))
                .collect::<Vec<_>>();
            sender
                .send(IChooseAltDist::Distributions(distributions).to_imsg())
                .expect("unable to send distributions");
            conductor.signal(ChooseAltDist::GetDistributions.to_event());
        }
    }
}
