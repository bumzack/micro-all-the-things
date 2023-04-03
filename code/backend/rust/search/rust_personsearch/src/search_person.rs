pub mod filters_search_person {
    use std::convert::Infallible;
    use std::future::Future;

    use warp::{Filter, Reply};

    use common::meili_filter::handlers_search_entity::meili_filter_person;
    use common::meili_search::handlers_search_entity::meili_search_person;
    use common::search::SearchPersonList;

    use crate::CLIENT;

    pub fn search_person_route(
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        let server1 = warp::path!("api" / "person" / "name" / String);
        let search_name = server1.and(warp::get()).and_then(|name: String| {
            //  println!("GET /api/person/name/:name matched");
            meili_search_person("person".to_string(), name, &CLIENT)
        });

        let server2 = warp::path!("api" / "person" / "nconst" / String);
        let search_nconst = server2.and(warp::get()).and_then(|name: String| {
            // println!("GET /api/person/nconst/:nconst matched");
            filter_entity(name)
        });

        let server3 = warp::path!("api" / "person" / "nconst");
        let search_nconsts = server3
            .and(warp::post())
            .and(json_body_search_person_list())
            .and_then(|req: SearchPersonList| {
                //  println!("POST  /api/person/nconst  matched");
                filter_entity_with_joined_or(req)
            });

        search_name.or(search_nconsts)
        //     .or(search_nconst)
    }

    fn json_body_search_person_list(
    ) -> impl Filter<Extract = (SearchPersonList,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 1000 * 1000).and(warp::body::json())
    }

    fn filter_entity(name: String) -> impl Future<Output = Result<impl Reply + Sized, Infallible>> {
        //  println!("filter_entity   {name}");
        let f = format!("\"{}\"  = \"{}\"", "nconst", name);
        let filter: Vec<String> = vec![f];
        meili_filter_person("person".to_string(), filter, &CLIENT)
    }

    fn filter_entity_with_joined_or(
        req: SearchPersonList,
    ) -> impl Future<Output = Result<impl Reply + Sized, Infallible>> {
        //  println!("filter_entity for a list of person nconsts  {:?}", &req);

        let nconsts = req
            .nconsts
            .iter()
            .map(|nconst| format!("nconst = {}", nconst))
            .collect::<Vec<String>>();
        let nconsts = nconsts.join(" OR ");
        //  println!("filter person request: {}", &nconsts);

        let filter: Vec<String> = vec![nconsts];
        meili_filter_person("person".to_string(), filter, &CLIENT)
    }
}
