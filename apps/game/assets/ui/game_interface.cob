Cob(
    Scene(
        children: [
            // Main game container
            Node(
                styles: Some(Style(
                    position_type: Absolute,
                    left: Px(0.0),
                    top: Px(0.0),
                    width: Percent(100.0),
                    height: Percent(100.0),
                )),
                children: [
                    // Game world viewport (center area)
                    Node(
                        styles: Some(Style(
                            position_type: Absolute,
                            left: Px(200.0),
                            top: Px(80.0),
                            width: Calc(Percent(100.0) - Px(400.0)),
                            height: Calc(Percent(100.0) - Px(160.0)),
                            border: UiRect::all(Px(3.0)),
                        )),
                        border_color: Some(Srgba(red: 0.4, green: 0.2, blue: 0.1, alpha: 1.0)),
                        background_color: Some(Srgba(red: 0.0, green: 0.0, blue: 0.0, alpha: 0.3)),
                        marker: GameViewport,
                    ),
                    
                    // Top status bar
                    Node(
                        styles: Some(Style(
                            position_type: Absolute,
                            left: Px(0.0),
                            top: Px(0.0),
                            width: Percent(100.0),
                            height: Px(80.0),
                            flex_direction: Row,
                            justify_content: SpaceBetween,
                            align_items: Center,
                            padding: UiRect::all(Px(10.0)),
                        )),
                        background_color: Some(Srgba(red: 0.1, green: 0.05, blue: 0.05, alpha: 0.95)),
                        border_color: Some(Srgba(red: 0.3, green: 0.15, blue: 0.1, alpha: 1.0)),
                        children: [
                            // Player name and level
                            Node(
                                styles: Some(Style(
                                    flex_direction: Column,
                                    align_items: FlexStart,
                                )),
                                children: [
                                    Text((
                                        text: "Cursed Wanderer",
                                        text_style: TextStyle(
                                            font: "https://fonts.googleapis.com/css2?family=Cinzel:wght@600&display=swap",
                                            font_size: 18.0,
                                            color: Srgba(red: 0.9, green: 0.8, blue: 0.7, alpha: 1.0),
                                        ),
                                    )),
                                    Text((
                                        text: "Dread Level: 15",
                                        text_style: TextStyle(
                                            font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                            font_size: 10.0,
                                            color: Srgba(red: 0.8, green: 0.3, blue: 0.3, alpha: 1.0),
                                        ),
                                    )),
                                ],
                            ),
                            
                            // Current location
                            Node(
                                children: [
                                    Text((
                                        text: "Ashen Meadows",
                                        text_style: TextStyle(
                                            font: "https://fonts.googleapis.com/css2?family=Cinzel:wght@400&display=swap",
                                            font_size: 16.0,
                                            color: Srgba(red: 0.7, green: 0.7, blue: 0.6, alpha: 1.0),
                                        ),
                                    )),
                                ],
                            ),
                            
                            // Day/time indicator
                            Node(
                                children: [
                                    Text((
                                        text: "Day 3, Dusk",
                                        text_style: TextStyle(
                                            font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                            font_size: 10.0,
                                            color: Srgba(red: 0.6, green: 0.5, blue: 0.4, alpha: 1.0),
                                        ),
                                    )),
                                ],
                            ),
                        ],
                    ),
                    
                    // Left panel - Character stats and health
                    Node(
                        styles: Some(Style(
                            position_type: Absolute,
                            left: Px(0.0),
                            top: Px(80.0),
                            width: Px(200.0),
                            height: Calc(Percent(100.0) - Px(160.0)),
                            flex_direction: Column,
                            padding: UiRect::all(Px(10.0)),
                            row_gap: Px(15.0),
                        )),
                        background_color: Some(Srgba(red: 0.08, green: 0.04, blue: 0.04, alpha: 0.95)),
                        border_color: Some(Srgba(red: 0.3, green: 0.15, blue: 0.1, alpha: 1.0)),
                        children: [
                            // Character portrait placeholder
                            Node(
                                styles: Some(Style(
                                    width: Px(120.0),
                                    height: Px(120.0),
                                    align_self: Center,
                                    border: UiRect::all(Px(2.0)),
                                )),
                                background_color: Some(Srgba(red: 0.15, green: 0.1, blue: 0.1, alpha: 1.0)),
                                border_color: Some(Srgba(red: 0.4, green: 0.2, blue: 0.1, alpha: 1.0)),
                                marker: CharacterPortrait,
                            ),
                            
                            // Health bar
                            Node(
                                styles: Some(Style(
                                    width: Percent(100.0),
                                    height: Px(25.0),
                                    flex_direction: Column,
                                )),
                                children: [
                                    Text((
                                        text: "Health",
                                        text_style: TextStyle(
                                            font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                            font_size: 8.0,
                                            color: Srgba(red: 0.8, green: 0.8, blue: 0.8, alpha: 1.0),
                                        ),
                                    )),
                                    Node(
                                        styles: Some(Style(
                                            width: Percent(100.0),
                                            height: Px(12.0),
                                            border: UiRect::all(Px(1.0)),
                                        )),
                                        background_color: Some(Srgba(red: 0.1, green: 0.1, blue: 0.1, alpha: 1.0)),
                                        border_color: Some(Srgba(red: 0.4, green: 0.2, blue: 0.1, alpha: 1.0)),
                                        children: [
                                            Node(
                                                styles: Some(Style(
                                                    width: Percent(75.0),
                                                    height: Percent(100.0),
                                                )),
                                                background_color: Some(Srgba(red: 0.6, green: 0.1, blue: 0.1, alpha: 1.0)),
                                                marker: HealthBar,
                                            ),
                                        ],
                                    ),
                                ],
                            ),
                            
                            // Sanity bar
                            Node(
                                styles: Some(Style(
                                    width: Percent(100.0),
                                    height: Px(25.0),
                                    flex_direction: Column,
                                )),
                                children: [
                                    Text((
                                        text: "Sanity",
                                        text_style: TextStyle(
                                            font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                            font_size: 8.0,
                                            color: Srgba(red: 0.8, green: 0.8, blue: 0.8, alpha: 1.0),
                                        ),
                                    )),
                                    Node(
                                        styles: Some(Style(
                                            width: Percent(100.0),
                                            height: Px(12.0),
                                            border: UiRect::all(Px(1.0)),
                                        )),
                                        background_color: Some(Srgba(red: 0.1, green: 0.1, blue: 0.1, alpha: 1.0)),
                                        border_color: Some(Srgba(red: 0.2, green: 0.2, blue: 0.4, alpha: 1.0)),
                                        children: [
                                            Node(
                                                styles: Some(Style(
                                                    width: Percent(60.0),
                                                    height: Percent(100.0),
                                                )),
                                                background_color: Some(Srgba(red: 0.1, green: 0.1, blue: 0.6, alpha: 1.0)),
                                                marker: SanityBar,
                                            ),
                                        ],
                                    ),
                                ],
                            ),
                            
                            // Character stats
                            Node(
                                styles: Some(Style(
                                    width: Percent(100.0),
                                    flex_direction: Column,
                                    row_gap: Px(5.0),
                                )),
                                children: [
                                    Text((
                                        text: "STATS",
                                        text_style: TextStyle(
                                            font: "https://fonts.googleapis.com/css2?family=Cinzel:wght@600&display=swap",
                                            font_size: 12.0,
                                            color: Srgba(red: 0.8, green: 0.6, blue: 0.4, alpha: 1.0),
                                        ),
                                    )),
                                    Text((
                                        text: "Corruption: 15",
                                        text_style: TextStyle(
                                            font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                            font_size: 8.0,
                                            color: Srgba(red: 0.8, green: 0.3, blue: 0.8, alpha: 1.0),
                                        ),
                                    )),
                                    Text((
                                        text: "Distance: 23 hexes",
                                        text_style: TextStyle(
                                            font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                            font_size: 8.0,
                                            color: Srgba(red: 0.6, green: 0.6, blue: 0.6, alpha: 1.0),
                                        ),
                                    )),
                                ],
                            ),
                        ],
                    ),
                    
                    // Right panel - Inventory and actions
                    Node(
                        styles: Some(Style(
                            position_type: Absolute,
                            right: Px(0.0),
                            top: Px(80.0),
                            width: Px(200.0),
                            height: Calc(Percent(100.0) - Px(160.0)),
                            flex_direction: Column,
                            padding: UiRect::all(Px(10.0)),
                            row_gap: Px(15.0),
                        )),
                        background_color: Some(Srgba(red: 0.04, green: 0.08, blue: 0.04, alpha: 0.95)),
                        border_color: Some(Srgba(red: 0.15, green: 0.3, blue: 0.1, alpha: 1.0)),
                        children: [
                            // Inventory header
                            Text((
                                text: "INVENTORY",
                                text_style: TextStyle(
                                    font: "https://fonts.googleapis.com/css2?family=Cinzel:wght@600&display=swap",
                                    font_size: 12.0,
                                    color: Srgba(red: 0.6, green: 0.8, blue: 0.4, alpha: 1.0),
                                ),
                            )),
                            
                            // Inventory grid
                            Node(
                                styles: Some(Style(
                                    width: Percent(100.0),
                                    height: Px(200.0),
                                    display: Grid,
                                    grid_template_columns: RepeatedGridTrack(
                                        repetitions: 4,
                                        tracks: vec![Fr(1.0)],
                                    ),
                                    grid_template_rows: RepeatedGridTrack(
                                        repetitions: 5,
                                        tracks: vec![Fr(1.0)],
                                    ),
                                    gap: Size::all(Px(2.0)),
                                )),
                                children: [
                                    // Inventory slots (16 total)
                                    Node(
                                        styles: Some(Style(
                                            border: UiRect::all(Px(1.0)),
                                        )),
                                        background_color: Some(Srgba(red: 0.1, green: 0.1, blue: 0.1, alpha: 0.8)),
                                        border_color: Some(Srgba(red: 0.3, green: 0.3, blue: 0.2, alpha: 1.0)),
                                        marker: InventorySlot,
                                    ),
                                ],
                            ),
                            
                            // Quick actions
                            Node(
                                styles: Some(Style(
                                    width: Percent(100.0),
                                    flex_direction: Column,
                                    row_gap: Px(8.0),
                                )),
                                children: [
                                    Text((
                                        text: "ACTIONS",
                                        text_style: TextStyle(
                                            font: "https://fonts.googleapis.com/css2?family=Cinzel:wght@600&display=swap",
                                            font_size: 12.0,
                                            color: Srgba(red: 0.8, green: 0.6, blue: 0.4, alpha: 1.0),
                                        ),
                                    )),
                                    
                                    // Rest button
                                    Button((
                                        styles: Some(Style(
                                            width: Percent(100.0),
                                            height: Px(35.0),
                                            justify_content: Center,
                                            align_items: Center,
                                            border: UiRect::all(Px(1.0)),
                                        )),
                                        background_color: Some(Srgba(red: 0.1, green: 0.2, blue: 0.1, alpha: 0.9)),
                                        border_color: Some(Srgba(red: 0.2, green: 0.4, blue: 0.2, alpha: 1.0)),
                                        children: [
                                            Text((
                                                text: "Rest",
                                                text_style: TextStyle(
                                                    font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                                    font_size: 10.0,
                                                    color: Srgba(red: 0.8, green: 0.8, blue: 0.8, alpha: 1.0),
                                                ),
                                            )),
                                        ],
                                        marker: RestButton,
                                    )),
                                    
                                    // Search button
                                    Button((
                                        styles: Some(Style(
                                            width: Percent(100.0),
                                            height: Px(35.0),
                                            justify_content: Center,
                                            align_items: Center,
                                            border: UiRect::all(Px(1.0)),
                                        )),
                                        background_color: Some(Srgba(red: 0.15, green: 0.1, blue: 0.2, alpha: 0.9)),
                                        border_color: Some(Srgba(red: 0.3, green: 0.2, blue: 0.4, alpha: 1.0)),
                                        children: [
                                            Text((
                                                text: "Search",
                                                text_style: TextStyle(
                                                    font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                                    font_size: 10.0,
                                                    color: Srgba(red: 0.8, green: 0.8, blue: 0.8, alpha: 1.0),
                                                ),
                                            )),
                                        ],
                                        marker: SearchButton,
                                    )),
                                ],
                            ),
                        ],
                    ),
                    
                    // Bottom panel - Companions and messages
                    Node(
                        styles: Some(Style(
                            position_type: Absolute,
                            left: Px(0.0),
                            bottom: Px(0.0),
                            width: Percent(100.0),
                            height: Px(80.0),
                            flex_direction: Row,
                            padding: UiRect::all(Px(10.0)),
                            column_gap: Px(15.0),
                        )),
                        background_color: Some(Srgba(red: 0.05, green: 0.05, blue: 0.1, alpha: 0.95)),
                        border_color: Some(Srgba(red: 0.15, green: 0.15, blue: 0.3, alpha: 1.0)),
                        children: [
                            // Companion status
                            Node(
                                styles: Some(Style(
                                    width: Px(300.0),
                                    flex_direction: Row,
                                    column_gap: Px(10.0),
                                    align_items: Center,
                                )),
                                children: [
                                    // Companion portrait
                                    Node(
                                        styles: Some(Style(
                                            width: Px(50.0),
                                            height: Px(50.0),
                                            border: UiRect::all(Px(1.0)),
                                        )),
                                        background_color: Some(Srgba(red: 0.1, green: 0.15, blue: 0.1, alpha: 1.0)),
                                        border_color: Some(Srgba(red: 0.2, green: 0.4, blue: 0.2, alpha: 1.0)),
                                        marker: CompanionPortrait,
                                    ),
                                    
                                    // Companion info
                                    Node(
                                        styles: Some(Style(
                                            flex_direction: Column,
                                            row_gap: Px(2.0),
                                        )),
                                        children: [
                                            Text((
                                                text: "Elara the Guide",
                                                text_style: TextStyle(
                                                    font: "https://fonts.googleapis.com/css2?family=Cinzel:wght@400&display=swap",
                                                    font_size: 10.0,
                                                    color: Srgba(red: 0.7, green: 0.8, blue: 0.6, alpha: 1.0),
                                                ),
                                            )),
                                            Text((
                                                text: "Trauma: Moderate",
                                                text_style: TextStyle(
                                                    font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                                    font_size: 7.0,
                                                    color: Srgba(red: 0.8, green: 0.5, blue: 0.2, alpha: 1.0),
                                                ),
                                            )),
                                        ],
                                    ),
                                ],
                            ),
                            
                            // Message/dialogue area
                            Node(
                                styles: Some(Style(
                                    flex_grow: 1.0,
                                    height: Percent(100.0),
                                    padding: UiRect::all(Px(8.0)),
                                    border: UiRect::all(Px(1.0)),
                                )),
                                background_color: Some(Srgba(red: 0.05, green: 0.05, blue: 0.05, alpha: 0.9)),
                                border_color: Some(Srgba(red: 0.2, green: 0.2, blue: 0.2, alpha: 1.0)),
                                children: [
                                    Text((
                                        text: "The mist grows thicker as you venture deeper into the cursed lands...",
                                        text_style: TextStyle(
                                            font: "https://fonts.googleapis.com/css2?family=Cinzel:wght@400&display=swap",
                                            font_size: 9.0,
                                            color: Srgba(red: 0.7, green: 0.7, blue: 0.7, alpha: 1.0),
                                        ),
                                    )),
                                ],
                                marker: MessageArea,
                            ),
                        ],
                    ),
                    
                    // Mobile touch controls overlay (hidden on desktop)
                    Node(
                        styles: Some(Style(
                            position_type: Absolute,
                            right: Px(10.0),
                            bottom: Px(90.0),
                            width: Px(120.0),
                            height: Px(120.0),
                            display: None, // Will be shown on mobile via system
                        )),
                        marker: TouchControlsOverlay,
                        children: [
                            // Virtual D-pad for mobile
                            Node(
                                styles: Some(Style(
                                    width: Percent(100.0),
                                    height: Percent(100.0),
                                    display: Grid,
                                    grid_template_columns: RepeatedGridTrack(
                                        repetitions: 3,
                                        tracks: vec![Fr(1.0)],
                                    ),
                                    grid_template_rows: RepeatedGridTrack(
                                        repetitions: 3,
                                        tracks: vec![Fr(1.0)],
                                    ),
                                    gap: Size::all(Px(2.0)),
                                )),
                                children: [
                                    // Virtual movement buttons (Q, W, E, A, S, D positions)
                                    Button((
                                        styles: Some(Style(
                                            grid_column: GridPlacement::start_end(1, 2),
                                            grid_row: GridPlacement::start_end(1, 2),
                                            justify_content: Center,
                                            align_items: Center,
                                        )),
                                        background_color: Some(Srgba(red: 0.2, green: 0.2, blue: 0.2, alpha: 0.7)),
                                        children: [
                                            Text((
                                                text: "Q",
                                                text_style: TextStyle(
                                                    font: "https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap",
                                                    font_size: 8.0,
                                                    color: Srgba(red: 0.9, green: 0.9, blue: 0.9, alpha: 1.0),
                                                ),
                                            )),
                                        ],
                                        marker: VirtualButton("Q"),
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
