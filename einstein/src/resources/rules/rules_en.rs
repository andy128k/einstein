use super::base::TextItem;

pub const RULES: &[TextItem] = &[
    TextItem::Text("The game goal is to open all cards in square of 6x6 cards.
        When every card is open, field looks like this:"),
    TextItem::Image(resource!("./opensquare.bmp")),
    TextItem::Text("Every row of square contains cards of one type only.  For example,
        first row contains arabic digits, second - letters, third - rome digits,
        fouths - dices, fifth - geometric figures, sixs - mathematic symbols."),
    TextItem::Text("Use logic and open cards with method of exclusion.  If card doesn't
        opened, cell contains every possible cards. For example,"),
    TextItem::Image(resource!("./closed.bmp")),
    TextItem::Text("means that this cell may contain every rome digit with exception of 
        III (because card with III image is absent).  To open card click on
        small image with left mouse button.  To exclude card click with right
        mouse button."),
    TextItem::Text("Use tips to solve this puzzle.  There is two types of tips:
        horizontal and vertical.  Vertical tips located at screen bottom.
        For example, vertical tip"),
    TextItem::Image(resource!("./verthint.bmp")),
    TextItem::Text("means that letter 'B' and '+' sign located in the same column."),
    TextItem::Text("Horizontal tips located at the right side of the puzzle square.
        There is few type of horizontal tips.  First type of horizontal
        tip says that two cards located at neighbour columns, but it is
        unknown, which one is at the right side and thich is at the left:"),
    TextItem::Image(resource!("./hornearhint.bmp")),
    TextItem::Text("Second tip type means that one cards is at the left of another.
        It says nothing about distance between that cards.  They may be
        neighbour columns or at the opposite sides of puzzle field:"),
    TextItem::Image(resource!("./horposhint.bmp")),
    TextItem::Text("The last type of tip means that one card is located between
        two another cards:"),
    TextItem::Image(resource!("./horbetweenhint.bmp")),
    TextItem::Text("All three cards must be located in neighbour columns, central
        card is always between other two, but it is unknown, which card is located
        at the right side and which at the left."),
    TextItem::Text("If you no longer need some tip, remove it by right mouse button click.
        You can always see removed tips by pressing 'Switch' button."),
];
