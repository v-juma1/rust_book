//mod 名称 后面若不是{},而是分号；则rust会从src下与mod同名的文件中引入所需内容
//mod rectangle_demo;
mod struct_demo;

mod word_counter;
fn main() {
    struct_demo::run_demo();
    //rectangle_demo::run_demo();
    //word_counter::run();
}
