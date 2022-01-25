// Copyright (c) 2018 The openrtb-rust authors
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str::FromStr;

use phf::{phf_map};
use serde;

macro_rules! categories {
    ( $( $parent:ident $pval:expr     // parent category
         $(, $sub:ident $sval:expr )* // optional subcategories separated by comma
       );*                            // parents separated by semicolon
    )=> {
        // create enum for each parent category
        $(
            #[derive(Clone, Debug, PartialEq)]
            pub enum $parent {
                $parent,  // parent variant
                $($sub,)* // all subcategories
            }
         )*

        // wrap parent categories in one enum
        #[derive(Clone, Debug, PartialEq)]
        pub enum Category {
            $($parent($parent),)*
            Unknown(String),
        }

        impl Category {
            fn value(&self) -> String {
                match *self {
                    $(
                        // match parent category
                        Category::$parent(ref p) => {
                            match p {
                                $parent::$parent => $pval.to_string(),  // parent variant
                                $($parent::$sub => $sval.to_string(),)* // all subcategories
                            }
                        },
                      )*
                    Category::Unknown(ref s) => s.clone(),
                }
            }
        }

        static TO_CATEGORY: phf::Map<&'static str, Category> = phf_map! {
            $(
                $pval => Category::$parent($parent::$parent),  // parent variant
                $($sval => Category::$parent($parent::$sub),)* // all subcategories
            )*
        };
    }
}

// 5.1 Content Categories
categories! {

    ArtsAndEntertainment "IAB1",
    BooksAndLiterature "IAB1-1",
    CelebrityFanGossip "IAB1-2",
    FineArt "IAB1-3",
    Humor "IAB1-4",
    Movies "IAB1-5",
    Music "IAB1-6",
    Television "IAB1-7";

    Automotive "IAB2",
    AutoParts "IAB2-1",
    AutoRepair "IAB2-2",
    BuyingSellingCars "IAB2-3",
    CarCulture "IAB2-4",
    CertifiedPreOwned "IAB2-5",
    Convertible "IAB2-6",
    Coupe "IAB2-7",
    Crossover "IAB2-8",
    Diesel "IAB2-9",
    ElectricVehicle "IAB2-10",
    Hatchback "IAB2-11",
    Hybrid "IAB2-12",
    Luxury "IAB2-13",
    Minivan "IAB2-14",
    Motorcycles "IAB2-15",
    OffRoadVehicles "IAB2-16",
    PerformanceVehicles "IAB2-17",
    Pickup "IAB2-18",
    RoadSideAssistance "IAB2-19",
    Sedan "IAB2-20",
    TrucksAndAccessories "IAB2-21",
    VintageCars "IAB2-22",
    Wagon "IAB2-23";

    Business "IAB3",
    Advertising "IAB3-1",
    Agriculture "IAB3-2",
    BiotechBiomedical "IAB3-3",
    BusinessSoftware "IAB3-4",
    Construction "IAB3-5",
    Forestry "IAB3-6",
    Government "IAB3-7",
    GreenSolutions "IAB3-8",
    HumanResources "IAB3-9",
    Logistics "IAB3-10",
    Marketing "IAB3-11",
    Metals "IAB3-12";

    Careers "IAB4",
    CareerPlanning "IAB4-1",
    College "IAB4-2",
    FinancialAid "IAB4-3",
    JobFairs "IAB4-4",
    JobSearch "IAB4-5",
    ResumeWritingAdvice "IAB4-6",
    Nursing "IAB4-7",
    Scholarships "IAB4-8",
    Telecommuting "IAB4-9",
    USMilitary "IAB4-10",
    CareerAdvice "IAB4-11";

    Education "IAB5",
    SevenThruTwelveEducation "IAB5-1",
    AdultEducation "IAB5-2",
    ArtHistory "IAB5-3",
    CollegeAdministration "IAB5-4",
    CollegeLife "IAB5-5",
    DistanceLearning "IAB5-6",
    EnglishAsASecondLanguage "IAB5-7",
    LanguageLearning "IAB5-8",
    GraduateSchool "IAB5-9",
    Homeschooling "IAB5-10",
    HomeworkStudyTips "IAB5-11",
    KThruSixEducators "IAB5-12",
    PrivateSchool "IAB5-13",
    SpecialEducation "IAB5-14",
    StudyingBusiness "IAB5-15";

    FamilyAndParenting "IAB6",
    Adoption "IAB6-1",
    BabiesAndToddlers "IAB6-2",
    DaycarePreSchool "IAB6-3",
    FamilyInternet "IAB6-4",
    ParentingKThruSixKids "IAB6-5",
    ParentingTeens "IAB6-6",
    Pregnancy "IAB6-7",
    SpecialNeedsKids "IAB6-8",
    Eldercare "IAB6-9";

    HealthAndFitness "IAB7",
    Exercise "IAB7-1",
    ADD "IAB7-2",
    AIDSHIV "IAB7-3",
    Allergies "IAB7-4",
    AlternativeMedicine "IAB7-5",
    Arthritis "IAB7-6",
    Asthma "IAB7-7",
    AutismPDD "IAB7-8",
    BipolarDisorder "IAB7-9",
    BrainTumor "IAB7-10",
    Cancer "IAB7-11",
    Cholesterol "IAB7-12",
    ChronicFatigueSyndrome "IAB7-13",
    ChronicPain "IAB7-14",
    ColdAndFlu "IAB7-15",
    Deafness "IAB7-16",
    DentalCare "IAB7-17",
    Depression "IAB7-18",
    Dermatology "IAB7-19",
    Diabetes "IAB7-20",
    Epilepsy "IAB7-21",
    GERDAcidReflux "IAB7-22",
    HeadachesMigraines "IAB7-23",
    HeartDisease "IAB7-24",
    HerbsForHealth "IAB7-25",
    HolisticHealing "IAB7-26",
    IBSCrohnsDisease "IAB7-27",
    IncestAbuseSupport "IAB7-28",
    Incontinence "IAB7-29",
    Infertility "IAB7-30",
    MensHealth "IAB7-31",
    Nutrition "IAB7-32",
    Orthopedics "IAB7-33",
    PanicAnxietyDisorders "IAB7-34",
    Pediatrics "IAB7-35",
    PhysicalTherapy "IAB7-36",
    PsychologyPsychiatry "IAB7-37",
    SeniorHealth "IAB7-38",
    Sexuality "IAB7-39",
    SleepDisorders "IAB7-40",
    SmokingCessation "IAB7-41",
    SubstanceAbuse "IAB7-42",
    ThyroidDisease "IAB7-43",
    WeightLoss "IAB7-44",
    WomensHealth "IAB7-45";

    FoodAndDrink "IAB8",
    AmericanCuisine "IAB8-1",
    BarbecuesAndGrilling "IAB8-2",
    CajunCreole "IAB8-3",
    ChineseCuisine "IAB8-4",
    CocktailsBeer "IAB8-5",
    CoffeeTea "IAB8-6",
    CuisineSpecific "IAB8-7",
    DessertsAndBaking "IAB8-8",
    DiningOut "IAB8-9",
    FoodAllergies "IAB8-10",
    FrenchCuisine "IAB8-11",
    HealthLowFatCooking "IAB8-12",
    ItalianCuisine "IAB8-13",
    JapaneseCuisine "IAB8-14",
    MexicanCuisine "IAB8-15",
    Vegan "IAB8-16",
    Vegetarian "IAB8-17",
    Wine "IAB8-18";

    HobbiesAndInterests "IAB9",
    ArtTechnology "IAB9-1",
    ArtsAndCrafts "IAB9-2",
    Beadwork "IAB9-3",
    BirdWatching "IAB9-4",
    BoardGamesPuzzles "IAB9-5",
    CandleAndSoapMaking "IAB9-6",
    CardGames "IAB9-7",
    Chess "IAB9-8",
    Cigars "IAB9-9",
    Collecting "IAB9-10",
    ComicBooks "IAB9-11",
    DrawingSketching "IAB9-12",
    FreelanceWriting "IAB9-13",
    Genealogy "IAB9-14",
    GettingPublished "IAB9-15",
    Guitar "IAB9-16",
    HomeRecording "IAB9-17",
    InvestorsAndPatents "IAB9-18",
    JewelryMaking "IAB9-19",
    MagicAndIllusion "IAB9-20",
    Needlework "IAB9-21",
    Painting "IAB9-22",
    Photography "IAB9-23",
    Radio "IAB9-24",
    RoleplayingGames "IAB9-25",
    SciFiAndFantasy "IAB9-26",
    Scrapbooking "IAB9-27",
    Screenwriting "IAB9-28",
    StampsAndCoins "IAB9-29",
    VideoAndComputerGames "IAB9-30",
    Woodworking "IAB9-31";

    HomeAndGarden "IAB10",
    Appliances "IAB10-1",
    Entertaining "IAB10-2",
    EnvironmentalSafety "IAB10-3",
    Gardening "IAB10-4",
    HomeRepair "IAB10-5",
    HomeTheater "IAB10-6",
    InteriorDecorating "IAB10-7",
    Landscaping "IAB10-8",
    RemodelingAndConstruction "IAB10-9";

    LawGovernmentAndPolitics "IAB11",
    Immigration "IAB11-1",
    LegalIssues "IAB11-2",
    USGovernmentResources "IAB11-3",
    Politics "IAB11-4",
    Commentary "IAB11-5";

    News "IAB12",
    InternationalNews "IAB12-1",
    NationalNews "IAB12-2",
    LocalNews "IAB12-3";

    PersonalFinance "IAB13",
    BeginningInvesting "IAB13-1",
    CreditDebtAndLoans "IAB13-2",
    FinancialNews "IAB13-3",
    FinancialPlanning "IAB13-4",
    HedgeFund "IAB13-5",
    Insurance "IAB13-6",
    Investing "IAB13-7",
    MutualFunds "IAB13-8",
    Options "IAB13-9",
    RetirementPlanning "IAB13-10",
    Stocks "IAB13-11",
    TaxPlanning "IAB13-12";

    Society "IAB14",
    Dating "IAB14-1",
    DivorceSupport "IAB14-2",
    GayLife "IAB14-3",
    Marriage "IAB14-4",
    SeniorLiving "IAB14-5",
    Teens "IAB14-6",
    Weddings "IAB14-7",
    EthnicSpecific "IAB14-8";

    Science "IAB15",
    Astrology "IAB15-1",
    Biology "IAB15-2",
    Chemistry "IAB15-3",
    Geology "IAB15-4",
    ParanormalPhenomena "IAB15-5",
    Physics "IAB15-6",
    SpaceAstronomy "IAB15-7",
    Geography "IAB15-8",
    Botany "IAB15-9",
    Weather "IAB15-10";

    Pets "IAB16",
    Aquariums "IAB16-1",
    Birds "IAB16-2",
    Cats "IAB16-3",
    Dogs "IAB16-4",
    LargeAnimals "IAB16-5",
    Reptiles "IAB16-6",
    VeterinaryMedicine "IAB16-7";

    Sports "IAB17",
    AutoRacing "IAB17-1",
    Baseball "IAB17-2",
    Bicycling "IAB17-3",
    Bodybuilding "IAB17-4",
    Boxing "IAB17-5",
    CanoeingKayaking "IAB17-6",
    Cheerleading "IAB17-7",
    Climbing "IAB17-8",
    Cricket "IAB17-9",
    FigureSkating "IAB17-10",
    FlyFishing "IAB17-11",
    Football "IAB17-12",
    FreshwaterFishing "IAB17-13",
    GameAndFish "IAB17-14",
    Golf "IAB17-15",
    HorseRacing "IAB17-16",
    Horses "IAB17-17",
    HuntingShooting "IAB17-18",
    InlineSkating "IAB17-19",
    MartialArts "IAB17-20",
    MountainBiking "IAB17-21",
    NASCARRacing "IAB17-22",
    Olympics "IAB17-23",
    Paintball "IAB17-24",
    PowerAndMotorcycles "IAB17-25",
    ProBasketball "IAB17-26",
    ProIceHockey "IAB17-27",
    Rodeo "IAB17-28",
    Rugby "IAB17-29",
    RunningJogging "IAB17-30",
    Sailing "IAB17-31",
    SaltwaterFishing "IAB17-32",
    ScubaDiving "IAB17-33",
    Skateboarding "IAB17-34",
    Skiing "IAB17-35",
    Snowboarding "IAB17-36",
    SurfingBodyBoarding "IAB17-37",
    Swimming "IAB17-38",
    TableTennisPingPong "IAB17-39",
    Tennis "IAB17-40",
    Volleyball "IAB17-41",
    Walking "IAB17-42",
    WaterskiWakeboard "IAB17-43",
    WorldSoccer "IAB17-44";

    StyleAndFashion "IAB18",
    Beauty "IAB18-1",
    BodyArt "IAB18-2",
    Fashion "IAB18-3",
    Jewelry "IAB18-4",
    Clothing "IAB18-5",
    Accessories "IAB18-6";

    TechnologyAndComputing "IAB19",
    ThreeDGraphics "IAB19-1",
    Animation "IAB19-2",
    AntivirusSoftware "IAB19-3",
    CCPlusPlus "IAB19-4",
    CamerasAndCamcorders "IAB19-5",
    CellPhones "IAB19-6",
    ComputerCertification "IAB19-7",
    ComputerNetworking "IAB19-8",
    ComputerPeripherals "IAB19-9",
    ComputerReviews "IAB19-10",
    DataCenters "IAB19-11",
    Databases "IAB19-12",
    DesktopPublishing "IAB19-13",
    DesktopVideo "IAB19-14",
    Email "IAB19-15",
    GraphicsSoftware "IAB19-16",
    HomeVideoDVD "IAB19-17",
    InternetTechnology "IAB19-18",
    Java "IAB19-19",
    JavaScript "IAB19-20",
    MacSupport "IAB19-21",
    MP3MIDI "IAB19-22",
    NetConferencing "IAB19-23",
    NetForBeginners "IAB19-24",
    NetworkSecurity "IAB19-25",
    PalmtopsPDAs "IAB19-26",
    PCSupport "IAB19-27",
    Portable "IAB19-28",
    Entertainment "IAB19-29",
    SharewareFreeware "IAB19-30",
    Unix "IAB19-31",
    VisualBasic "IAB19-32",
    WebClipArt "IAB19-33",
    WebDesignHTML "IAB19-34",
    WebSearch "IAB19-35",
    Windows "IAB19-36";

    Travel "IAB20",
    AdventureTravel "IAB20-1",
    Africa "IAB20-2",
    AirTravel "IAB20-3",
    AustraliaAndNewZealand "IAB20-4",
    BedAndBreakfasts "IAB20-5",
    BudgetTravel "IAB20-6",
    BusinessTravel "IAB20-7",
    ByUSLocale "IAB20-8",
    Camping "IAB20-9",
    Canada "IAB20-10",
    Caribbean "IAB20-11",
    Cruises "IAB20-12",
    EasternEurope "IAB20-13",
    Europe "IAB20-14",
    France "IAB20-15",
    Greece "IAB20-16",
    HoneymoonsGetaways "IAB20-17",
    Hotels "IAB20-18",
    Italy "IAB20-19",
    Japan "IAB20-20",
    MexicoAndCentralAmerica "IAB20-21",
    NationalParks "IAB20-22",
    SouthAmerica "IAB20-23",
    Spas "IAB20-24",
    ThemeParks "IAB20-25",
    TravelingWithKids "IAB20-26",
    UnitedKingdom "IAB20-27";

    RealEstate "IAB21",
    Apartments "IAB21-1",
    Architects "IAB21-2",
    BuyingSellingHomes "IAB21-3";

    Shopping "IAB22",
    ContestsAndFreebies "IAB22-1",
    Couponing "IAB22-2",
    Comparison "IAB22-3",
    Engines "IAB22-4";

    ReligionAndSpirituality "IAB23",
    AlternativeReligions "IAB23-1",
    AtheismAgnosticism "IAB23-2",
    Buddhism "IAB23-3",
    Catholicism "IAB23-4",
    Christianity "IAB23-5",
    Hinduism "IAB23-6",
    Islam "IAB23-7",
    Judaism "IAB23-8",
    LatterDaySaints "IAB23-9",
    PaganWiccan "IAB23-10";

    Uncategorized "IAB24";

    NonStandardContent "IAB25",
    UnmoderatedUGC "IAB25-1",
    ExtremeGraphicExplicitViolence "IAB25-2",
    Pornography "IAB25-3",
    ProfaneContent "IAB25-4",
    HateContent "IAB25-5",
    UnderConstruction "IAB25-6",
    Incentivized "IAB25-7";

    IllegalContent "IAB26",
    IllegalContent_ "IAB26-1",
    Warez "IAB26-2",
    SpywareMalware "IAB26-3",
    CopyrightInfringement "IAB26-4"
}

impl serde::Serialize for Category {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.value())
    }
}

impl<'de> serde::Deserialize<'de> for Category {
    fn deserialize<D>(deserializer: D) -> Result<Category, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match serde::Deserialize::deserialize(deserializer) {
            Ok(s) => match TO_CATEGORY.get(s).cloned() {
                Some(c) => Ok(c),
                None => Ok(Category::Unknown(FromStr::from_str(s).unwrap())),
            },
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn serialize_parent() {
        let c = Category::ArtsAndEntertainment(ArtsAndEntertainment::ArtsAndEntertainment);
        let expected = r#""IAB1""#;
        let serialized = serde_json::to_string(&c).unwrap();

        assert_eq!(expected, serialized)
    }

    #[test]
    fn deserialize_parent() {
        let serialized = r#""IAB1""#;
        let expected = Category::ArtsAndEntertainment(ArtsAndEntertainment::ArtsAndEntertainment);
        let c = serde_json::from_str(serialized).unwrap();

        assert_eq!(expected, c)
    }

    #[test]
    fn serialize_sub() {
        let c = Category::ArtsAndEntertainment(ArtsAndEntertainment::BooksAndLiterature);
        let expected = r#""IAB1-1""#;
        let serialized = serde_json::to_string(&c).unwrap();

        assert_eq!(expected, serialized)
    }

    #[test]
    fn deserialize_sub() {
        let serialized = r#""IAB1-1""#;
        let expected = Category::ArtsAndEntertainment(ArtsAndEntertainment::BooksAndLiterature);
        let c = serde_json::from_str(serialized).unwrap();

        assert_eq!(expected, c)
    }

    #[test]
    fn serialize_unknown() {
        let c = Category::Unknown("1234".to_string());
        let expected = r#""1234""#;
        let serialized = serde_json::to_string(&c).unwrap();

        assert_eq!(expected, serialized)
    }

    #[test]
    fn deserialize_unknown() {
        let serialized = r#""1234""#;
        let expected = Category::Unknown("1234".to_string());
        let c = serde_json::from_str(serialized).unwrap();

        assert_eq!(expected, c)
    }
}
