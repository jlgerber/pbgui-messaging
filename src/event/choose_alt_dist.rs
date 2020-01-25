use super::*;

#[derive(Debug, PartialEq)]
pub enum ChooseAltDist {
    GetPackages,
}

impl ToEvent for ChooseAltDist {
    fn to_event(self) -> Event {
        Event::ChooseAltDist(self)
    }
}

impl ToQString for ChooseAltDist {
    fn to_qstring(&self) -> CppBox<QString> {
        match &self {
            &ChooseAltDist::GetDistributions => {
                QString::from_std_str("ChooseAltDist::GetDistributions")
            }
        }
    }
}

impl FromQString for ChooseAltDist {
    fn from_qstring(qs: Ref<QString>) -> Self {
        match qs.to_std_string().as_str() {
            "ChooseAltDist::GetDistributions" => ChooseAltDist::GetDistributions,
            _ => panic!("In ChooseAltDist event, unable to convert to Event"),
        }
    }
}
