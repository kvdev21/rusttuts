use crate::Utils;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryMetadata {
    pub starttime: String,
    pub endtime: String,
    pub opendays: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Catalog {
    pub library: LibraryMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Library {
    pub books: Vec<String>,
    pub metadata: LibraryMetadata,
}

impl Library {
    pub fn get_rand_book(&self, book_list: &mut Vec<String>) -> String {
        let mut rng = rand::thread_rng();
        let books = self.books.clone();
        let book_id = &books[rng.gen_range(0..books.len())];
        book_list.push(book_id.to_owned());
        book_id.to_owned()
    }

    pub fn get_non_repeated_book(&self, book_list: &mut Vec<String>) -> String {
        let mut rng = rand::thread_rng();

        if book_list.len() <= REPEAT_AFTER {
            let to_remove = book_list.clone();
            let unique_books = Utils::u_items(self.books.clone(), to_remove);
            let book_id = &unique_books[rng.gen_range(0..unique_books.len())];
            book_list.push(book_id.to_owned());
            book_id.to_owned()
        } else {
            let offset = (book_list.len() / REPEAT_AFTER) * REPEAT_AFTER;
            let to_remove = Vec::from_iter(book_list[offset..].iter().cloned());
            let unique_books = Utils::u_items(self.books.clone(), to_remove);
            let book_id = &unique_books[rng.gen_range(0..unique_books.len())];
            book_list.push(book_id.to_owned());
            book_id.to_owned()
        }
    }
}

/*
  Get only generic libraries from file system
  and store as a vector of Library struct
*/
pub fn get_generic_libraries(dir: &str) -> Vec<Library> {
    let mut lib_list = Vec::new();

    /*
      Incase `dir` contains files other than libraries
      This was implemented for earlier version.
    */
    let pattern = Regex::new(r".xml$").unwrap();

    let libraries = Utils::read_file_contents(dir, pattern);

    for library in libraries {
        let item: Catalog = serde_xml_rs::from_str(library.as_str()).unwrap();
        let book_ids = Utils::extract_ids(&library);

        lib_list.push(Library {
            books: book_ids,
            metadata: item.library,
        })
    }

    lib_list
}

/*
  Get available libraries based on user read time
  and other business logic
*/
pub fn get_available_libraries(libraries: &Vec<Library>) -> Option<Vec<Library>> {
    let mut catalogs = Vec::new();
    let current_hour = offset::Local::now().hour();
    let today = offset::Local::now().weekday().num_days_from_sunday() + 1;
    println!("{} today!", today);
    // For manual query
    // let today = 5;
    // let current_hour = 10;

    for library in libraries {
        if library.metadata.opendays == 0 || library.metadata.opendays == today {
            // Converts time HH:MM into minutes
            let start_at = time_in_minutes(library.metadata.starttime.as_str());
            let end_at = time_in_minutes(library.metadata.endtime.as_str());
            let time_span = end_at - start_at;

            if time_span != 0 && current_hour * 60 >= start_at && current_hour * 60 < end_at {
                catalogs.push(library.clone());
            }
        }
    }

    if catalogs.len() == 0 {
        None
    } else {
        Some(catalogs)
    }
}

pub fn time_in_minutes(time: &str) -> u32 {
    let hours = time[..2].parse::<u32>().unwrap();
    let minutes = time[2..].parse::<u32>().unwrap();
    hours * 60 + minutes
}
