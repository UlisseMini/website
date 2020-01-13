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


type Model
    = Content String
    | Submitting
    | Submitted (Result Http.Error String)


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
        Change str ->
            ( Content str, Cmd.none )

        Submit ->
            ( model
            , Http.post
                { url = "/message"
                , body = Http.stringBody "text/plain" (content model)
                , expect = Http.expectString Response
                }
            )

        Response result ->
            ( Submitted result, Cmd.none )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none



-- VIEW


view : Model -> Html Msg
view model =
    div [ id "elm-msgbox" ]
        [ textarea [ placeholder "Or leave a message here", value (content model), onInput Change, cols 40, rows 15 ] []
        , button [ onClick Submit ] [ text (buttonText model) ]
        ]


buttonText : Model -> String
buttonText model =
    case model of
        Content _ ->
            "Submit"

        Submitting ->
            "Submitting"

        Submitted result ->
            case result of
                Err e ->
                    httpError e

                -- The backend provides the button status in this case
                Ok status ->
                    status


httpError : Http.Error -> String
httpError e =
    case e of
        Timeout ->
            "Timed out"

        BadUrl s ->
            "Bad URL " ++ s

        NetworkError ->
            "Network Error"

        BadStatus i ->
            "Bad Status " ++ String.fromInt i

        BadBody s ->
            "Bad Body " ++ s


content : Model -> String
content model =
    case model of
        Content str ->
            str

        _ ->
            ""
