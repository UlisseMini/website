module Main exposing (..)

import Browser
import Html exposing (Html, Attribute, div, input, text)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput)



-- MAIN


main =
  Browser.sandbox { init = init, update = update, view = view }



-- MODEL


type Model =
  HomePage


init : Model
init =
  HomePage


-- UPDATE


type Msg
  = None


update : Msg -> Model -> Model
update msg model =
  HomePage


-- VIEW


view : Model -> Html Msg
view model =
  div []
    [
    ]
