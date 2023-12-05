#![allow(dead_code, unused)]
use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Map {
    dst: i64,
    src: i64,
    count: i64,
}

impl Map {
    fn new(dst: i64, src: i64, count: i64) -> Self {
        Self { src, dst, count }
    }
}

#[derive(Clone, Debug)]
struct Almanac {
    seeds: Vec<i64>,

    seed_to_soil: Vec<Map>,
    soil_to_fertilizer: Vec<Map>,
    fertilizer_to_water: Vec<Map>,
    water_to_light: Vec<Map>,
    light_to_temperature: Vec<Map>,
    temperature_to_humidity: Vec<Map>,
    humidity_to_location: Vec<Map>,
}

fn find_in_map(map: &[Map], src: i64) -> i64 {
    let map = map
        .iter()
        .find(|map| map.src <= src && src < map.src + map.count);

    if let Some(map) = map {
        if map.src <= src && src <= map.src + map.count {
            // dbg!(src, map, map.src, map.dst, (src - map.dst));
            return map.dst + (src - map.src);
        }
    }

    // 1-to-1
    src
}

impl Almanac {
    fn get_seed_to_soil(&self, seed: i64) -> i64 {
        find_in_map(&self.seed_to_soil, seed)
    }

    fn get_soil_to_fertilizer(&self, seed: i64) -> i64 {
        find_in_map(&self.soil_to_fertilizer, seed)
    }

    fn get_fertilizer_to_water(&self, seed: i64) -> i64 {
        find_in_map(&self.fertilizer_to_water, seed)
    }

    fn get_water_to_light(&self, seed: i64) -> i64 {
        find_in_map(&self.water_to_light, seed)
    }

    fn get_light_to_temperature(&self, seed: i64) -> i64 {
        find_in_map(&self.light_to_temperature, seed)
    }

    fn get_temperature_to_humidity(&self, seed: i64) -> i64 {
        find_in_map(&self.temperature_to_humidity, seed)
    }

    fn get_humidity_to_location(&self, seed: i64) -> i64 {
        find_in_map(&self.humidity_to_location, seed)
    }

    fn get_seed_to_location(&self, seed: i64) -> i64 {
        let x = self.get_seed_to_soil(seed);
        let x = self.get_soil_to_fertilizer(x);
        let x = self.get_fertilizer_to_water(x);
        let x = self.get_water_to_light(x);
        let x = self.get_light_to_temperature(x);
        let x = self.get_temperature_to_humidity(x);
        self.get_humidity_to_location(x)
    }
}

fn parse_almanac_p1(input: &str) -> Almanac {
    let _ = input;

    let seeds: Vec<_>;
    let mut seed_to_soil: Vec<_>;
    let mut soil_to_fertilizer: Vec<_>;
    let mut fertilizer_to_water: Vec<_>;
    let mut water_to_light: Vec<_>;
    let mut light_to_temperature: Vec<_>;
    let mut temperature_to_humidity: Vec<_>;
    let mut humidity_to_location: Vec<_>;

    if cfg!(test) {
        seeds = vec![79, 14, 55, 13];

        seed_to_soil = vec![
            //
            Map::new(50, 98, 2),
            Map::new(52, 50, 48),
        ];
        soil_to_fertilizer = vec![
            Map::new(0, 15, 37),
            Map::new(37, 52, 2),
            Map::new(39, 0, 15),
        ];
        fertilizer_to_water = vec![
            Map::new(49, 53, 8),
            Map::new(0, 11, 42),
            Map::new(42, 0, 7),
            Map::new(57, 7, 4),
        ];
        water_to_light = vec![
            //
            Map::new(88, 18, 7),
            Map::new(18, 25, 70),
        ];
        light_to_temperature = vec![
            Map::new(45, 77, 23),
            Map::new(81, 45, 19),
            Map::new(68, 64, 13),
        ];
        temperature_to_humidity = vec![
            //
            Map::new(0, 69, 1),
            Map::new(1, 0, 69),
        ];
        humidity_to_location = vec![
            //
            Map::new(60, 56, 37),
            Map::new(56, 93, 4),
        ];
    } else {
        seeds = vec![
            1310704671, 312415190, 1034820096, 106131293, 682397438, 30365957, 2858337556,
            1183890307, 665754577, 13162298, 2687187253, 74991378, 1782124901, 3190497, 208902075,
            226221606, 4116455504, 87808390, 2403629707, 66592398,
        ];

        seed_to_soil = vec![
            Map::new(2879792625, 0, 201678008),
            Map::new(2425309256, 1035790247, 296756276),
            Map::new(2722065532, 1759457739, 157727093),
            Map::new(400354950, 1917184832, 1164285801),
            Map::new(0, 201678008, 400354950),
            Map::new(1564640751, 602032958, 433757289),
            Map::new(1998398040, 1332546523, 426911216),
        ];

        soil_to_fertilizer = vec![
            Map::new(3434127746, 3670736129, 29685965),
            Map::new(1809924203, 1168707872, 308179),
            Map::new(2108903682, 1437989162, 44479258),
            Map::new(237181023, 2915565442, 27901445),
            Map::new(1173998623, 2434447796, 13633544),
            Map::new(75539025, 740516241, 29278225),
            Map::new(41104738, 706081954, 34434287),
            Map::new(3279397405, 3488165796, 12149874),
            Map::new(3463813711, 3827946213, 157129363),
            Map::new(1810232382, 769794466, 15695437),
            Map::new(877824710, 677909236, 28172718),
            Map::new(2215709448, 1746651561, 307558709),
            Map::new(1825927819, 1692597620, 54053941),
            Map::new(104817250, 420198730, 132363773),
            Map::new(2916210208, 392942051, 27256679),
            Map::new(1022591555, 2448081340, 151407068),
            Map::new(3925105941, 3985075576, 182313682),
            Map::new(1897186025, 2212065968, 211717657),
            Map::new(2198981202, 1304666789, 16728246),
            Map::new(850656807, 2054210270, 27167903),
            Map::new(3766599721, 3500315670, 158506220),
            Map::new(3419071398, 3279397405, 15056348),
            Map::new(7830088, 2126976435, 33274650),
            Map::new(3620943074, 3658821890, 11914239),
            Map::new(1264213180, 2599488408, 138420934),
            Map::new(811586355, 2160251085, 12020898),
            Map::new(3632857313, 3354423388, 133742408),
            Map::new(1612763314, 1169016051, 108601184),
            Map::new(1721364498, 2172271983, 39793985),
            Map::new(1187632167, 601328223, 76581013),
            Map::new(823607253, 1277617235, 27049554),
            Map::new(728944387, 2737909342, 82641968),
            Map::new(0, 2426617708, 7830088),
            Map::new(3291547279, 3700422094, 127524119),
            Map::new(1402634114, 1482468420, 210129200),
            Map::new(905997428, 1321395035, 107714902),
            Map::new(4107419623, 3294453753, 59969635),
            Map::new(1879981760, 785489903, 17204265),
            Map::new(2153382940, 2081378173, 45598262),
            Map::new(277361019, 802694168, 366013704),
            Map::new(1761158483, 552562503, 48765720),
            Map::new(646208806, 2832829861, 82735581),
            Map::new(2523268157, 0, 392942051),
            Map::new(1013712330, 1429109937, 8879225),
            Map::new(643374723, 2423783625, 2834083),
            Map::new(265082468, 2820551310, 12278551),
        ];

        fertilizer_to_water = vec![
            Map::new(4253122607, 1473424614, 41844689),
            Map::new(3040447798, 2659805568, 46237011),
            Map::new(0, 146022665, 42081460),
            Map::new(55436822, 188104125, 65067713),
            Map::new(42081460, 132667303, 13355362),
            Map::new(2429043181, 3587614447, 54605699),
            Map::new(888256662, 672288214, 24436041),
            Map::new(4064969883, 1978094070, 95324589),
            Map::new(3086684809, 977403736, 339965972),
            Map::new(120504535, 253171838, 93494065),
            Map::new(2810558403, 2603914183, 55891385),
            Map::new(3898695123, 2901215107, 166274760),
            Map::new(2483648880, 4002918707, 103777141),
            Map::new(1300545784, 2848997109, 52217998),
            Map::new(2418717938, 1463099371, 10325243),
            Map::new(1022681665, 808998429, 30429585),
            Map::new(2866449788, 1411682577, 4750813),
            Map::new(1181605510, 4172708724, 118940274),
            Map::new(2078503930, 2466708865, 42530000),
            Map::new(1105548530, 1545561518, 76056980),
            Map::new(978705579, 2573458117, 30456066),
            Map::new(2324405069, 1317369708, 94312869),
            Map::new(1991848966, 3429793336, 22435712),
            Map::new(4190586687, 2706042579, 43180396),
            Map::new(1352763782, 1416433390, 46665981),
            Map::new(3760606255, 1683093685, 138088868),
            Map::new(1399429763, 3452229048, 135385399),
            Map::new(2121033930, 839428014, 137975722),
            Map::new(2940673664, 2749222975, 99774134),
            Map::new(1053111250, 2073418659, 52437280),
            Map::new(3426650781, 1821182553, 152991287),
            Map::new(1534815162, 2195329002, 252024339),
            Map::new(730962658, 3067489867, 157294004),
            Map::new(3579642068, 710244275, 98754154),
            Map::new(1786839501, 3224783871, 205009465),
            Map::new(2259009652, 1974173840, 3920230),
            Map::new(2587426021, 370264097, 223132382),
            Map::new(2871200601, 2125855939, 69473063),
            Map::new(213998600, 44701447, 87965856),
            Map::new(4233767083, 2447353341, 19355524),
            Map::new(2262929882, 1621618498, 61475187),
            Map::new(1009161645, 696724255, 13520020),
            Map::new(3678396222, 593396479, 78891735),
            Map::new(912692703, 4106695848, 66012876),
            Map::new(3757287957, 4291648998, 3318298),
            Map::new(301964456, 0, 44701447),
            Map::new(2014284678, 2509238865, 64219252),
            Map::new(370264097, 3642220146, 360698561),
            Map::new(4160294472, 1515269303, 30292215),
        ];

        water_to_light = vec![
            Map::new(4066036887, 2992193346, 95912236),
            Map::new(531075515, 493316918, 162009008),
            Map::new(3260565192, 854248031, 437396028),
            Map::new(1341316194, 4205924684, 89042612),
            Map::new(1879858967, 2058162578, 692895326),
            Map::new(452475911, 655325926, 78599604),
            Map::new(2997176790, 1690328655, 208783332),
            Map::new(2731804884, 3324847814, 265371906),
            Map::new(355611136, 0, 96864775),
            Map::new(2572754293, 1899111987, 159050591),
            Map::new(1081338600, 3590219720, 138271571),
            Map::new(1430358806, 2779435417, 212757929),
            Map::new(3234337635, 4179697127, 26227557),
            Map::new(854248031, 3728491291, 227090569),
            Map::new(4161949123, 3955581860, 102409244),
            Map::new(3205960122, 2751057904, 28377513),
            Map::new(50952557, 147817332, 304658579),
            Map::new(1219610171, 4057991104, 121706023),
            Map::new(4264358367, 1291644059, 30608929),
            Map::new(3697961220, 1322252988, 368075667),
            Map::new(1643116735, 3088105582, 236742232),
            Map::new(693084523, 452475911, 40841007),
            Map::new(0, 96864775, 50952557),
        ];

        light_to_temperature = vec![
            Map::new(2756401132, 2384899493, 13749631),
            Map::new(1163093625, 0, 117407544),
            Map::new(3603435593, 3599927411, 262731037),
            Map::new(2081436411, 2089913126, 119300659),
            Map::new(693703633, 117407544, 395383894),
            Map::new(1672621164, 1405157690, 24997208),
            Map::new(3873714258, 2780774148, 107551276),
            Map::new(3355072403, 2593861641, 186912507),
            Map::new(1953100586, 3862658448, 62069331),
            Map::new(143286272, 672639421, 194814248),
            Map::new(1562062673, 1010739941, 110558491),
            Map::new(2869050867, 2888325424, 31673634),
            Map::new(3159859886, 2398649124, 195212517),
            Map::new(2900724501, 3298674599, 34708838),
            Map::new(2243940568, 4059045429, 56605170),
            Map::new(691405879, 1193483066, 2297754),
            Map::new(2300545738, 2005749676, 25248062),
            Map::new(3541984910, 3924727779, 61450683),
            Map::new(2200737070, 3986178462, 43203498),
            Map::new(3981265534, 2030997738, 58915388),
            Map::new(2530829166, 4276276595, 18690701),
            Map::new(621411866, 641250212, 31389209),
            Map::new(1784026205, 4037549491, 21495938),
            Map::new(1519774068, 1362869085, 42288605),
            Map::new(3866166630, 3584674072, 7547628),
            Map::new(652801075, 1430154898, 38604804),
            Map::new(2015169917, 4029381960, 8167531),
            Map::new(2770150763, 2936555750, 98900104),
            Map::new(1813227854, 2258880377, 123316040),
            Map::new(3032290681, 1784026205, 127569205),
            Map::new(0, 867453669, 143286272),
            Map::new(1805522143, 3592221700, 7705711),
            Map::new(4040180922, 3043888225, 254786374),
            Map::new(2023337448, 3035455854, 8432371),
            Map::new(3029587605, 2382196417, 2703076),
            Map::new(392553196, 1468759702, 228858670),
            Map::new(2710145863, 3538418803, 46255269),
            Map::new(1089087527, 567244114, 74006098),
            Map::new(2325793800, 3333383437, 205035366),
            Map::new(2549519867, 4115650599, 160625996),
            Map::new(338100520, 512791438, 54452676),
            Map::new(2935433339, 1911595410, 94154266),
            Map::new(1280501169, 1121298432, 72184634),
            Map::new(1352685803, 1195780820, 167088265),
            Map::new(2031769819, 2209213785, 49666592),
            Map::new(1936543894, 2919999058, 16556692),
        ];

        temperature_to_humidity = vec![
            Map::new(1606220966, 2958863752, 268926464),
            Map::new(2994413958, 1467440292, 348583188),
            Map::new(1347324773, 3453966662, 171497865),
            Map::new(3342997146, 3227790216, 188948930),
            Map::new(0, 211826810, 113744983),
            Map::new(1875147430, 1816023480, 774831860),
            Map::new(699941162, 0, 211826810),
            Map::new(443679044, 325571793, 256262118),
            Map::new(3531946076, 1280528675, 186911617),
            Map::new(1280528675, 4228171198, 66796098),
            Map::new(113744983, 581833911, 329934061),
            Map::new(1518822638, 2590855340, 50170812),
            Map::new(1568993450, 3416739146, 37227516),
            Map::new(2967816890, 4201574130, 26597068),
            Map::new(3718857693, 3625464527, 576109603),
            Map::new(2649979290, 2641026152, 317837600),
        ];

        humidity_to_location = vec![
            Map::new(3244927, 955737016, 9389705),
            Map::new(380524056, 2531586403, 38604778),
            Map::new(3713586211, 965126721, 158937945),
            Map::new(3122843287, 1406574654, 236795236),
            Map::new(776685423, 1643369890, 534268825),
            Map::new(2053493196, 0, 55930434),
            Map::new(582662115, 695344450, 194023308),
            Map::new(3885666529, 3855399097, 320692779),
            Map::new(88096722, 283368340, 98672354),
            Map::new(1901561222, 3703467123, 151931974),
            Map::new(1317500428, 2570191181, 151780331),
            Map::new(3872524156, 3690324750, 13142373),
            Map::new(2109423630, 249685414, 30437999),
            Map::new(1310954248, 4199813128, 6546180),
            Map::new(1751790747, 382040694, 149770475),
            Map::new(3056474029, 889367758, 66369258),
            Map::new(2139861629, 4176091876, 23721252),
            Map::new(12634632, 2721971512, 75462090),
            Map::new(186769076, 55930434, 193754980),
            Map::new(419128834, 531811169, 163533281),
            Map::new(3359638523, 2177638715, 353947688),
            Map::new(2163582881, 2797433602, 892891148),
            Map::new(1469280759, 1124064666, 282509988),
            Map::new(0, 280123413, 3244927),
        ];
    }

    seed_to_soil.sort();
    soil_to_fertilizer.sort();
    fertilizer_to_water.sort();
    water_to_light.sort();
    light_to_temperature.sort();
    temperature_to_humidity.sort();
    humidity_to_location.sort();

    Almanac {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

// Part1 ========================================================================
#[aoc(day5, part1)]
pub fn part1(input: &str) -> i64 {
    let almanac = parse_almanac_p1(input);

    almanac
        .seeds
        .iter()
        .map(|s| almanac.get_seed_to_location(*s))
        .min()
        .unwrap()
}

// Part2 ========================================================================
#[aoc(day5, part2)]
pub fn part2(input: &str) -> i64 {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[rstest]
    #[case::sample_line_1([50, 98, 2], Map { dst: 50, src: 98, count: 2})]
    #[case::sample_line_2([52, 50, 48], Map { dst: 52, src: 50, count: 48})]
    #[trace]
    fn check_sample_lines(#[case] nums: [i64; 3], #[case] map: Map) {
        assert_eq!(Map::new(nums[0], nums[1], nums[2]), map);
    }

    #[rstest]
    // #[case::seed_79(79, 81)]
    #[case::seed_14(14, 14)]
    #[case::seed_55(55, 57)]
    #[case::seed_13(13, 13)]
    #[trace]
    fn check_ex_seed_to_soil(#[case] seed: i64, #[case] soil: i64) {
        let almanac = parse_almanac_p1(EXAMPLE_INPUT);

        assert_eq!(almanac.get_seed_to_soil(seed), soil);
    }

    #[rstest]
    #[case::given(35, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    // #[rstest]
    // #[case::given(999_999, EXAMPLE_INPUT)]
    // #[trace]
    // fn check_ex_part_2(
    //     #[notrace]
    //     #[values(part2)]
    //     p: impl FnOnce(&str) -> i64,
    //     #[case] expected: i64,
    //     #[case] input: &str,
    // ) {
    //     let input = input.trim();
    //     assert_eq!(p(input), expected);
    // }
}
