pub fn iterator_demo() {
    let v1 = vec![1, 2, 10, 3];

    let itr = v1.iter();

    let list: Vec<_> = itr.map(|x| x+1).collect();

    println!("{:?}", list)
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == size).collect()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn filter_in_size(){
        let shoes = vec![
            Shoe{
                size: 12,
                style: String::from("school")
            },
            Shoe{
                size: 20,
                style: String::from("rainy")
            },
            Shoe{
                size: 12,
                style: String::from("casual")
            },
            Shoe{
                size: 12,
                style: String::from("sneakers")
            }
        ];

        let in_my_size = shoes_in_size(shoes, 12);

        assert_eq!(
            in_my_size,
            vec![
                Shoe{
                    size: 12,
                    style: String::from("school")
                },
                Shoe{
                    size: 12,
                    style: String::from("casual")
                },
                Shoe{
                    size: 12,
                    style: String::from("sneakers")
                }
            ]
        );
    }
}