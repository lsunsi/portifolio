module Main exposing (main)

import Browser exposing (Document, UrlRequest(..))
import Browser.Navigation exposing (Key, pushUrl)
import Html exposing (Html, a, div, h1, img, span, text)
import Html.Attributes exposing (class, classList, href)
import Url exposing (Url)
import Url.Parser as UrlParser


urlToPage : Url -> Page
urlToPage url =
    let
        pageParser =
            UrlParser.oneOf
                [ UrlParser.map Home UrlParser.top
                , UrlParser.map Import (UrlParser.s "importar")
                , UrlParser.map Position (UrlParser.s "posicao")
                ]
    in
    UrlParser.parse pageParser url
        |> Maybe.withDefault Unknown


pageToPath : Page -> String
pageToPath page =
    case page of
        Home ->
            "/"

        Import ->
            "/importar"

        Position ->
            "/posicao"

        Unknown ->
            "/:eyes:"



---- MODEL ----


type Page
    = Home
    | Import
    | Position
    | Unknown


type alias Model =
    { key : Key, page : Page }


init : () -> Url -> Key -> ( Model, Cmd Msg )
init () url key =
    ( { key = key, page = urlToPage url }, Cmd.none )



---- UPDATE ----


type Msg
    = UrlRequested UrlRequest
    | UrlChanged Url


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        UrlRequested (Internal url) ->
            ( model, pushUrl model.key (Url.toString url) )

        UrlRequested (External _) ->
            ( model, Cmd.none )

        UrlChanged url ->
            ( { model | page = urlToPage url }, Cmd.none )



---- VIEW ----


navbar : Page -> Html Msg
navbar page =
    let
        navLinkOptions =
            [ ( "Importar", Import )
            , ( "Posição", Position )
            ]

        navLink ( label, navPage ) =
            a
                [ class "py-3"
                , classList [ ( "font-semibold", page == navPage ) ]
                , href (pageToPath navPage)
                ]
                [ text label ]
    in
    div [ class "border-r w-40 px-2" ]
        [ a [ class "text-3xl font-medium h-16 text-center items-center flex", href (pageToPath Home) ]
            [ span [ class "text-indigo-600" ] [ text "Porti" ]
            , span [] [ text "folio" ]
            ]
        , div [ class "mt-6 flex flex-col font-light" ]
            (List.map navLink navLinkOptions)
        ]


header : Page -> Html Msg
header page =
    let
        pageTitle =
            case page of
                Home ->
                    Nothing

                Import ->
                    Just "Importar"

                Position ->
                    Just "Posição"

                Unknown ->
                    Nothing

        titleHeader title =
            [ div [ class "pl-4 text-2xl font-semibold border-l-2 border-indigo-600" ] [ text title ] ]
    in
    div
        [ class "border-b h-16 flex items-center" ]
        (pageTitle
            |> Maybe.map titleHeader
            |> Maybe.withDefault []
        )


content : Html Msg
content =
    div [ class "h-full" ] []


body : Model -> List (Html Msg)
body model =
    [ div [ class "font-sans h-full" ]
        [ div [ class "flex h-full" ]
            [ navbar model.page
            , div [ class "w-full" ]
                [ header model.page, content ]
            ]
        ]
    ]


view : Model -> Document Msg
view model =
    { title = "📒 📈 Portifolio"
    , body = body model
    }



---- PROGRAM ----


subs : Model -> Sub Msg
subs _ =
    Sub.none


main : Program () Model Msg
main =
    Browser.application
        { view = view
        , init = init
        , update = update
        , subscriptions = subs
        , onUrlRequest = UrlRequested
        , onUrlChange = UrlChanged
        }
