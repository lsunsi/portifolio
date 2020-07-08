module Format exposing (money)

import FormatNumber
import FormatNumber.Locales


money : Float -> String
money =
    FormatNumber.format
        { decimals = FormatNumber.Locales.Exact 2
        , thousandSeparator = "."
        , decimalSeparator = ","
        , negativePrefix = "-"
        , negativeSuffix = ""
        , positivePrefix = ""
        , positiveSuffix = ""
        , zeroPrefix = ""
        , zeroSuffix = ""
        }
