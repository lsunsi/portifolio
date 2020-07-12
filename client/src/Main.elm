module Main exposing (main)

import Api exposing (HttpData, HttpResult)
import Browser exposing (Document, UrlRequest(..))
import Browser.Navigation exposing (Key, pushUrl)
import File exposing (File)
import File.Select as FileSelect
import Format
import Html exposing (..)
import Html.Attributes exposing (attribute, class, classList, href, style, type_)
import Html.Events exposing (onClick)
import Http
import Ports
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


init : Maybe Int -> Url -> Key -> ( Model, Cmd Msg )
init portfolioId url key =
    loadRoute (Route.fromUrl url) { key = key, page = Home, portfolioId = portfolioId }


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
                    ( { model | page = Home }, pushUrl model.key (Route.toPath Route.Home) )

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
                    , Cmd.batch
                        [ pushUrl model.key (Route.toPath Route.Position)
                        , Ports.storePortfolioId (Just id)
                        ]
                    )

                Err _ ->
                    ( model, Cmd.none )

        GotPortfolioPosition result ->
            case model.page of
                Position id _ ->
                    ( { model | page = Position id (Api.httpResultToData result) }, Cmd.none )

                _ ->
                    ( model, Cmd.none )


sidebar : Page -> List (Html Msg)
sidebar page =
    let
        navOptions =
            [ ( "Posição", Route.Position )
            , ( "Importar", Route.Import )
            ]

        isRoute route =
            case pageToRoute page of
                Just r ->
                    r == route

                Nothing ->
                    False
    in
    [ h1 [ class "uk-margin-left" ]
        [ span [] [ text "Porti" ]
        , span [ class "uk-text-primary" ] [ text "folio" ]
        ]
    , ul [ class "uk-nav uk-nav-primary uk-align-right" ]
        (List.map
            (\( label, route ) ->
                li [ classList [ ( "uk-active", isRoute route ) ] ]
                    [ a [ href (Route.toPath route) ] [ text label ] ]
            )
            navOptions
        )
    ]


contentTitle : String -> Html Msg
contentTitle title =
    h1 [ class "uk-heading-line" ] [ span [] [ text title ] ]


importView : Maybe File -> List (Html Msg)
importView maybeFile =
    case maybeFile of
        Just file ->
            [ contentTitle "Importar"
            , div [ class "uk-button-group" ]
                [ button
                    [ class "uk-button uk-button-primary", onClick (ImportFileSubmit file) ]
                    [ text "Importar" ]
                , button
                    [ class "uk-button uk-button-default", onClick ImportFileInputClicked ]
                    [ text "Re-selecionar" ]
                ]
            ]

        Nothing ->
            [ contentTitle "Importar"
            , button
                [ class "uk-button uk-button-primary", onClick ImportFileInputClicked ]
                [ text "Selecionar" ]
            ]


positionView : HttpData Api.PortfolioPosition -> List (Html Msg)
positionView httpPosition =
    case httpPosition of
        Api.Success { amount, assets } ->
            [ contentTitle "Posição"
            , div [ class "uk-card" ]
                [ div [ class "uk-card-title" ] [ text "Total" ]
                , div [ class "uk-card-body" ] [ text (Format.money amount) ]
                ]
            , div [ class "uk-card" ]
                [ div [ class "uk-card-title" ] [ text "Por Ativo" ]
                , table [ class "uk-card-body uk-table" ]
                    (List.map
                        (\asset ->
                            tr []
                                [ td []
                                    [ text
                                        (case asset.assetable of
                                            Api.Treasury _ ->
                                                "Tesouro SELIC"

                                            Api.Etf ticker ->
                                                "ETF " ++ ticker
                                        )
                                    ]
                                , td [] [ text (Format.money asset.amount) ]
                                ]
                        )
                        assets
                    )
                ]
            ]

        Api.Loading ->
            [ contentTitle "Posição", div [ attribute "uk-pinner" "true" ] [] ]

        _ ->
            [ contentTitle "Posição", div [ attribute "uk-pinner" "true" ] [] ]


content : Page -> List (Html Msg)
content page =
    case page of
        Import maybeFile ->
            importView maybeFile

        Position _ position ->
            positionView position

        _ ->
            []


body : Model -> List (Html Msg)
body model =
    [ div [ class "uk-flex uk-height-1-1 uk-flex-right uk-padding" ]
        [ div [ class "uk-flex-1" ] (content model.page)
        , div [ class "uk-flex-none" ] (sidebar model.page)
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


main : Program (Maybe Int) Model Msg
main =
    Browser.application
        { view = view
        , init = init
        , update = update
        , subscriptions = subs
        , onUrlRequest = UrlRequested
        , onUrlChange = UrlChanged
        }
