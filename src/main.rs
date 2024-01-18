fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    component MemoryTile inherits Rectangle {
        width: 200px;
        height: 200px;
        background: @linear-gradient(90deg, #fc0303, #820db8);

        Image {
            source: @image-url("./icons/bus.png");
            width: parent.width;
            height: parent.height;
        }
    }
    export component MainWindow inherits Window {
        MemoryTile {}
    }
}