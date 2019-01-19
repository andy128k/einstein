use super::base::TextItem;

pub const RULES: &[TextItem] = &[
    TextItem::Text("Das Ziel des Spieles besteht darin, alle Karten in einem 6x6-Quadrat
        aufzudecken. Wenn alle Karten offen sind, sieht das Feld wie folgt aus:"),
    TextItem::Image(resource!("./opensquare.bmp")),
    TextItem::Text("Jede Zeile des Quadrat enthält nur Karten eines Typs. Beispielsweise enthält
        die erste Zeile arabische Zahlen, die zweite Buchstaben, die dritte römische
        Zahlen, die vierte Würfel, die fünfte geometrische Figuren und die sechste
        mathematische Symbole."),
    TextItem::Text("Verwenden Sie Logik und öffnen Sie Karten mit der Ausschlussmethode.
        Falls eine Karte sich nicht öffnet, enthält die Zelle alle möglichen
        Karten. Zum Beispiel bedeutet"),
    TextItem::Image(resource!("./closed.bmp")),
    TextItem::Text("dass diese Zelle jede römische Zahl außer der III enthalten könnte 
        (da die Karte mit dem Bild III fehlt). Um eine Karte zu öffnen, klicken 
        Sie mit der linken Maustaste auf das kleine Bild . Um eine Karte 
        auszuschließen, klicken Sie mit der rechten Maustaste."),
    TextItem::Text("Verwenden Sie Tipps, um das Puzzle zu lösen. Es gibt zwei Arten von 
        Tipps: Horizontale und Vertikale. Vertikale Tipps befinden sich unten
        am Bildschirm. Zum Beispiel bedeutet der vertikale Tipp"),
    TextItem::Image(resource!("./verthint.bmp")),
    TextItem::Text("dass der Buchstabe »B« und das Zeichen »+« sich in der gleichen Spalte befinden."),
    TextItem::Text("Horizontale Tipps befinden sich auf der rechten Seite des Puzzlequadrats.
        Es gibt eine Reihe von Arten von horizontalen Tipps. Die erste Art von
        horizontalen Tipps besagt, dass zwei Karten sich in benachbarten Spalten
        befinden, es aber unbekannt ist, welche sich auf der rechten und welche
        sich auf der linken Seite befindet:"),
    TextItem::Image(resource!("./hornearhint.bmp")),
    TextItem::Text("Die zweite Art von Tipp bedeutet, dass sich eine Karte links von einer
        anderen befindet. Es sagt nichts über die Distanz zwischen den Karten
        aus. Sie können sich in benachbarten Spalten oder auf gegenüberliegenden
        Seiten des Puzzles befinden:"),
    TextItem::Image(resource!("./horposhint.bmp")),
    TextItem::Text("Die letzte Art von Tipp bedeutet, dass sich eine Karte zwischen zwei
        anderen Karten befindet:"),
    TextItem::Image(resource!("./horbetweenhint.bmp")),
    TextItem::Text("Alle drei Karten müssen sich in benachbarten Spalten befinden, die zentrale
        Karte ist immer zwischen den anderen zwei, aber es ist unbekannt, welche
        Karte sich auf der rechten Seite und welche sich auf der linken befindet."),
    TextItem::Text("Falls Sie einen Tipp nicht mehr benötigen, entfernen Sie ihn durch einen
        rechten Mausklick. Sie können entfernte Tipps immer durch Drücken des
        »Umschalte«-Knopfs wieder sehen."),
    TextItem::Text("Wer ein Spiel lädt oder neu startet, kommt nicht in die Ruhmeshalle."),
];
