mod front_of_house{
    pub mod hosting{
        pub fn add_to_wait_list(){}
    }
}

use front_of_house::hosting;

fn eat_at_restaurant() {
    hosting::add_to_wait_list();
    front_of_house::hosting::add_to_wait_list();
}