use crate::tuts::traits::{Tweet, Summary};

pub fn generic_demo(){
    let arr = vec![1, 20, 54, 10, 3, 4];

    let crr = vec!['a', 'z', 'b', 'd'];
    let maxi = get_maximum(&arr);

    let max_char = get_maximum(&crr); 
    println!("Largest value is {}", maxi);
    println!("Larget char is {}", max_char);

    let tweet = Tweet{
        username: String::from("Avdhoot"),
        content: String::from("Hello guys!!"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet ==> {:?}", tweet.summarise());
}


fn get_maximum<T: std::cmp::PartialOrd>(crr: &[T]) -> &T{
    let mut maxi = &crr[0];

    for a in crr{
        if a > maxi{
            maxi = a;
        }
    }

    maxi
}