use scraper;
use tl;

// this will be used soon...
// pub(crate) async fn get_elements_by_selector(response: &str, selector: &str) -> Vec<String> {
//     let document = scraper::Html::parse_document(response);

//     let selector_p = scraper::Selector::parse(selector).unwrap();

//     let elements = document.select(&selector_p).map(|x| x.inner_html());

//     return elements.collect();
// }

pub(crate) async fn get_element_by_selector(response: &str, selector: &str) -> String {
    let document = scraper::Html::parse_document(response);

    let selector_p = scraper::Selector::parse(selector).unwrap();

    let elements: Vec<String> = document
        .select(&selector_p)
        .map(|x| x.inner_html())
        .collect();

    assert_eq!(elements.len(), 1, "There should only be one one element for this method {:#?}. To get multiple elemets, use get_elements_by_selector()", elements);

    let element = elements.get(0).map(String::as_str).unwrap().to_string();

    return element;
}

pub(crate) async fn get_element_text_by_id(response: &str, id: &str) -> String {
    let dom = tl::parse(response, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    let element = dom
        .get_element_by_id(id)
        .expect("Failed to find element")
        .get(parser)
        .unwrap();

    return element.inner_text(parser).to_string();
}

pub(crate) async fn get_element_classes_by_id(
    response: &str,
    id: &str,
    selector: &str,
) -> Vec<String> {
    let dom = tl::parse(response, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    let element = dom
        .get_element_by_id(id)
        .expect("Failed to find element")
        .get(parser)
        .unwrap();

    let element_html = scraper::Html::parse_document(&element.outer_html(parser).to_string());
    let doc_selector = scraper::Selector::parse(selector).unwrap();
    let classes_v: Vec<String> = element_html
        .select(&doc_selector)
        .map(|x| x.value().attr("class").unwrap().to_string())
        .collect();

    assert_eq!(
        classes_v.len(),
        1,
        "element selector {:#?} is supposed to return only 1 string. Check the selector <{}>",
        classes_v,
        selector
    );

    let classes: Vec<String> = classes_v
        .get(0)
        .unwrap()
        .split(' ')
        .map(|x| x.to_string())
        .collect();

    return classes;
}
