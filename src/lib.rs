#[derive(Debug)]
pub enum Event {
    SetCounter(u64)
}

pub fn send_to_flutter(event: Event) {
    println!("{:?}", event);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
