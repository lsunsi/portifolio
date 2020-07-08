module Api exposing (Assetable(..), HttpData(..), HttpResult, PortfolioPosition, httpResultToData, importTrades, portfolioPosition)

import File exposing (File)
import Http
import Iso8601
import Json.Decode as Dec
import Time exposing (Posix)
import Url.Builder as UrlBuilder


type alias HttpResult a =
    Result Http.Error a


type HttpData a
    = Unasked
    | Loading
    | Success a
    | Failure Http.Error


httpResultToData : HttpResult a -> HttpData a
httpResultToData result =
    case result of
        Ok data ->
            Success data

        Err err ->
            Failure err


url : List String -> String
url paths =
    UrlBuilder.crossOrigin "http://localhost:8000" paths []


importTrades : File -> (HttpResult Int -> msg) -> Cmd msg
importTrades file msg =
    Http.post
        { url = url [ "import-trades" ]
        , expect = Http.expectJson msg Dec.int
        , body = Http.fileBody file
        }


portfolioPosition : Int -> (HttpResult PortfolioPosition -> msg) -> Cmd msg
portfolioPosition id msg =
    Http.get
        { url = url [ "portfolio-position", String.fromInt id ]
        , expect = Http.expectJson msg portfolioPositionDecoder
        }


assetableDecoder : Dec.Decoder Assetable
assetableDecoder =
    Dec.andThen
        (\type_ ->
            case type_ of
                "Etf" ->
                    Dec.map Etf (Dec.field "c" Dec.string)

                "Treasury" ->
                    Dec.map Treasury (Dec.field "c" Iso8601.decoder)

                _ ->
                    Dec.fail ":("
        )
        (Dec.field "t" Dec.string)


assetPositionDecoder : Dec.Decoder AssetPosition
assetPositionDecoder =
    Dec.map4 AssetPosition
        (Dec.field "assetable" assetableDecoder)
        (Dec.field "quantity" Dec.float)
        (Dec.field "amount" Dec.float)
        (Dec.field "price" Dec.float)


portfolioPositionDecoder : Dec.Decoder PortfolioPosition
portfolioPositionDecoder =
    Dec.map2 PortfolioPosition
        (Dec.field "assets" (Dec.list assetPositionDecoder))
        (Dec.field "amount" Dec.float)


type Assetable
    = Treasury Posix
    | Etf String


type alias AssetPosition =
    { assetable : Assetable
    , quantity : Float
    , amount : Float
    , price : Float
    }


type alias PortfolioPosition =
    { assets : List AssetPosition
    , amount : Float
    }
