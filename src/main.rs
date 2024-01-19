fn main() {
    use slint::Model;

    let main_window = MainWindow::new().unwrap();

    let mut tiles: Vec<TileData> = main_window.get_memory_tiles().iter().collect();
    tiles.extend(tiles.clone());

    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);

    let tiles_model = std::rc::Rc::new(slint::VecModel::from(tiles));
    main_window.set_memory_tiles(tiles_model.into());

    main_window.run().unwrap();
}

slint::slint! {
    struct TileData {
        image: image,
        image_visible: bool,
        solved: bool,
    }
    component MemoryTile inherits Rectangle {
        callback clicked;
        in property <bool> open_curtain;
        in property <bool> solved;
        in property <image> icon;

        width: 128px;
        height: 128px;
        background: solved ? #0083fd : #74c8f0;
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
        width: 720px;
        height: 720px;

        in property <[TileData]> memory_tiles: [
            {image: @image-url("icons/bank.png")},
            {image: @image-url("icons/bus.png")},
            {image: @image-url("icons/fish.png")},
            {image: @image-url("icons/love.png")},
            {image: @image-url("icons/gift.png")},
            {image: @image-url("icons/usd.png")},
            {image: @image-url("icons/rocket.png")},
            {image: @image-url("icons/woman.png")},
        ];

        for tile[i] in memory-tiles : MemoryTile {
            x: mod(i, 4) * 136px;
            y: floor(i / 4) * 136px;
            width: 128px;
            height: 128px;
            icon: tile.image;
            open-curtain: tile.image-visible || tile.solved;
            solved: tile.solved;

            clicked => {
                tile.image-visible = !tile.image_visible;
            }
        }
    }
}