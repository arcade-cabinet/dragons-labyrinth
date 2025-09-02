Cob(
    Scene(
        children: [
            // Main menu background
            Node(
                styles: Some(Style(
                    position_type: Absolute,
                    left: Px(0.0),
                    top: Px(0.0),
                    width: Percent(100.0),
                    height: Percent(100.0),
                )),
                background_color: Some(Srgba(red: 0.03, green: 0.03, blue: 0.03, alpha: 1.0)),
                children: [
                    // Title
                    Node(
                        styles: Some(Style(
                            position_type: Absolute,
                            left: Percent(50.0),
                            top: Percent(15.0),
                            width: Auto,
                            height: Auto,
                            margin: UiRect(
                                left: Percent(-25.0),
                                top: Px(0.0),
                                right: Px(0.0),
                                bottom: Px(0.0),
                            ),
                        )),
                        children: [
                            Text((
                                text: "DRAGON'S LABYRINTH",
                                text_style: TextStyle(
                                    font: "https://fonts.googleapis.com/css2?family=Creepster&display=swap",
                                    font_size: 48.0,
                                    color: Srgba(red: 0.9, green: 0.1, blue: 0.1, alpha: 1.0),
                                ),
                            )),
                        ],
                    ),
                    
                    // Menu buttons container
                    Node(
                        styles: Some(Style(
                            position_type: Absolute,
                            left: Percent(50.0),
                            top: Percent(45.0),
                            width: Px(300.0),
                            height: Px(400.0),
                            margin: UiRect(
                                left: Px(-150.0),
                                top: Px(0.0),
                                right: Px(0.0),
                                bottom: Px(0.0),
                            ),
                            flex_direction: Column,
                            row_gap: Px(20.0),
                            align_items: Center,
                        )),
                        children: [
                            // New Game Button
                            Button((
                                styles: Some(Style(
                                    width: Px(250.0),
                                    height: Px(60.0),
                                    justify_content: Center,
                                    align_items: Center,
                                    border: UiRect::all(Px(2.0)),
                                )),
                                background_color: Some(Srgba(red: 0.2, green: 0.05, blue: 0.05, alpha: 0.9)),
                                border_color: Some(Srgba(red: 0.6, green: 0.1, blue: 0.1, alpha: 1.0)),
                                children: [
                                    Text((
                                        text: "Begin Journey",
                                        text_style: TextStyle(
                                            font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                            font_size: 16.0,
                                            color: Srgba(red: 0.9, green: 0.9, blue: 0.9, alpha: 1.0),
                                        ),
                                    )),
                                ],
                                marker: NewGameButton,
                            )),
                            
                            // Continue Button
                            Button((
                                styles: Some(Style(
                                    width: Px(250.0),
                                    height: Px(60.0),
                                    justify_content: Center,
                                    align_items: Center,
                                    border: UiRect::all(Px(2.0)),
                                )),
                                background_color: Some(Srgba(red: 0.15, green: 0.15, blue: 0.05, alpha: 0.9)),
                                border_color: Some(Srgba(red: 0.4, green: 0.4, blue: 0.1, alpha: 1.0)),
                                children: [
                                    Text((
                                        text: "Continue Quest",
                                        text_style: TextStyle(
                                            font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                            font_size: 16.0,
                                            color: Srgba(red: 0.7, green: 0.7, blue: 0.7, alpha: 1.0),
                                        ),
                                    )),
                                ],
                                marker: ContinueButton,
                            )),
                            
                            // Settings Button
                            Button((
                                styles: Some(Style(
                                    width: Px(250.0),
                                    height: Px(60.0),
                                    justify_content: Center,
                                    align_items: Center,
                                    border: UiRect::all(Px(2.0)),
                                )),
                                background_color: Some(Srgba(red: 0.05, green: 0.1, blue: 0.2, alpha: 0.9)),
                                border_color: Some(Srgba(red: 0.1, green: 0.2, blue: 0.6, alpha: 1.0)),
                                children: [
                                    Text((
                                        text: "Settings",
                                        text_style: TextStyle(
                                            font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                            font_size: 16.0,
                                            color: Srgba(red: 0.7, green: 0.7, blue: 0.7, alpha: 1.0),
                                        ),
                                    )),
                                ],
                                marker: SettingsButton,
                            )),
                            
                            // Exit Button
                            Button((
                                styles: Some(Style(
                                    width: Px(250.0),
                                    height: Px(60.0),
                                    justify_content: Center,
                                    align_items: Center,
                                    border: UiRect::all(Px(2.0)),
                                )),
                                background_color: Some(Srgba(red: 0.1, green: 0.1, blue: 0.1, alpha: 0.9)),
                                border_color: Some(Srgba(red: 0.3, green: 0.3, blue: 0.3, alpha: 1.0)),
                                children: [
                                    Text((
                                        text: "Escape",
                                        text_style: TextStyle(
                                            font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                            font_size: 16.0,
                                            color: Srgba(red: 0.6, green: 0.6, blue: 0.6, alpha: 1.0),
                                        ),
                                    )),
                                ],
                                marker: ExitButton,
                            )),
                        ],
                    ),
                    
                    // Atmospheric footer text
                    Node(
                        styles: Some(Style(
                            position_type: Absolute,
                            left: Percent(50.0),
                            top: Percent(90.0),
                            width: Auto,
                            height: Auto,
                            margin: UiRect(
                                left: Percent(-20.0),
                                top: Px(0.0),
                                right: Px(0.0),
                                bottom: Px(0.0),
                            ),
                        )),
                        children: [
                            Text((
                                text: "The dragon stirs... your fate awaits",
                                text_style: TextStyle(
                                    font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                    font_size: 10.0,
                                    color: Srgba(red: 0.5, green: 0.5, blue: 0.5, alpha: 1.0),
                                ),
                            )),
                        ],
                    ),
                ],
            ),
        ],
    )
)