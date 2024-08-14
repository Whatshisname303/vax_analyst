use crate::reader::GeneralData;

pub fn get_scen_search_results(general_data: &GeneralData, query: &str) -> Vec<String> {
    general_data.scenarios
        .keys()
        .filter(|key| key.to_lowercase().contains(query.to_lowercase().as_str()))
        .map(|key| key.clone())
        .take(25)
        .collect::<Vec<String>>()
}
// pub fn get_scen_search_results<'a>(general_data: &'a GeneralData, query: &'a str) -> impl Iterator<Item = &'a String> {
//     general_data.scenarios.keys().filter(move |key| key.contains(query)).take(5)
// }
