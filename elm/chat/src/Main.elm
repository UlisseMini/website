module Main exposing (Model, Msg(..), init, main, subscriptions, update, view)

import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http exposing (..)



-- MAIN


main =
    Browser.element
        { init = init
        , update = update
        , subscriptions = subscriptions
        , view = view
        }



-- MODEL


type alias Message =
    { sender : String, message : String }


type Status
    = Idle
    | Sending


type alias Model =
    { username : String
    , pastMessages : List Message
    , status : Status
    }


init : () -> ( Model, Cmd Msg )
init _ =
    ( Content "", Cmd.none )



-- UPDATE


type Msg
    = Change String
    | Submit
    | Response (Result Http.Error String)


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        _ ->
            ( model, Cmd.none )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none



-- VIEW


view : Model -> Html Msg
view model =
    div [ id "elm-chat" ]
        [ textarea [ placeholder "", value (content model), onInput Change, cols 40, rows 15 ] []
        , button [ onClick Submit ] [ text (buttonText model) ]
        ]
