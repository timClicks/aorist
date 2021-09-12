from aorist import (
    Attribute,
    NaturalNumber,
    StringIdentifier,
    DateString,
    POSIXTimestamp,
    PositiveFloat,
    default_tabular_schema,
    RowStruct,
    StaticDataTable,
    DataSchema,
    StorageSetup,
    RemoteStorageSetup,
    Storage,
    RemoteStorage,
    RemoteLocation,
    CSVEncoding,
    Encoding,
    DataSet,
    DatumTemplate,
    Asset,
    WebLocation,
    FileBasedStorageLayout,
    CSVHeader,
    FileHeader,
    APIOrFileLayout,
    SingleFileLayout,
)

attributes2020 = (2020,2020,[
    Attribute(StringIdentifier("Div")),
    Attribute(DateString("Date")),
    Attribute(POSIXTimestamp("Time")),
    Attribute(StringIdentifier("HomeTeam")),
    Attribute(StringIdentifier("AwayTeam")),
    Attribute(NaturalNumber("FTHG")),
    Attribute(NaturalNumber("FTAG")),
    Attribute(StringIdentifier("FTR")),
    Attribute(NaturalNumber("HTHG")),
    Attribute(NaturalNumber("HTAG")),
    Attribute(StringIdentifier("HTR")),
    Attribute(NaturalNumber("HS")),
    Attribute(NaturalNumber("AS")),
    Attribute(NaturalNumber("HST")),
    Attribute(NaturalNumber("AST")),
    Attribute(NaturalNumber("HF")),
    Attribute(NaturalNumber("AF")),
    Attribute(NaturalNumber("HC")),
    Attribute(NaturalNumber("AC")),
    Attribute(NaturalNumber("HY")),
    Attribute(NaturalNumber("AY")),
    Attribute(NaturalNumber("HR")),
    Attribute(NaturalNumber("AR")),
    Attribute(PositiveFloat("B365H")),
    Attribute(PositiveFloat("B365D")),
    Attribute(PositiveFloat("B365A")),
    Attribute(PositiveFloat("BWH")),
    Attribute(PositiveFloat("BWD")),
    Attribute(PositiveFloat("BWA")),
    Attribute(PositiveFloat("IWH")),
    Attribute(PositiveFloat("IWD")),
    Attribute(PositiveFloat("IWA")),
    Attribute(PositiveFloat("PSH")),
    Attribute(PositiveFloat("PSD")),
    Attribute(PositiveFloat("PSA")),
    Attribute(PositiveFloat("WHH")),
    Attribute(PositiveFloat("WHD")),
    Attribute(PositiveFloat("WHA")),
    Attribute(PositiveFloat("VCH")),
    Attribute(PositiveFloat("VCD")),
    Attribute(PositiveFloat("VCA")),
    Attribute(PositiveFloat("MaxH")),
    Attribute(PositiveFloat("MaxD")),
    Attribute(PositiveFloat("MaxA")),
    Attribute(PositiveFloat("AvgH")),
    Attribute(PositiveFloat("AvgD")),
    Attribute(PositiveFloat("AvgA")),
    Attribute(PositiveFloat("B365>2.5")),
    Attribute(PositiveFloat("B365<2.5")),
    Attribute(PositiveFloat("P>2.5")),
    Attribute(PositiveFloat("P<2.5")),
    Attribute(PositiveFloat("Max>2.5")),
    Attribute(PositiveFloat("Max<2.5")),
    Attribute(PositiveFloat("Avg>2.5")),
    Attribute(PositiveFloat("Avg<2.5")),
    Attribute(PositiveFloat("AHh")),
    Attribute(PositiveFloat("B365AHH")),
    Attribute(PositiveFloat("B365AHA")),
    Attribute(PositiveFloat("PAHH")),
    Attribute(PositiveFloat("PAHA")),
    Attribute(PositiveFloat("MaxAHH")),
    Attribute(PositiveFloat("MaxAHA")),
    Attribute(PositiveFloat("AvgAHH")),
    Attribute(PositiveFloat("AvgAHA")),
    Attribute(PositiveFloat("B365CH")),
    Attribute(PositiveFloat("B365CD")),
    Attribute(PositiveFloat("B365CA")),
    Attribute(PositiveFloat("BWCH")),
    Attribute(PositiveFloat("BWCD")),
    Attribute(PositiveFloat("BWCA")),
    Attribute(PositiveFloat("IWCH")),
    Attribute(PositiveFloat("IWCD")),
    Attribute(PositiveFloat("IWCA")),
    Attribute(PositiveFloat("PSCH")),
    Attribute(PositiveFloat("PSCD")),
    Attribute(PositiveFloat("PSCA")),
    Attribute(PositiveFloat("WHCH")),
    Attribute(PositiveFloat("WHCD")),
    Attribute(PositiveFloat("WHCA")),
    Attribute(PositiveFloat("VCCH")),
    Attribute(PositiveFloat("VCCD")),
    Attribute(PositiveFloat("VCCA")),
    Attribute(PositiveFloat("MaxCH")),
    Attribute(PositiveFloat("MaxCD")),
    Attribute(PositiveFloat("MaxCA")),
    Attribute(PositiveFloat("AvgCH")),
    Attribute(PositiveFloat("AvgCD")),
    Attribute(PositiveFloat("AvgCA")),
    Attribute(PositiveFloat("B365C>2.5")),
    Attribute(PositiveFloat("B365C<2.5")),
    Attribute(PositiveFloat("PC>2.5")),
    Attribute(PositiveFloat("PC<2.5")),
    Attribute(PositiveFloat("MaxC>2.5")),
    Attribute(PositiveFloat("MaxC<2.5")),
    Attribute(PositiveFloat("AvgC>2.5")),
    Attribute(PositiveFloat("AvgC<2.5")),
    Attribute(PositiveFloat("AHCh")),
    Attribute(PositiveFloat("B365CAHH")),
    Attribute(PositiveFloat("B365CAHA")),
    Attribute(PositiveFloat("PCAHH")),
    Attribute(PositiveFloat("PCAHA")),
    Attribute(PositiveFloat("MaxCAHH")),
    Attribute(PositiveFloat("MaxCAHA")),
    Attribute(PositiveFloat("AvgCAHH")),
    Attribute(PositiveFloat("AvgCAHA")),
])


attributes2019 = (2019,2019,[
    Attribute(StringIdentifier("Div")),
    Attribute(DateString("Date")),
    Attribute(StringIdentifier("HomeTeam")),
    Attribute(StringIdentifier("AwayTeam")),
    Attribute(NaturalNumber("FTHG")),
    Attribute(NaturalNumber("FTAG")),
    Attribute(StringIdentifier("FTR")),
    Attribute(NaturalNumber("HTHG")),
    Attribute(NaturalNumber("HTAG")),
    Attribute(StringIdentifier("HTR")),
    Attribute(NaturalNumber("HS")),
    Attribute(NaturalNumber("AS")),
    Attribute(NaturalNumber("HST")),
    Attribute(NaturalNumber("AST")),
    Attribute(NaturalNumber("HF")),
    Attribute(NaturalNumber("AF")),
    Attribute(NaturalNumber("HC")),
    Attribute(NaturalNumber("AC")),
    Attribute(NaturalNumber("HY")),
    Attribute(NaturalNumber("AY")),
    Attribute(NaturalNumber("HR")),
    Attribute(NaturalNumber("AR")),
    Attribute(PositiveFloat("B365H")),
    Attribute(PositiveFloat("B365D")),
    Attribute(PositiveFloat("B365A")),
    Attribute(PositiveFloat("BWH")),
    Attribute(PositiveFloat("BWD")),
    Attribute(PositiveFloat("BWA")),
    Attribute(PositiveFloat("IWH")),
    Attribute(PositiveFloat("IWD")),
    Attribute(PositiveFloat("IWA")),
    Attribute(PositiveFloat("PSH")),
    Attribute(PositiveFloat("PSD")),
    Attribute(PositiveFloat("PSA")),
    Attribute(PositiveFloat("WHH")),
    Attribute(PositiveFloat("WHD")),
    Attribute(PositiveFloat("WHA")),
    Attribute(PositiveFloat("VCH")),
    Attribute(PositiveFloat("VCD")),
    Attribute(PositiveFloat("VCA")),
    Attribute(NaturalNumber("Bb1X2")),
    Attribute(PositiveFloat("BbMxH")),
    Attribute(PositiveFloat("BbAvH")),
    Attribute(PositiveFloat("BbMxD")),
    Attribute(PositiveFloat("BbAvD")),
    Attribute(PositiveFloat("BbMxA")),
    Attribute(PositiveFloat("BbAvA")),
    Attribute(NaturalNumber("BbOU")),
    Attribute(PositiveFloat("BbMx>2.5")),
    Attribute(PositiveFloat("BbAv>2.5")),
    Attribute(PositiveFloat("BbMx<2.5")),
    Attribute(PositiveFloat("BbAv<2.5")),
    Attribute(NaturalNumber("BbAH")),
    Attribute(PositiveFloat("BbAHh")),
    Attribute(PositiveFloat("BbMxAHH")), 
    Attribute(PositiveFloat("BbAvAHH")), 
    Attribute(PositiveFloat("BbMxAHA")), 
    Attribute(PositiveFloat("BbAvAHA")), 
    Attribute(PositiveFloat("PSCH")), 
    Attribute(PositiveFloat("PSCD")), 
    Attribute(PositiveFloat("PSCA")), 
])

attributes2018 = (2016,2018,[
    Attribute(StringIdentifier("Div")),
    Attribute(DateString("Date")),
    Attribute(StringIdentifier("HomeTeam")),
    Attribute(StringIdentifier("AwayTeam")),
    Attribute(NaturalNumber("FTHG")),
    Attribute(NaturalNumber("FTAG")),
    Attribute(StringIdentifier("FTR")),
    Attribute(NaturalNumber("HTHG")),
    Attribute(NaturalNumber("HTAG")),
    Attribute(StringIdentifier("HTR")),
    Attribute(NaturalNumber("HS")),
    Attribute(NaturalNumber("AS")),
    Attribute(NaturalNumber("HST")),
    Attribute(NaturalNumber("AST")),
    Attribute(NaturalNumber("HF")),
    Attribute(NaturalNumber("AF")),
    Attribute(NaturalNumber("HC")),
    Attribute(NaturalNumber("AC")),
    Attribute(NaturalNumber("HY")),
    Attribute(NaturalNumber("AY")),
    Attribute(NaturalNumber("HR")),
    Attribute(NaturalNumber("AR")),
    Attribute(PositiveFloat("B365H")),
    Attribute(PositiveFloat("B365D")),
    Attribute(PositiveFloat("B365A")),
    Attribute(PositiveFloat("BWH")),
    Attribute(PositiveFloat("BWD")),
    Attribute(PositiveFloat("BWA")),
    Attribute(PositiveFloat("IWH")),
    Attribute(PositiveFloat("IWD")),
    Attribute(PositiveFloat("IWA")),
    Attribute(PositiveFloat("LBH")),
    Attribute(PositiveFloat("LBD")),
    Attribute(PositiveFloat("LBA")),
    Attribute(PositiveFloat("PSH")),
    Attribute(PositiveFloat("PSD")),
    Attribute(PositiveFloat("PSA")),
    Attribute(PositiveFloat("WHH")),
    Attribute(PositiveFloat("WHD")),
    Attribute(PositiveFloat("WHA")),
    Attribute(PositiveFloat("VCH")),
    Attribute(PositiveFloat("VCD")),
    Attribute(PositiveFloat("VCA")),
    Attribute(NaturalNumber("Bb1X2")),
    Attribute(PositiveFloat("BbMxH")),
    Attribute(PositiveFloat("BbAvH")),
    Attribute(PositiveFloat("BbMxD")),
    Attribute(PositiveFloat("BbAvD")),
    Attribute(PositiveFloat("BbMxA")),
    Attribute(PositiveFloat("BbAvA")),
    Attribute(NaturalNumber("BbOU")),
    Attribute(PositiveFloat("BbMx>2.5")),
    Attribute(PositiveFloat("BbAv>2.5")),
    Attribute(PositiveFloat("BbMx<2.5")),
    Attribute(PositiveFloat("BbAv<2.5")),
    Attribute(NaturalNumber("BbAH")),
    Attribute(PositiveFloat("BbAHh")),
    Attribute(PositiveFloat("BbMxAHH")),
    Attribute(PositiveFloat("BbAvAHH")),
    Attribute(PositiveFloat("BbMxAHA")),
    Attribute(PositiveFloat("BbAvAHA")),
    Attribute(PositiveFloat("PSCH")),
    Attribute(PositiveFloat("PSCD")),
    Attribute(PositiveFloat("PSCA")),
])

attributes2015 = (2014,2015,[
    Attribute(StringIdentifier("Div")),
    Attribute(DateString("Date")),
    Attribute(StringIdentifier("HomeTeam")),
    Attribute(StringIdentifier("AwayTeam")),
    Attribute(NaturalNumber("FTHG")),
    Attribute(NaturalNumber("FTAG")),
    Attribute(StringIdentifier("FTR")),
    Attribute(NaturalNumber("HTHG")),
    Attribute(NaturalNumber("HTAG")),
    Attribute(StringIdentifier("HTR")),
    Attribute(NaturalNumber("HS")),
    Attribute(NaturalNumber("AS")),
    Attribute(NaturalNumber("HST")),
    Attribute(NaturalNumber("AST")),
    Attribute(NaturalNumber("HF")),
    Attribute(NaturalNumber("AF")),
    Attribute(NaturalNumber("HC")),
    Attribute(NaturalNumber("AC")),
    Attribute(NaturalNumber("HY")),
    Attribute(NaturalNumber("AY")),
    Attribute(NaturalNumber("HR")),
    Attribute(NaturalNumber("AR")),
    Attribute(PositiveFloat("B365H")),
    Attribute(PositiveFloat("B365D")),
    Attribute(PositiveFloat("B365A")),
    Attribute(PositiveFloat("BWH")),
    Attribute(PositiveFloat("BWD")),
    Attribute(PositiveFloat("BWA")),
    Attribute(PositiveFloat("IWH")),
    Attribute(PositiveFloat("IWD")),
    Attribute(PositiveFloat("IWA")),
    Attribute(PositiveFloat("LBH")),
    Attribute(PositiveFloat("LBD")),
    Attribute(PositiveFloat("LBA")),
    Attribute(PositiveFloat("PSH")),
    Attribute(PositiveFloat("PSD")),
    Attribute(PositiveFloat("PSA")),
    Attribute(PositiveFloat("WHH")),
    Attribute(PositiveFloat("WHD")),
    Attribute(PositiveFloat("WHA")),
    Attribute(PositiveFloat("SJH")),
    Attribute(PositiveFloat("SJD")),
    Attribute(PositiveFloat("SJA")),
    Attribute(PositiveFloat("VCH")),
    Attribute(PositiveFloat("VCD")),
    Attribute(PositiveFloat("VCA")),
    Attribute(NaturalNumber("Bb1X2")),
    Attribute(PositiveFloat("BbMxH")),
    Attribute(PositiveFloat("BbAvH")),
    Attribute(PositiveFloat("BbMxD")),
    Attribute(PositiveFloat("BbAvD")),
    Attribute(PositiveFloat("BbMxA")),
    Attribute(PositiveFloat("BbAvA")),
    Attribute(NaturalNumber("BbOU")),
    Attribute(PositiveFloat("BbMx>2.5")),
    Attribute(PositiveFloat("BbAv>2.5")),
    Attribute(PositiveFloat("BbMx<2.5")),
    Attribute(PositiveFloat("BbAv<2.5")),
    Attribute(NaturalNumber("BbAH")),
    Attribute(PositiveFloat("BbAHh")),
    Attribute(PositiveFloat("BbMxAHH")),
    Attribute(PositiveFloat("BbAvAHH")),
    Attribute(PositiveFloat("BbMxAHA")),
    Attribute(PositiveFloat("BbAvAHA")),
    Attribute(PositiveFloat("PSCH")),
    Attribute(PositiveFloat("PSCD")),
    Attribute(PositiveFloat("PSCA")),
])

attributes2013 = (2013,2013,[
    Attribute(StringIdentifier("Div")),
    Attribute(DateString("Date")),
    Attribute(StringIdentifier("HomeTeam")),
    Attribute(StringIdentifier("AwayTeam")),
    Attribute(NaturalNumber("FTHG")),
    Attribute(NaturalNumber("FTAG")),
    Attribute(StringIdentifier("FTR")),
    Attribute(NaturalNumber("HTHG")),
    Attribute(NaturalNumber("HTAG")),
    Attribute(StringIdentifier("HTR")),
    Attribute(NaturalNumber("HS")),
    Attribute(NaturalNumber("AS")),
    Attribute(NaturalNumber("HST")),
    Attribute(NaturalNumber("AST")),
    Attribute(NaturalNumber("HF")),
    Attribute(NaturalNumber("AF")),
    Attribute(NaturalNumber("HC")),
    Attribute(NaturalNumber("AC")),
    Attribute(NaturalNumber("HY")),
    Attribute(NaturalNumber("AY")),
    Attribute(NaturalNumber("HR")),
    Attribute(NaturalNumber("AR")),
    Attribute(PositiveFloat("B365H")),
    Attribute(PositiveFloat("B365D")),
    Attribute(PositiveFloat("B365A")),
    Attribute(PositiveFloat("BWH")),
    Attribute(PositiveFloat("BWD")),
    Attribute(PositiveFloat("BWA")),
    Attribute(PositiveFloat("GBH")),
    Attribute(PositiveFloat("GBD")),
    Attribute(PositiveFloat("GBA")),
    Attribute(PositiveFloat("IWH")),
    Attribute(PositiveFloat("IWD")),
    Attribute(PositiveFloat("IWA")),
    Attribute(PositiveFloat("LBH")),
    Attribute(PositiveFloat("LBD")),
    Attribute(PositiveFloat("LBA")),
    Attribute(PositiveFloat("PSH")),
    Attribute(PositiveFloat("PSD")),
    Attribute(PositiveFloat("PSA")),
    Attribute(PositiveFloat("WHH")),
    Attribute(PositiveFloat("WHD")),
    Attribute(PositiveFloat("WHA")),
    Attribute(PositiveFloat("SJH")),
    Attribute(PositiveFloat("SJD")),
    Attribute(PositiveFloat("SJA")),
    Attribute(PositiveFloat("VCH")),
    Attribute(PositiveFloat("VCD")),
    Attribute(PositiveFloat("VCA")),
    Attribute(PositiveFloat("BSH")),
    Attribute(PositiveFloat("BSD")),
    Attribute(PositiveFloat("BSA")),
    Attribute(NaturalNumber("Bb1X2")),
    Attribute(PositiveFloat("BbMxH")),
    Attribute(PositiveFloat("BbAvH")),
    Attribute(PositiveFloat("BbMxD")),
    Attribute(PositiveFloat("BbAvD")),
    Attribute(PositiveFloat("BbMxA")),
    Attribute(PositiveFloat("BbAvA")),
    Attribute(NaturalNumber("BbOU")),
    Attribute(PositiveFloat("BbMx>2.5")),
    Attribute(PositiveFloat("BbAv>2.5")),
    Attribute(PositiveFloat("BbMx<2.5")),
    Attribute(PositiveFloat("BbAv<2.5")),
    Attribute(NaturalNumber("BbAH")),
    Attribute(PositiveFloat("BbAHh")),
    Attribute(PositiveFloat("BbMxAHH")),
    Attribute(PositiveFloat("BbAvAHH")),
    Attribute(PositiveFloat("BbMxAHA")),
    Attribute(PositiveFloat("BbAvAHA")),
    Attribute(PositiveFloat("PSCH")),
    Attribute(PositiveFloat("PSCD")),
    Attribute(PositiveFloat("PSCA")),
])

attributes2012 = (2008,2012,[
    Attribute(StringIdentifier("Div")),
    Attribute(DateString("Date")),
    Attribute(StringIdentifier("HomeTeam")),
    Attribute(StringIdentifier("AwayTeam")),
    Attribute(NaturalNumber("FTHG")),
    Attribute(NaturalNumber("FTAG")),
    Attribute(StringIdentifier("FTR")),
    Attribute(NaturalNumber("HTHG")),
    Attribute(NaturalNumber("HTAG")),
    Attribute(StringIdentifier("HTR")),
    Attribute(NaturalNumber("HS")),
    Attribute(NaturalNumber("AS")),
    Attribute(NaturalNumber("HST")),
    Attribute(NaturalNumber("AST")),
    Attribute(NaturalNumber("HF")),
    Attribute(NaturalNumber("AF")),
    Attribute(NaturalNumber("HC")),
    Attribute(NaturalNumber("AC")),
    Attribute(NaturalNumber("HY")),
    Attribute(NaturalNumber("AY")),
    Attribute(NaturalNumber("HR")),
    Attribute(NaturalNumber("AR")),
    Attribute(PositiveFloat("B365H")),
    Attribute(PositiveFloat("B365D")),
    Attribute(PositiveFloat("B365A")),
    Attribute(PositiveFloat("BWH")),
    Attribute(PositiveFloat("BWD")),
    Attribute(PositiveFloat("BWA")),
    Attribute(PositiveFloat("GBH")),
    Attribute(PositiveFloat("GBD")),
    Attribute(PositiveFloat("GBA")),
    Attribute(PositiveFloat("IWH")),
    Attribute(PositiveFloat("IWD")),
    Attribute(PositiveFloat("IWA")),
    Attribute(PositiveFloat("LBH")),
    Attribute(PositiveFloat("LBD")),
    Attribute(PositiveFloat("LBA")),
    Attribute(PositiveFloat("SBH")),
    Attribute(PositiveFloat("SBD")),
    Attribute(PositiveFloat("SBA")),
    Attribute(PositiveFloat("WHH")),
    Attribute(PositiveFloat("WHD")),
    Attribute(PositiveFloat("WHA")),
    Attribute(PositiveFloat("SJH")),
    Attribute(PositiveFloat("SJD")),
    Attribute(PositiveFloat("SJA")),
    Attribute(PositiveFloat("VCH")),
    Attribute(PositiveFloat("VCD")),
    Attribute(PositiveFloat("VCA")),
    Attribute(PositiveFloat("BSH")),
    Attribute(PositiveFloat("BSD")),
    Attribute(PositiveFloat("BSA")),
    Attribute(NaturalNumber("Bb1X2")),
    Attribute(PositiveFloat("BbMxH")),
    Attribute(PositiveFloat("BbAvH")),
    Attribute(PositiveFloat("BbMxD")),
    Attribute(PositiveFloat("BbAvD")),
    Attribute(PositiveFloat("BbMxA")),
    Attribute(PositiveFloat("BbAvA")),
    Attribute(NaturalNumber("BbOU")),
    Attribute(PositiveFloat("BbMx>2.5")),
    Attribute(PositiveFloat("BbAv>2.5")),
    Attribute(PositiveFloat("BbMx<2.5")),
    Attribute(PositiveFloat("BbAv<2.5")),
    Attribute(NaturalNumber("BbAH")),
    Attribute(PositiveFloat("BbAHh")),
    Attribute(PositiveFloat("BbMxAHH")),
    Attribute(PositiveFloat("BbAvAHH")),
    Attribute(PositiveFloat("BbMxAHA")),
    Attribute(PositiveFloat("BbAvAHA")),
])

attributes2007 = (2007,2007,[
    Attribute(StringIdentifier("Div")),
    Attribute(DateString("Date")),
    Attribute(StringIdentifier("HomeTeam")),
    Attribute(StringIdentifier("AwayTeam")),
    Attribute(NaturalNumber("FTHG")),
    Attribute(NaturalNumber("FTAG")),
    Attribute(StringIdentifier("FTR")),
    Attribute(NaturalNumber("HTHG")),
    Attribute(NaturalNumber("HTAG")),
    Attribute(StringIdentifier("HTR")),
    Attribute(NaturalNumber("HS")),
    Attribute(NaturalNumber("AS")),
    Attribute(NaturalNumber("HST")),
    Attribute(NaturalNumber("AST")),
    Attribute(NaturalNumber("HF")),
    Attribute(NaturalNumber("AF")),
    Attribute(NaturalNumber("HC")),
    Attribute(NaturalNumber("AC")),
    Attribute(NaturalNumber("HY")),
    Attribute(NaturalNumber("AY")),
    Attribute(NaturalNumber("HR")),
    Attribute(NaturalNumber("AR")),
    Attribute(PositiveFloat("B365H")),
    Attribute(PositiveFloat("B365D")),
    Attribute(PositiveFloat("B365A")),
    Attribute(PositiveFloat("BWH")),
    Attribute(PositiveFloat("BWD")),
    Attribute(PositiveFloat("BWA")),
    Attribute(PositiveFloat("GBH")),
    Attribute(PositiveFloat("GBD")),
    Attribute(PositiveFloat("GBA")),
    Attribute(PositiveFloat("IWH")),
    Attribute(PositiveFloat("IWD")),
    Attribute(PositiveFloat("IWA")),
    Attribute(PositiveFloat("LBH")),
    Attribute(PositiveFloat("LBD")),
    Attribute(PositiveFloat("LBA")),
    Attribute(PositiveFloat("SBH")),
    Attribute(PositiveFloat("SBD")),
    Attribute(PositiveFloat("SBA")),
    Attribute(PositiveFloat("WHH")),
    Attribute(PositiveFloat("WHD")),
    Attribute(PositiveFloat("WHA")),
    Attribute(PositiveFloat("SJH")),
    Attribute(PositiveFloat("SJD")),
    Attribute(PositiveFloat("SJA")),
    Attribute(PositiveFloat("VCH")),
    Attribute(PositiveFloat("VCD")),
    Attribute(PositiveFloat("VCA")),
    Attribute(NaturalNumber("Bb1X2")),
    Attribute(PositiveFloat("BbMxH")),
    Attribute(PositiveFloat("BbAvH")),
    Attribute(PositiveFloat("BbMxD")),
    Attribute(PositiveFloat("BbAvD")),
    Attribute(PositiveFloat("BbMxA")),
    Attribute(PositiveFloat("BbAvA")),
    Attribute(NaturalNumber("BbOU")),
    Attribute(PositiveFloat("BbMx>2.5")),
    Attribute(PositiveFloat("BbAv>2.5")),
    Attribute(PositiveFloat("BbMx<2.5")),
    Attribute(PositiveFloat("BbAv<2.5")),
    Attribute(NaturalNumber("BbAH")),
    Attribute(PositiveFloat("BbAHh")),
    Attribute(PositiveFloat("BbMxAHH")),
    Attribute(PositiveFloat("BbAvAHH")),
    Attribute(PositiveFloat("BbMxAHA")),
    Attribute(PositiveFloat("BbAvAHA")),
])

attributes2006 = (2006,2006,[
    Attribute(StringIdentifier("Div")),
    Attribute(DateString("Date")),
    Attribute(StringIdentifier("HomeTeam")),
    Attribute(StringIdentifier("AwayTeam")),
    Attribute(NaturalNumber("FTHG")),
    Attribute(NaturalNumber("FTAG")),
    Attribute(StringIdentifier("FTR")),
    Attribute(NaturalNumber("HTHG")),
    Attribute(NaturalNumber("HTAG")),
    Attribute(StringIdentifier("HTR")),
    Attribute(NaturalNumber("HS")),
    Attribute(NaturalNumber("AS")),
    Attribute(NaturalNumber("HF")),
    Attribute(NaturalNumber("AF")),
    Attribute(NaturalNumber("HC")),
    Attribute(NaturalNumber("AC")),
    Attribute(NaturalNumber("HY")),
    Attribute(NaturalNumber("AY")),
    Attribute(NaturalNumber("HR")),
    Attribute(NaturalNumber("AR")),
    Attribute(PositiveFloat("B365H")),
    Attribute(PositiveFloat("B365D")),
    Attribute(PositiveFloat("B365A")),
    Attribute(PositiveFloat("BWH")),
    Attribute(PositiveFloat("BWD")),
    Attribute(PositiveFloat("BWA")),
    Attribute(PositiveFloat("GBH")),
    Attribute(PositiveFloat("GBD")),
    Attribute(PositiveFloat("GBA")),
    Attribute(PositiveFloat("IWH")),
    Attribute(PositiveFloat("IWD")),
    Attribute(PositiveFloat("IWA")),
    Attribute(PositiveFloat("LBH")),
    Attribute(PositiveFloat("LBD")),
    Attribute(PositiveFloat("LBA")),
    Attribute(PositiveFloat("SBH")),
    Attribute(PositiveFloat("SBD")),
    Attribute(PositiveFloat("SBA")),
    Attribute(PositiveFloat("WHH")),
    Attribute(PositiveFloat("WHD")),
    Attribute(PositiveFloat("WHA")),
    Attribute(PositiveFloat("SJH")),
    Attribute(PositiveFloat("SJD")),
    Attribute(PositiveFloat("SJA")),
    Attribute(PositiveFloat("VCH")),
    Attribute(PositiveFloat("VCD")),
    Attribute(PositiveFloat("VCA")),
    Attribute(NaturalNumber("Bb1X2")),
    Attribute(PositiveFloat("BbMxH")),
    Attribute(PositiveFloat("BbAvH")),
    Attribute(PositiveFloat("BbMxD")),
    Attribute(PositiveFloat("BbAvD")),
    Attribute(PositiveFloat("BbMxA")),
    Attribute(PositiveFloat("BbAvA")),
    Attribute(NaturalNumber("BbOU")),
    Attribute(PositiveFloat("BbMx>2.5")),
    Attribute(PositiveFloat("BbAv>2.5")),
    Attribute(PositiveFloat("BbMx<2.5")),
    Attribute(PositiveFloat("BbAv<2.5")),
    Attribute(NaturalNumber("BbAH")),
    Attribute(PositiveFloat("BbAHh")),
    Attribute(PositiveFloat("BbMxAHH")),
    Attribute(PositiveFloat("BbAvAHH")),
    Attribute(PositiveFloat("BbMxAHA")),
    Attribute(PositiveFloat("BbAvAHA")),
])

attributes2005 = (2005,2005,[
    Attribute(StringIdentifier("Div")),
    Attribute(DateString("Date")),
    Attribute(StringIdentifier("HomeTeam")),
    Attribute(StringIdentifier("AwayTeam")),
    Attribute(NaturalNumber("FTHG")),
    Attribute(NaturalNumber("FTAG")),
    Attribute(StringIdentifier("FTR")),
    Attribute(NaturalNumber("HTHG")),
    Attribute(NaturalNumber("HTAG")),
    Attribute(StringIdentifier("HTR")),
    Attribute(NaturalNumber("HS")),
    Attribute(NaturalNumber("AS")),
    Attribute(NaturalNumber("HF")),
    Attribute(NaturalNumber("AF")),
    Attribute(NaturalNumber("HC")),
    Attribute(NaturalNumber("AC")),
    Attribute(NaturalNumber("HY")),
    Attribute(NaturalNumber("AY")),
    Attribute(NaturalNumber("HR")),
    Attribute(NaturalNumber("AR")),
    Attribute(PositiveFloat("B365H")),
    Attribute(PositiveFloat("B365D")),
    Attribute(PositiveFloat("B365A")),
    Attribute(PositiveFloat("BWH")),
    Attribute(PositiveFloat("BWD")),
    Attribute(PositiveFloat("BWA")),
    Attribute(PositiveFloat("GBH")),
    Attribute(PositiveFloat("GBD")),
    Attribute(PositiveFloat("GBA")),
    Attribute(PositiveFloat("IWH")),
    Attribute(PositiveFloat("IWD")),
    Attribute(PositiveFloat("IWA")),
    Attribute(PositiveFloat("LBH")),
    Attribute(PositiveFloat("LBD")),
    Attribute(PositiveFloat("LBA")),
    Attribute(PositiveFloat("SBH")),
    Attribute(PositiveFloat("SBD")),
    Attribute(PositiveFloat("SBA")),
    Attribute(PositiveFloat("WHH")),
    Attribute(PositiveFloat("WHD")),
    Attribute(PositiveFloat("WHA")),
    Attribute(PositiveFloat("GB>2.5")),
    Attribute(PositiveFloat("GB<2.5")),
    Attribute(PositiveFloat("B365>2.5")),
    Attribute(PositiveFloat("B365<2.5")),
    Attribute(PositiveFloat("GBAHH")),
    Attribute(PositiveFloat("GBAHA")),
    Attribute(PositiveFloat("GBAH")),
    Attribute(PositiveFloat("LBAHH")),
    Attribute(PositiveFloat("LBAHA")),
    Attribute(PositiveFloat("LBAH")),
    Attribute(PositiveFloat("B365AHH")),
    Attribute(PositiveFloat("B365AHA")),
    Attribute(PositiveFloat("B365AH")),
])

attributes2004 = (2004,2004,[
    Attribute(StringIdentifier("Div")),
    Attribute(DateString("Date")),
    Attribute(StringIdentifier("HomeTeam")),
    Attribute(StringIdentifier("AwayTeam")),
    Attribute(NaturalNumber("FTHG")),
    Attribute(NaturalNumber("FTAG")),
    Attribute(StringIdentifier("FTR")),
    Attribute(NaturalNumber("HTHG")),
    Attribute(NaturalNumber("HTAG")),
    Attribute(StringIdentifier("HTR")),
    Attribute(NaturalNumber("HS")),
    Attribute(NaturalNumber("AS")),
    Attribute(NaturalNumber("HF")),
    Attribute(NaturalNumber("AF")),
    Attribute(NaturalNumber("HC")),
    Attribute(NaturalNumber("AC")),
    Attribute(NaturalNumber("HY")),
    Attribute(NaturalNumber("AY")),
    Attribute(NaturalNumber("HR")),
    Attribute(NaturalNumber("AR")),
    Attribute(PositiveFloat("B365H")),
    Attribute(PositiveFloat("B365D")),
    Attribute(PositiveFloat("B365A")),
    Attribute(PositiveFloat("GBH")),
    Attribute(PositiveFloat("GBD")),
    Attribute(PositiveFloat("GBA")),
    Attribute(PositiveFloat("IWH")),
    Attribute(PositiveFloat("IWD")),
    Attribute(PositiveFloat("IWA")),
    Attribute(PositiveFloat("LBH")),
    Attribute(PositiveFloat("LBD")),
    Attribute(PositiveFloat("LBA")),
    Attribute(PositiveFloat("SOH")),
    Attribute(PositiveFloat("SOD")),
    Attribute(PositiveFloat("SOA")),
    Attribute(PositiveFloat("SBH")),
    Attribute(PositiveFloat("SBD")),
    Attribute(PositiveFloat("SBA")),
    Attribute(PositiveFloat("WHH")),
    Attribute(PositiveFloat("WHD")),
    Attribute(PositiveFloat("WHA")),
    Attribute(PositiveFloat("GB>2.5")),
    Attribute(PositiveFloat("GB<2.5")),
    Attribute(PositiveFloat("B365>2.5")),
    Attribute(PositiveFloat("B365<2.5")),
    Attribute(PositiveFloat("GBAHH")),
    Attribute(PositiveFloat("GBAHA")),
    Attribute(PositiveFloat("GBAH")),
    Attribute(PositiveFloat("LBAHH")),
    Attribute(PositiveFloat("LBAHA")),
    Attribute(PositiveFloat("LBAH")),
    Attribute(PositiveFloat("B365AHH")),
    Attribute(PositiveFloat("B365AHA")),
    Attribute(PositiveFloat("B365AH")),
])

attributes2003 = (2003,2003,[
    Attribute(StringIdentifier("Div")),
    Attribute(DateString("Date")),
    Attribute(StringIdentifier("HomeTeam")),
    Attribute(StringIdentifier("AwayTeam")),
    Attribute(NaturalNumber("FTHG")),
    Attribute(NaturalNumber("FTAG")),
    Attribute(StringIdentifier("FTR")),
    Attribute(NaturalNumber("HTHG")),
    Attribute(NaturalNumber("HTAG")),
    Attribute(StringIdentifier("HTR")),
    Attribute(PositiveFloat("B365H")),
    Attribute(PositiveFloat("B365D")),
    Attribute(PositiveFloat("B365A")),
    Attribute(PositiveFloat("GBH")),
    Attribute(PositiveFloat("GBD")),
    Attribute(PositiveFloat("GBA")),
    Attribute(PositiveFloat("IWH")),
    Attribute(PositiveFloat("IWD")),
    Attribute(PositiveFloat("IWA")),
    Attribute(PositiveFloat("LBH")),
    Attribute(PositiveFloat("LBD")),
    Attribute(PositiveFloat("LBA")),
    Attribute(PositiveFloat("SOH")),
    Attribute(PositiveFloat("SOD")),
    Attribute(PositiveFloat("SOA")),
    Attribute(PositiveFloat("SBH")),
    Attribute(PositiveFloat("SBD")),
    Attribute(PositiveFloat("SBA")),
    Attribute(PositiveFloat("WHH")),
    Attribute(PositiveFloat("WHD")),
    Attribute(PositiveFloat("WHA")),
    Attribute(PositiveFloat("GB>2.5")),
    Attribute(PositiveFloat("GB<2.5")),
    Attribute(PositiveFloat("B365>2.5")),
    Attribute(PositiveFloat("B365<2.5")),
])

attributes2002 = (2002,2002,[
    Attribute(StringIdentifier("Div")),
    Attribute(DateString("Date")),
    Attribute(StringIdentifier("HomeTeam")),
    Attribute(StringIdentifier("AwayTeam")),
    Attribute(NaturalNumber("FTHG")),
    Attribute(NaturalNumber("FTAG")),
    Attribute(StringIdentifier("FTR")),
    Attribute(NaturalNumber("HTHG")),
    Attribute(NaturalNumber("HTAG")),
    Attribute(StringIdentifier("HTR")),
    Attribute(NaturalNumber("Attendance")),
    Attribute(StringIdentifier("Referee")),
    Attribute(NaturalNumber("HS")),
    Attribute(NaturalNumber("AS")),
    Attribute(NaturalNumber("HST")),
    Attribute(NaturalNumber("AST")),
    Attribute(NaturalNumber("HHW")),
    Attribute(NaturalNumber("AHW")),
    Attribute(NaturalNumber("HC")),
    Attribute(NaturalNumber("AC")),
    Attribute(NaturalNumber("HF")),
    Attribute(NaturalNumber("AF")),
    Attribute(NaturalNumber("HO")),
    Attribute(NaturalNumber("AO")),
    Attribute(NaturalNumber("HY")),
    Attribute(NaturalNumber("AY")),
    Attribute(NaturalNumber("HR")),
    Attribute(NaturalNumber("AR")),
    Attribute(NaturalNumber("HBP")),
    Attribute(NaturalNumber("ABP")),
    Attribute(PositiveFloat("GBH")),
    Attribute(PositiveFloat("GBD")),
    Attribute(PositiveFloat("GBA")),
    Attribute(PositiveFloat("IWH")),
    Attribute(PositiveFloat("IWD")),
    Attribute(PositiveFloat("IWA")),
    Attribute(PositiveFloat("LBH")),
    Attribute(PositiveFloat("LBD")),
    Attribute(PositiveFloat("LBA")),
    Attribute(PositiveFloat("SBH")),
    Attribute(PositiveFloat("SBD")),
    Attribute(PositiveFloat("SBA")),
    Attribute(PositiveFloat("SYH")),
    Attribute(PositiveFloat("SYD")),
    Attribute(PositiveFloat("SYA")),
    Attribute(PositiveFloat("WHH")),
    Attribute(PositiveFloat("WHD")),
    Attribute(PositiveFloat("WHA")),
])

attributes2001 = (2001,2001,[
    Attribute(StringIdentifier("Div"),
    Attribute(DateString("Date"),
    Attribute(StringIdentifier("HomeTeam"),
    Attribute(StringIdentifier("AwayTeam"),
    Attribute(NaturalNumber("FTHG"),
    Attribute(NaturalNumber("FTAG"),
    Attribute(StringIdentifier("FTR"),
    Attribute(NaturalNumber("HTHG"),
    Attribute(NaturalNumber("HTAG"),
    Attribute(StringIdentifier("HTR"),
    Attribute(NaturalNumber("Attendance"),
    Attribute(StringIdentifier("Referee"),
    Attribute(NaturalNumber("HS"),
    Attribute(NaturalNumber("AS"),
    Attribute(NaturalNumber("HST"),
    Attribute(NaturalNumber("AST"),
    Attribute(NaturalNumber("HHW"),
    Attribute(NaturalNumber("AHW"),
    Attribute(NaturalNumber("HC"),
    Attribute(NaturalNumber("AC"),
    Attribute(NaturalNumber("HF"),
    Attribute(NaturalNumber("AF"),
    Attribute(NaturalNumber("HO"),
    Attribute(NaturalNumber("AO"),
    Attribute(NaturalNumber("HY"),
    Attribute(NaturalNumber("AY"),
    Attribute(NaturalNumber("HR"),
    Attribute(NaturalNumber("AR"),
    Attribute(NaturalNumber("HBP"),
    Attribute(NaturalNumber("ABP"),
    Attribute(PositiveFloat("GBH"),
    Attribute(PositiveFloat("GBD"),
    Attribute(PositiveFloat("GBA"),
    Attribute(PositiveFloat("IWH"),
    Attribute(PositiveFloat("IWD"),
    Attribute(PositiveFloat("IWA"),
    Attribute(PositiveFloat("LBH"),
    Attribute(PositiveFloat("LBD"),
    Attribute(PositiveFloat("LBA"),
    Attribute(PositiveFloat("SBH"),
    Attribute(PositiveFloat("SBD"),
    Attribute(PositiveFloat("SBA"),
    Attribute(PositiveFloat("WHH"),
    Attribute(PositiveFloat("WHD"),
    Attribute(PositiveFloat("WHA"),
])

attributes2000 = (1996,2000,[
    Attribute(StringIdentifier("Div")),
    Attribute(DateString("Date")),
    Attribute(StringIdentifier("HomeTeam")),
    Attribute(StringIdentifier("AwayTeam")),
    Attribute(NaturalNumber("FTHG")),
    Attribute(NaturalNumber("FTAG")),
    Attribute(StringIdentifier("FTR")),
    Attribute(NaturalNumber("HTHG")),
    Attribute(NaturalNumber("HTAG")),
    Attribute(StringIdentifier("HTR")),
])

attributes1995 = (1994,1995,[
    Attribute(StringIdentifier("Div")),
    Attribute(DateString("Date")),
    Attribute(StringIdentifier("HomeTeam")),
    Attribute(StringIdentifier("AwayTeam")),
    Attribute(NaturalNumber("FTHG")),
    Attribute(NaturalNumber("FTAG")),
    Attribute(StringIdentifier("FTR")),
])

attr_list = [
    attributes1995,
    attributes2000,
    attributes2001,
    attributes2002,
    attributes2003,
    attributes2004,
    attributes2005,
    attributes2007,
    attributes2012,
    attributes2013,
    attributes2015,
    attributes2018,
    attributes2019,
    attributes2020,
]

templates = {}

for (start_year, end_year, attributes) in attr_list:
    key=(start_year, end_year)
    templates[key]=DatumTemplate(RowStruct(
        name="SSBundesliga_%d-%d_datum" % (start_year, end_year),
        attributes=attributes,
    ))


def build_assets(start_year, end_year, attributes):
    return {"SSBundesliga%d" % (x): Asset(StaticDataTable(
        name="SSBundesliga%d" % (x),
        schema=DataSchema(default_tabular_schema(
            templates[(start_year,end_year)],
            attributes=templates[(start_year,end_year)].attributes()
        )),
        setup=StorageSetup(RemoteStorageSetup(
            remote=Storage(RemoteStorage(
                location=RemoteLocation(WebLocation(
                    address=("https://sports-statistics.com/database/soccer-data"
                            "germany-bundesliga-1-%d-to-%d.csv" % (x-1,x)
                    ),
                )),
                layout=APIOrFileLayout(FileBasedStorageLayout(
                    SingleFileLayout()
                )),
                encoding=Encoding(CSVEncoding(header=FileHeader(
                    CSVHeader(num_lines=1
                    )
                ))),
            )),
        )),
        tag=str(x-1)+"-"+str(x)+"_ss_bundesliga",
        )) for x in range(start_year,end_year)}

assets1995 = build_assets(1994, 1995, attributes1995)
assets2000 = build_assets(1996, 2000, attributes2000)
assets2001 = build_assets(2001, 2001, attributes2001)
assets2002 = build_assets(2002, 2002, attributes2002)
assets2003 = build_assets(2003, 2003, attributes2003)
assets2004 = build_assets(2004, 2004, attributes2004)
assets2005 = build_assets(2005, 2005, attributes2005)
assets2007 = build_assets(2006, 2007, attributes2007)
assets2012 = build_assets(2008, 2012, attributes2012)
assets2013 = build_assets(2013, 2013, attributes2013)
assets2015 = build_assets(2014, 2015, attributes2015)
assets2018 = build_assets(2016, 2018, attributes2018)
assets2019 = build_assets(2029, 2019, attributes2019)
assets2020 = build_assets(2020, 2020, attributes2020)

assets = {**assets1995, **assets2000, **assets2001,
          **assets2002, **assets2003, **assets2004,
          **assets2005, **assets2007, **assets2012,
          **assets2013, **assets2015, **assets2018,
          **assets2019, **assets2020}

bundesliga_dataset = DataSet(
    name="bundesliga",
    description="""
        This dataset contains detailed match level statistics for the
         Bundesliga (Div 1) seasons 31 (1993/94) to 57 (2019/20).
    """,
    source_path=__file__,
    datumTemplates=list(templates.values()),
    assets=assets,
    access_policies=[]
)
