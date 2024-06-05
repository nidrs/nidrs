use crate::Service;

pub trait ControllerService: Service {}

pub fn template_format<T: IntoIterator<Item = (&'static str, &'static str)>>(path: &str, map: T) -> String {
    let mut path = path.to_string();
    for (k, v) in map {
        path = path.replace(&format!("{{{}}}", k), v);
    }
    path
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_format() {
        let prefix = "/api/{version}";
        let path = prefix.to_string() + "/user";
        let result = template_format(&path, [("version", "v1")]);
        assert_eq!(result, "/api/v1/user");
    }
}
