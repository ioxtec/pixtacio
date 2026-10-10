# Pixtacio: mapa kodu

## Core

Core nie zna GPUI ani bieżącej sesji. Operacje odczytu są synchroniczne;
desktop uruchamia je poza wątkiem UI.

- `image/catalog.rs`: lista ścieżek i wstępne filtrowanie rozszerzeń.
- `image/image_file.rs`: bajty pojedynczego pliku i metadane. To nie piksele.
- `image/loader.rs`: odczyt jednego pliku i utworzenie `ImageFile`.
- `image/metadata.rs`: istniejący odczyt wymiarów, formatu i EXIF.
- `image/decoder.rs`: TODO — dekodowanie do `DynamicImage`.
- `image/thumbnail.rs`: TODO — tworzenie miniaturki.
- `image/filters.rs`: TODO — jasność i zmiana rozmiaru.
- `image/writer.rs`: TODO — kodowanie i zapis w wybranym formacie.
- `video/`: zarezerwowane miejsce na przyszłą obsługę wideo.

Stary moduł `io` zastąpiono modułem `image`. Zamiast
`ImageFile::new(path)` używaj `image::loader::load_file(path)`.
Nie ma już operacji wczytującej cały folder do `Vec<ImageFile>`.

## Desktop

- `main.rs`: uruchomienie.
- `app.rs`: składanie widoków, subskrypcje menu i nawigacja.
- `dialogs/file_dialogs.rs`: wybór folderu; wybór pliku pozostaje TODO.
- `image/image_session.rs`: zwykła struktura Rust, bez GPUI.
  Przechowuje folder, ścieżki i indeks wyboru. Zachowano przejście do następnego
  obrazu z zawijaniem; wybór indeksu, poprzedni obraz i czyszczenie są TODO.
- `image/image_controller.rs`: adapter GPUI, przechowujący sesję.
  Uruchamia odczyt katalogu w tle, zapisuje wynik i powiadamia widoki.
  Otwieranie pojedynczego pliku oraz ładowanie/dekodowanie wyboru są TODO.
- `ui/views/image_view.rs`: obserwuje kontroler i pokazuje status/wybraną ścieżkę.
  Piksele nie są jeszcze wyświetlane; zoom pozostaje TODO.
- `ui/sidebar.rs`: menu aplikacji, bez listy zdjęć.
- `ui/thumbnail_strip.rs`: szkielet przyszłego paska miniaturek, jeszcze nie
  zamontowany w oknie. Pole kontrolera jest uchwytem do wspólnego stanu.
- `ui/toolbar.rs`, `ui/navigation.rs`: polecenia i wybór ekranu.
- `ui/views/settings_view.rs`, `video_view.rs`: dotychczasowe widoki.

## Działający przepływ

Toolbar → app → dialog → ImageController → catalog::list_images w tle
→ ImageSession → powiadomienie → ImageView.

Folder zawiera tylko ścieżki. Załadowany obraz i ograniczony cache miniaturek
należy dodać przy implementacji dekodowania; nie kopiować całych obrazów do
każdego widoku. Przy przyszłym ładowaniu wyboru odrzucaj wyniki starszych żądań.

## Kolejność nauki

1. Zaimplementuj i sprawdź `decode_image` na jednym pliku.
2. Uzupełnij `load_selected_image`: odczyt/dekodowanie w tle, zapis wyniku
   przez `Entity::update`, bez trzymania `&mut self` przez await.
3. Przekaż gotowe piksele do renderera GPUI.
4. Uzupełnij wybór pliku, nawigację i wejście ze ścieżki argumentu programu.
5. Dodaj miniaturki na żądanie, limit pamięci cache i ograniczenie równoległości.

`todo!()` kompiluje się, ale panikuje po wywołaniu. Obecny przycisk otwarcia
folderu nie wywołuje tych szkieletów. Ostrzeżenia o nieużywanym kodzie są
oczekiwane do czasu podłączenia nowych funkcji.
