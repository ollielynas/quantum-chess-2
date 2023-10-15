
const boxes = Array.from(document.getElementsByClassName("square"));

const style2 = document.getElementById("style2");

let hover_val = document.getElementById("hover_value");
let update = document.getElementById("update");

function add_events() {
    boxes.forEach((box) => {
        box.addEventListener("click", function handleClick(event) {
        window.click_pos = this.id;
        update.click();
    });
    box.addEventListener("mouseover", function handleClick(event) {
        a = this.id.split(",");
        style2.innerHTML =
        ".move-" + a[0] + "-" + a[1] + "{background-color: rgba(255, 255, 100, 0.9)}";
    });
    });
};
add_events();
