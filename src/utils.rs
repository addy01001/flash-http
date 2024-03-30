pub fn curl_parser(input: &String)-> String {
    // let mut temp_input = input.clone().replace("\\\n", "\\");
    let vect_input: Vec<&str> = input.split("\\").collect();
    let ret_url = vect_input[0];
    let mut method = String::from("GET");

    let url: Vec<&str> = ret_url.split("'").collect();
    if url[0].contains("--request") {

    } else {

    }

    if url.len() < 2 {
        return String::new();  
    }

    return url[1].to_string();
}