module Main exposing (main)

import Api exposing (HttpData, HttpResult)
import Browser exposing (Document, UrlRequest(..))
import Browser.Navigation exposing (Key, pushUrl)
import File exposing (File)
import File.Select as FileSelect
import Format
import Html exposing (Html, a, button, div, h1, img, span, table, tbody, td, text, th, thead, tr)
import Html.Attributes exposing (class, classList, href)
import Html.Events exposing (onClick)
import Http
import Route exposing (Route)
import Url exposing (Url)
import Url.Parser as UrlParser exposing ((</>))


type Page
    = Home
    | Import (Maybe File)
    | Position Int (HttpData Api.PortfolioPosition)
    | Unknown


pageToRoute : Page -> Maybe Route
pageToRoute page =
    case page of
        Home ->
            Just Route.Home

        Import _ ->
            Just Route.Import

        Position _ _ ->
            Just Route.Position

        _ ->
            Nothing


type alias Model =
    { key : Key
    , page : Page
    , portfolioId : Maybe Int
    }


init : () -> Url -> Key -> ( Model, Cmd Msg )
init () url key =
    ( { key = key
      , page = Home
      , portfolioId = Nothing
      }
    , Cmd.none
    )


type Msg
    = UrlRequested UrlRequest
    | UrlChanged Url
    | ImportFileInputClicked
    | ImportFileSelected File
    | ImportFileSubmit File
    | ImportTradesResponded (HttpResult Int)
    | GotPortfolioPosition (HttpResult Api.PortfolioPosition)


loadRoute : Maybe Route -> Model -> ( Model, Cmd Msg )
loadRoute route model =
    case route of
        Just Route.Home ->
            ( { model | page = Home }, Cmd.none )

        Just Route.Position ->
            case model.portfolioId of
                Just id ->
                    ( { model | page = Position id Api.Loading }, Api.portfolioPosition id GotPortfolioPosition )

                Nothing ->
                    ( { model | page = Unknown }, Cmd.none )

        Just Route.Import ->
            ( { model | page = Import Nothing }, Cmd.none )

        Nothing ->
            ( { model | page = Unknown }, Cmd.none )


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        UrlRequested request ->
            case request of
                Internal url ->
                    ( model, pushUrl model.key (Url.toString url) )

                External _ ->
                    ( model, Cmd.none )

        UrlChanged url ->
            loadRoute (Route.fromUrl url) model

        ImportFileInputClicked ->
            ( model, FileSelect.file [ "text/csv" ] ImportFileSelected )

        ImportFileSelected file ->
            ( { model | page = Import (Just file) }, Cmd.none )

        ImportFileSubmit file ->
            ( model, Api.importTrades file ImportTradesResponded )

        ImportTradesResponded result ->
            case result of
                Ok id ->
                    ( { model | portfolioId = Just id }
                    , pushUrl model.key (Route.toPath Route.Position)
                    )

                Err _ ->
                    ( model, Cmd.none )

        GotPortfolioPosition result ->
            case model.page of
                Position id _ ->
                    ( { model | page = Position id (Api.httpResultToData result) }, Cmd.none )

                _ ->
                    ( model, Cmd.none )


navbar : Page -> Html Msg
navbar page =
    let
        navLinkOptions =
            [ ( "Posição", Route.Position )
            , ( "Importar", Route.Import )
            ]

        isRoute route =
            case pageToRoute page of
                Just r ->
                    r == route

                Nothing ->
                    False

        navLink ( label, route ) =
            a
                [ class "py-3"
                , classList [ ( "font-semibold", isRoute route ) ]
                , href (Route.toPath route)
                ]
                [ text label ]
    in
    div [ class "border-r w-40 px-2" ]
        [ a
            [ class "text-3xl font-medium h-16 text-center items-center flex"
            , href (Route.toPath Route.Home)
            ]
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

                Position _ _ ->
                    Just "Posição"

                Import _ ->
                    Just "Importar"

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


content : Page -> Html Msg
content page =
    case page of
        Import maybeFile ->
            case maybeFile of
                Nothing ->
                    div [ class "px-4 pt-8" ]
                        [ button [ onClick ImportFileInputClicked ] [ text "Selecionar arquivo" ]
                        ]

                Just file ->
                    div []
                        [ div [] [ text (File.name file) ]
                        , button [ onClick (ImportFileSubmit file) ] [ text "Importar" ]
                        ]

        Position _ (Api.Success { amount, assets }) ->
            div [ class "px-4 pt-8" ]
                [ table []
                    ([ tr []
                        [ td [ class "text-right text-3xl pb-6" ] [ text (Format.money amount) ]
                        , td [ class "text font-light pb-6" ] [ text "Portfolio" ]
                        ]
                     ]
                        ++ List.map
                            (\asset ->
                                tr []
                                    [ td [ class "text-right text-xl py-1" ] [ text (Format.money asset.amount) ]
                                    , td [ class "text-sm font-light py-1" ]
                                        [ text
                                            (case asset.assetable of
                                                Api.Treasury _ ->
                                                    "Tesouro SELIC"

                                                Api.Etf ticker ->
                                                    "ETF " ++ ticker
                                            )
                                        ]
                                    ]
                            )
                            assets
                    )
                ]

        _ ->
            div [] []


body : Model -> List (Html Msg)
body model =
    [ div [ class "font-sans h-full" ]
        [ div [ class "flex h-full" ]
            [ navbar model.page
            , div [ class "w-full" ]
                [ header model.page
                , div [ class "h-full" ]
                    [ content model.page
                    ]
                ]
            ]
        ]
    ]


view : Model -> Document Msg
view model =
    { title = "📒 📈 Portifolio"
    , body = body model
    }


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
