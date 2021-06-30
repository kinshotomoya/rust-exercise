// こうすることで、moduleの中身をmoduleと同じ名前をしたファイルから読み込む宣言をしている
mod front_of_house;

// front_of_houseのhostingを利用するので、use
use crate::front_of_house::hosting;

fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
