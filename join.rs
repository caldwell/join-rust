// Copyright © 2015 David Caldwell <david@porkrind.org>

// Why isn't this in std::iterator???

pub trait Join {
    fn join(&mut self, sep: &str) -> String;
}
impl<T,U> Join for T where T: Iterator<Item=U>, U: ToString {
    fn join(&mut self, sep: &str) -> String {
        self.fold(String::new(), |mut acc,arg| {
            if acc.is_empty() {
                acc.push_str(&arg.to_string())
            } else {
                acc.push_str(sep);
                acc.push_str(&arg.to_string())
            };
            acc
        })
    }
}

#[test]
fn join_strings() {
    assert_eq!(vec!["hi", "there", "buddy"].iter().join(" "), "hi there buddy");
    assert_eq!(vec!["hi", "there", "buddy"].iter().join("<=>"), "hi<=>there<=>buddy");
}
#[test]
fn join_numbers() {
    assert_eq!(vec![1,2,3,4].iter().join(" "), "1 2 3 4");
    assert_eq!(vec![4,3,2,1].iter().join("<=>"), "4<=>3<=>2<=>1");
}
