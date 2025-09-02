Cob(
    Scene(
        children: [
            // Character creator background
            Node(
                styles: Some(Style(
                    position_type: Absolute,
                    left: Px(0.0),
                    top: Px(0.0),
                    width: Percent(100.0),
                    height: Percent(100.0),
                    flex_direction: Row,
                )),
                background_color: Some(Srgba(red: 0.02, green: 0.02, blue: 0.02, alpha: 1.0)),
                children: [
                    // Left side - Character preview
                    Node(
                        styles: Some(Style(
                            width: Percent(60.0),
                            height: Percent(100.0),
                            justify_content: Center,
                            align_items: Center,
                            flex_direction: Column,
                        )),
                        background_color: Some(Srgba(red: 0.05, green: 0.05, blue: 0.05, alpha: 1.0)),
                        children: [
                            // Preview title
                            Text((
                                text: "Your Avatar",
                                text_style: TextStyle(
                                    font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                    font_size: 20.0,
                                    color: Srgba(red: 0.8, green: 0.2, blue: 0.2, alpha: 1.0),
                                ),
                            )),
                            
                            // Character preview container
                            Node(
                                styles: Some(Style(
                                    width: Px(400.0),
                                    height: Px(400.0),
                                    margin: UiRect::all(Px(20.0)),
                                    border: UiRect::all(Px(2.0)),
                                )),
                                background_color: Some(Srgba(red: 0.1, green: 0.1, blue: 0.1, alpha: 0.8)),
                                border_color: Some(Srgba(red: 0.4, green: 0.1, blue: 0.1, alpha: 1.0)),
                                marker: CharacterPreview,
                            ),
                        ],
                    ),
                    
                    // Right side - Customization options
                    Node(
                        styles: Some(Style(
                            width: Percent(40.0),
                            height: Percent(100.0),
                            flex_direction: Column,
                            padding: UiRect::all(Px(30.0)),
                            row_gap: Px(20.0),
                        )),
                        background_color: Some(Srgba(red: 0.08, green: 0.08, blue: 0.08, alpha: 1.0)),
                        children: [
                            // Title
                            Text((
                                text: "Create Your Hero",
                                text_style: TextStyle(
                                    font: "https://fonts.googleapis.com/css2?family=Creepster&display=swap",
                                    font_size: 28.0,
                                    color: Srgba(red: 0.9, green: 0.1, blue: 0.1, alpha: 1.0),
                                ),
                            )),
                            
                            // Gender selection
                            Node(
                                styles: Some(Style(
                                    flex_direction: Column,
                                    row_gap: Px(10.0),
                                )),
                                children: [
                                    Text((
                                        text: "Gender:",
                                        text_style: TextStyle(
                                            font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                            font_size: 14.0,
                                            color: Srgba(red: 0.9, green: 0.9, blue: 0.9, alpha: 1.0),
                                        ),
                                    )),
                                    
                                    Node(
                                        styles: Some(Style(
                                            flex_direction: Row,
                                            column_gap: Px(15.0),
                                        )),
                                        children: [
                                            Button((
                                                styles: Some(Style(
                                                    width: Px(100.0),
                                                    height: Px(40.0),
                                                    justify_content: Center,
                                                    align_items: Center,
                                                    border: UiRect::all(Px(2.0)),
                                                )),
                                                background_color: Some(Srgba(red: 0.2, green: 0.2, blue: 0.4, alpha: 0.9)),
                                                border_color: Some(Srgba(red: 0.4, green: 0.4, blue: 0.8, alpha: 1.0)),
                                                children: [
                                                    Text((
                                                        text: "Male",
                                                        text_style: TextStyle(
                                                            font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                                            font_size: 12.0,
                                                            color: Srgba(red: 0.9, green: 0.9, blue: 0.9, alpha: 1.0),
                                                        ),
                                                    )),
                                                ],
                                                marker: GenderSelector(Male),
                                            )),
                                            
                                            Button((
                                                styles: Some(Style(
                                                    width: Px(100.0),
                                                    height: Px(40.0),
                                                    justify_content: Center,
                                                    align_items: Center,
                                                    border: UiRect::all(Px(2.0)),
                                                )),
                                                background_color: Some(Srgba(red: 0.4, green: 0.2, blue: 0.4, alpha: 0.9)),
                                                border_color: Some(Srgba(red: 0.8, green: 0.4, blue: 0.8, alpha: 1.0)),
                                                children: [
                                                    Text((
                                                        text: "Female",
                                                        text_style: TextStyle(
                                                            font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                                            font_size: 12.0,
                                                            color: Srgba(red: 0.9, green: 0.9, blue: 0.9, alpha: 1.0),
                                                        ),
                                                    )),
                                                ],
                                                marker: GenderSelector(Female),
                                            )),
                                        ],
                                    ),
                                ],
                            ),
                            
                            // Appearance sliders
                            Node(
                                styles: Some(Style(
                                    flex_direction: Column,
                                    row_gap: Px(15.0),
                                )),
                                children: [
                                    Text((
                                        text: "Appearance:",
                                        text_style: TextStyle(
                                            font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                            font_size: 14.0,
                                            color: Srgba(red: 0.9, green: 0.9, blue: 0.9, alpha: 1.0),
                                        ),
                                    )),
                                    
                                    // Hair Style
                                    Node(
                                        styles: Some(Style(
                                            flex_direction: Row,
                                            justify_content: SpaceBetween,
                                            align_items: Center,
                                        )),
                                        children: [
                                            Text((
                                                text: "Hair:",
                                                text_style: TextStyle(
                                                    font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                                    font_size: 10.0,
                                                    color: Srgba(red: 0.8, green: 0.8, blue: 0.8, alpha: 1.0),
                                                ),
                                            )),
                                            
                                            Node(
                                                styles: Some(Style(
                                                    width: Px(150.0),
                                                    height: Px(20.0),
                                                    border: UiRect::all(Px(1.0)),
                                                )),
                                                background_color: Some(Srgba(red: 0.15, green: 0.15, blue: 0.15, alpha: 1.0)),
                                                border_color: Some(Srgba(red: 0.5, green: 0.5, blue: 0.5, alpha: 1.0)),
                                                marker: AppearanceSlider(HairStyle, 0),
                                            ),
                                        ],
                                    ),
                                    
                                    // Skin Tone
                                    Node(
                                        styles: Some(Style(
                                            flex_direction: Row,
                                            justify_content: SpaceBetween,
                                            align_items: Center,
                                        )),
                                        children: [
                                            Text((
                                                text: "Skin:",
                                                text_style: TextStyle(
                                                    font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                                    font_size: 10.0,
                                                    color: Srgba(red: 0.8, green: 0.8, blue: 0.8, alpha: 1.0),
                                                ),
                                            )),
                                            
                                            Node(
                                                styles: Some(Style(
                                                    width: Px(150.0),
                                                    height: Px(20.0),
                                                    border: UiRect::all(Px(1.0)),
                                                )),
                                                background_color: Some(Srgba(red: 0.15, green: 0.15, blue: 0.15, alpha: 1.0)),
                                                border_color: Some(Srgba(red: 0.5, green: 0.5, blue: 0.5, alpha: 1.0)),
                                                marker: AppearanceSlider(SkinTone, 0),
                                            ),
                                        ],
                                    ),
                                    
                                    // Height
                                    Node(
                                        styles: Some(Style(
                                            flex_direction: Row,
                                            justify_content: SpaceBetween,
                                            align_items: Center,
                                        )),
                                        children: [
                                            Text((
                                                text: "Height:",
                                                text_style: TextStyle(
                                                    font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                                    font_size: 10.0,
                                                    color: Srgba(red: 0.8, green: 0.8, blue: 0.8, alpha: 1.0),
                                                ),
                                            )),
                                            
                                            Node(
                                                styles: Some(Style(
                                                    width: Px(150.0),
                                                    height: Px(20.0),
                                                    border: UiRect::all(Px(1.0)),
                                                )),
                                                background_color: Some(Srgba(red: 0.15, green: 0.15, blue: 0.15, alpha: 1.0)),
                                                border_color: Some(Srgba(red: 0.5, green: 0.5, blue: 0.5, alpha: 1.0)),
                                                marker: AppearanceSlider(Height, 0),
                                            ),
                                        ],
                                    ),
                                    
                                    // Build
                                    Node(
                                        styles: Some(Style(
                                            flex_direction: Row,
                                            justify_content: SpaceBetween,
                                            align_items: Center,
                                        )),
                                        children: [
                                            Text((
                                                text: "Build:",
                                                text_style: TextStyle(
                                                    font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                                    font_size: 10.0,
                                                    color: Srgba(red: 0.8, green: 0.8, blue: 0.8, alpha: 1.0),
                                                ),
                                            )),
                                            
                                            Node(
                                                styles: Some(Style(
                                                    width: Px(150.0),
                                                    height: Px(20.0),
                                                    border: UiRect::all(Px(1.0)),
                                                )),
                                                background_color: Some(Srgba(red: 0.15, green: 0.15, blue: 0.15, alpha: 1.0)),
                                                border_color: Some(Srgba(red: 0.5, green: 0.5, blue: 0.5, alpha: 1.0)),
                                                marker: AppearanceSlider(Build, 0),
                                            ),
                                        ],
                                    ),
                                ],
                            ),
                            
                            // Action buttons
                            Node(
                                styles: Some(Style(
                                    flex_direction: Column,
                                    row_gap: Px(15.0),
                                    margin: UiRect(
                                        top: Px(30.0),
                                        left: Px(0.0),
                                        right: Px(0.0),
                                        bottom: Px(0.0),
                                    ),
                                )),
                                children: [
                                    Button((
                                        styles: Some(Style(
                                            width: Percent(100.0),
                                            height: Px(50.0),
                                            justify_content: Center,
                                            align_items: Center,
                                            border: UiRect::all(Px(2.0)),
                                        )),
                                        background_color: Some(Srgba(red: 0.2, green: 0.6, blue: 0.2, alpha: 0.9)),
                                        border_color: Some(Srgba(red: 0.4, green: 0.8, blue: 0.4, alpha: 1.0)),
                                        children: [
                                            Text((
                                                text: "Begin Adventure",
                                                text_style: TextStyle(
                                                    font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                                    font_size: 14.0,
                                                    color: Srgba(red: 0.9, green: 0.9, blue: 0.9, alpha: 1.0),
                                                ),
                                            )),
                                        ],
                                        marker: BeginAdventureButton,
                                    )),
                                    
                                    Button((
                                        styles: Some(Style(
                                            width: Percent(100.0),
                                            height: Px(40.0),
                                            justify_content: Center,
                                            align_items: Center,
                                            border: UiRect::all(Px(2.0)),
                                        )),
                                        background_color: Some(Srgba(red: 0.3, green: 0.3, blue: 0.3, alpha: 0.9)),
                                        border_color: Some(Srgba(red: 0.5, green: 0.5, blue: 0.5, alpha: 1.0)),
                                        children: [
                                            Text((
                                                text: "Back to Menu",
                                                text_style: TextStyle(
                                                    font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                                    font_size: 12.0,
                                                    color: Srgba(red: 0.8, green: 0.8, blue: 0.8, alpha: 1.0),
                                                ),
                                            )),
                                        ],
                                        marker: BackToMenuButton,
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