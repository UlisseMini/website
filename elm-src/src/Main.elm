module Main exposing (Model(..), Msg(..), init, main, update, view)

import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput)


title =
    "Uli's Blog"



-- MAIN


main : Program {} Model Msg
main =
    Browser.document
        { init = init
        , update = update
        , view = \model -> { title = title, body = [ view model ] }
        , subscriptions = subscriptions
        }



-- MODEL


type Model
    = HomePage


init : flags -> ( Model, Cmd Msg )
init _ =
    ( HomePage, Cmd.none )



-- UPDATE


type Msg
    = None


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    ( HomePage, Cmd.none )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub msg
subscriptions model =
    Sub.none



-- VIEW


view : Model -> Html Msg
view model =
    div []
        [ h1 [] [ text title ]
        ]



-- POSTS


type alias Post =
    { title : String
    , content : String -- TODO: Make content markdown.
    }



-- TODO: don't hardcode, store serverside and get via http request


posts : List Post
posts =
    [ { title = "First post!"
      , content = "This is the content of the post!"
      }
    ]
