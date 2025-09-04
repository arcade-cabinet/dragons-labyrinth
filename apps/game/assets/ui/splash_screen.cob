Cob(
    Scene(
        children: [
            // Full screen splash image background
            Node(
                styles: Some(Style(
                    position_type: Absolute,
                    left: Px(0.0),
                    top: Px(0.0),
                    width: Percent(100.0),
                    height: Percent(100.0),
                )),
                background_image: Some("images/splash/dragons_labyrinth_splash.png"),
                children: [
                    // Dark overlay for text readability
                    Node(
                        styles: Some(Style(
                            position_type: Absolute,
                            left: Px(0.0),
                            top: Px(0.0),
                            width: Percent(100.0),
                            height: Percent(100.0),
                        )),
                        background_color: Some(Srgba(red: 0.0, green: 0.0, blue: 0.0, alpha: 0.4)),
                        children: [
                    // Dragon's Labyrinth title with horror glow effect
                    Node(
                        styles: Some(Style(
                            position_type: Absolute,
                            left: Percent(50.0),
                            top: Percent(20.0),
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
                                    font_size: 64.0,
                                    color: Srgba(red: 0.9, green: 0.1, blue: 0.1, alpha: 1.0),
                                ),
                            )),
                        ],
                    ),
                    
                    // Subtitle with ominous description
                    Node(
                        styles: Some(Style(
                            position_type: Absolute,
                            left: Percent(50.0),
                            top: Percent(35.0),
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
                                text: "A descent into madness awaits...",
                                text_style: TextStyle(
                                    font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                    font_size: 18.0,
                                    color: Srgba(red: 0.7, green: 0.7, blue: 0.7, alpha: 1.0),
                                ),
                            )),
                        ],
                    ),
                    
                    // Loading progress container
                    Node(
                        styles: Some(Style(
                            position_type: Absolute,
                            left: Percent(25.0),
                            top: Percent(70.0),
                            width: Percent(50.0),
                            height: Px(20.0),
                            border: UiRect::all(Px(2.0)),
                        )),
                        background_color: Some(Srgba(red: 0.1, green: 0.1, blue: 0.1, alpha: 0.8)),
                        border_color: Some(Srgba(red: 0.6, green: 0.1, blue: 0.1, alpha: 1.0)),
                        children: [
                            // Progress bar fill
                            Node(
                                styles: Some(Style(
                                    position_type: Absolute,
                                    left: Px(0.0),
                                    top: Px(0.0),
                                    width: Percent(0.0), // Will be updated by system
                                    height: Percent(100.0),
                                )),
                                background_color: Some(Srgba(red: 0.8, green: 0.2, blue: 0.2, alpha: 0.7)),
                                marker: ProgressBar,
                            ),
                        ],
                    ),
                    
                    // Loading text
                    Node(
                        styles: Some(Style(
                            position_type: Absolute,
                            left: Percent(50.0),
                            top: Percent(75.0),
                            width: Auto,
                            height: Auto,
                            margin: UiRect(
                                left: Percent(-15.0),
                                top: Px(0.0),
                                right: Px(0.0),
                                bottom: Px(0.0),
                            ),
                        )),
                        children: [
                            Text((
                                text: "Awakening ancient horrors...",
                                text_style: TextStyle(
                                    font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                    font_size: 14.0,
                                    color: Srgba(red: 0.8, green: 0.8, blue: 0.8, alpha: 1.0),
                                ),
                                marker: LoadingText,
                            )),
                        ],
                    ),
                    
                    // Press any key text (initially hidden)
                    Node(
                        styles: Some(Style(
                            position_type: Absolute,
                            left: Percent(50.0),
                            top: Percent(85.0),
                            width: Auto,
                            height: Auto,
                            margin: UiRect(
                                left: Percent(-12.0),
                                top: Px(0.0),
                                right: Px(0.0),
                                bottom: Px(0.0),
                            ),
                        )),
                        children: [
                            Text((
                                text: "Press SPACE to enter",
                                text_style: TextStyle(
                                    font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                    font_size: 12.0,
                                    color: Srgba(red: 0.9, green: 0.9, blue: 0.9, alpha: 0.0), // Initially invisible
                                ),
                                marker: PressKeyText,
                            )),
                        ],
                    ),
                        ],
                    ),
                ],
            ),
        ],
    )
)
