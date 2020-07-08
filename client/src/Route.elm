module Route exposing (Route(..), fromUrl, toPath)

import Url exposing (Url)
import Url.Parser as Parser exposing ((</>))


type Route
    = Home
    | Position
    | Import


type alias Paths =
    { home : String, position : String, import_ : String }


fromUrl : Url -> Maybe Route
fromUrl url =
    let
        pageParser =
            Parser.oneOf
                [ Parser.map Home Parser.top
                , Parser.map Position (Parser.s "posicao")
                , Parser.map Import (Parser.s "importar")
                ]
    in
    Parser.parse pageParser url


toPath : Route -> String
toPath route =
    case route of
        Home ->
            "/"

        Position ->
            "/posicao"

        Import ->
            "/importar"
