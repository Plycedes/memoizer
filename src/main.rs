fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    component MemoryTile inherits Rectangle {
        callback clicked;
        in property <bool> open_curtain;
        in property <bool> solved;
        in property <image> icon;

        width: 64px;
        height: 64px;
        background: solved ? #0043fd : #74c8f0;
        animate background { duration: 800ms; }

        Image {
            source: icon;
            width: parent.width;
            height: parent.height;
        }

        //Left curtain
        Rectangle {
            background: #3157c9;
            x: 0px;
            width: open_curtain ? 0px : (parent.width / 2);
            height: parent.height;
            animate width {duration: 250ms; easing: ease-in;}
        }

        //Right curtain
        Rectangle {
            background: #3157c9;
            x: open-curtain ? parent.width : (parent.width / 2);
            width: open-curtain ? 0px : (parent.width / 2);
            height: parent.height;
            animate width { duration: 250ms; easing: ease-in; }
            animate x { duration: 250ms; easing: ease-in; }
        }

        TouchArea {
            clicked => {
                root.clicked();
            }
        }
    }
    export component MainWindow inherits Window {
        MemoryTile {
            icon: @image-url("icons/bus.png");
            clicked => {
                self.open_curtain = !self.open_curtain;
            }
        }
    }
}