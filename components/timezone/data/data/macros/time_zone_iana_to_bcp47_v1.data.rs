// @generated
/// Implement `DataProvider<IanaToBcp47MapV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_time_zone_iana_to_bcp47_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.66"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.66"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_TIME_ZONE_IANA_TO_BCP47_V1: &'static <icu::timezone::provider::names::IanaToBcp47MapV1Marker as icu_provider::DataMarker>::Yokeable = &icu::timezone::provider::names::IanaToBcp47MapV1 { map: zerotrie::ZeroTriePerfectHash { store: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\xE1s\0\0\x02\x01\x01\x04\0\0\0\x01\x01\x02\0\x04\0\0\0\0\0\0rautbwegczhjilmnkps\0\x0F\x0F\x0F\x10\x10\x13\x13\x14\x14\x14\x14\x15\x15\x15\x15\x15\x17\tU\xF2\xF9 %\x8F\xB8\\ap\x7F\t\x0F>Q[Go\xC2ck\x02\x92i\x91J\xE1gfmnrstu\x02\t\t\t\r\x0E<T\xCE\xE1\xBBArica/\xE1r\0\0\0\0\0\0\0\0\x02\x02\x02\x01\x03\x01\x0C\0\0\0\x02lmnopsbcwatdefhkjg\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x01\x01\x013h\x8F\x9B\xA6\xAF\xF0\x13\x1CQk\x93\x9C\xA5\xAC\xD1\xE5\xC4aiou\x05\x0F\x13gos\x92\nbreville\x91\x10me\x92^\xC3abs\x04\rnda\x88umbashi\x90_aka\x93;\xC3abo\x15\x1C\xC3lps\x05\nabo\x91#uto\x92\x05eru\x91Zabane\x92Z\xC2gn\x08adishu\x92Srovia\x91Y\xC4adio\x07\x0F\x15irobi\x91Ajamena\x92\\amey\x92\x08uakchott\x91ouagadougou\x90$orto-novo\x90(ao_tome\x92V\xC5ailru\x15\x1B#.\xC2mn\x05ako\x91g\xC2gj\x04ui\x90aul\x91\x1Dssau\x91(antyre\x91uazzaville\x90bjumbura\x90'\xC3aeo\x11\x16\xC2is\x04ro\x90\x7Fablanca\x91_uta\x91\x02nakry\x91\x1Findhoek\x92\x06\xC5bcdls\x07\x0C\x17\x1Eidjan\x90dcra\x91\x17dis_ababa\x91\x06giers\x90{m\xC2ae\x04ra\x91\x01ra\x91\x01\xC3iru\x08\x0Fmbuktu\x91gipoli\x91^nis\x92d\xC3ajo\x14\x1C\xC2kr\x04ar\x92R_es_salaam\x92jibouti\x90wuala\x90il_aaiun\x91\0reetown\x92Parare\x93<\xC3ahi\x07\x0Fmpala\x92oartoum\x92I\xC2gn\x05ali\x92Eshasa\x90`\xC2ou\x0Channesburg\x93:ba\x92Uaborone\x90?erica/\xE1v\0\0\0\0\0\0\0\0\0\0\x02\0\x02\0\x02\0\n\0\0\0\0\0\x1Cnopmrstwvcbadyfehijklg\0\0\0\x01\x01\x02\x02\x02\x02\x03\x03\x04\x04\x04\x05\x05\x05\x05\x05\x06\x06Zb\xD5{\xC4a\x9B\xB1\xC4g\xC6\xAB\xE4\xFA\x18C`\xD9\xF1)^\xC5aeiou\x06\x0E\x15Lssau\x90=w_york\x93\x07pigon\x90K\xC2mr\x03e\x93\t\xC2ot\x05nha\x903h_dakota/\xC3bcn\x07\x0Eeulah\x93\x0Fenter\x93\x05ew_salem\x93\x06uk\x91\x1Ajinaga\x91\x7F\xC4ahou\x1E%R\xC2nr\x11\xC2ag\x04ma\x92\x14nirtung\x90Lamaribo\x92Toenix\x93\nrt\xC3-_o\x0B\x15au-prince\x91.of_spain\x92g_\xC2av\x05cre\x908elho\x907\xC2en\nrto_rico\x92 ta_arenas\x90g\xC4aeio;ks\xC5cnrtz\x05\x11\"*eio\x906a\xC2gu\x04ua\x92\x0Bs\x905\xC2it\x05got\x91!inique\x91namoros\x91zatlan\x91~\xC4nrtx\x10\x15\x1E\xC2do\x05oza\x90\tminee\x93\x02ida\x91|lakatla\x93\x04ico_city\x91{quelon\x92\x1En\xC2ct\x05ton\x90J\xC3ers\x0F\x14\xC2rv\x05rey\x91}ideo\x93,eal\x90Qerrat\x91p\xC4aeio\x190:\xC2in\nny_river\x90Dkin_inlet\x90X\xC3cgs\x05\nife\x909ina\x90Nolute\x90Mo_branco\x908sario\x90\x04\xC6achitw2>FK\x84\xC2no&t\xC3aio\x10\x15\xC2_r\x08isabel\x92\x01em\x90<ago\x90h_domingo\x90z_paulo\x90:oresbysund\x91\x1Biprock\x92ytka\x93\x0B_\xC6bjkltv\x0B\x11\x17\x1D$arthelemy\x91\"ohns\x90Oitts\x91Hucia\x91Vhomas\x933incent\x930ift_current\x90[\xC4ehio\x0B\x1C#gucigalpa\x91,u\xC2ln\x03e\x91\x1Cder_bay\x90Pjuana\x92\x02r\xC2ot\x05nto\x90Qola\x932\xC2hi\nitehorse\x90Znnipeg\x90S\xC2ai\tncouver\x90Rrgin\x933\xC6ahioruCTa\x82\x89\xC5mnrty\x19\x1E$,\xC2bp\x0Bridge_bay\x90Uo_grande\x901cun\x91xacas\x931amarca\x90\x05\xC2em\x05nne\x91\x15an\x91Li\xC2ch\x05ago\x92xuahua\x91vudad_juarez\x91w\xC2rs\x14\xC2ad\x0Bl_harbour\x90]oba\x90\x04ta_rica\x90meston\x90B\xC2ir\x05aba\x900acao\x87\xC5aelou\x1A&2I\xC2hr\x0Fia\x90;_banderas\x92\0bados\x90!l\xC2ei\x03m\x90.ze\x90Aanc-sablon\x90T\xC3agi\x08\r_vista\x90/ota\x90lse\x92wenos_aires\x90\x03\xC5dnrst\x04\x1D\xC5\xCDak\x92t\xC3cgt\x08\x0Ehorage\x92vuilla\x84igua\x83\xC3agu\x08\x9Eguaina\x90-entina/\xC9bcjlmrstu\r17@HUmuuenos_aires\x90\x03\xC2ao\ttamarca\x90\x05\xC2mr\rodrivadavia\x90\x05doba\x90\x04ujuy\x90\x07a_rioja\x90\x06endoza\x90\tio_gallegos\x90\na\xC2ln\x04ta\x90\x0B_\xC2jl\x05uan\x90\ruis\x90\x08ucuman\x90\x0Cshuaia\x90\x0Eba\x90\x1Euncion\x92&\xC2ik\x07kokan\x90]a\x92t\xC3aeo\x1C+\xC2nw\x0Bmarkshavn\x91\x19son\x90V_creek\x90W\xC2nt\x05ver\x92yroit\x92zminica\x90y\xC2ae\x07kutat\x93\x10llowknife\x90\\ort\xC2_a\x11\xC2nw\x07elson\x90Eayne\x92|leza\x904\xC4diln\x08\x10\x1Bmonton\x90Crunepe\x902_salvador\x92Wsenada\x92\x02\xC2ae\x0F\xC2lv\x06ifax\x90Hana\x90ormosillo\x91y\xC2nqn\xC2dueiana\xC2/pW\xC7ikmptvw\r\x12\x1A%/Andianapolis\x92|nox\x92\x7Farengo\x92uetersburg\x93\x0Eell_city\x93\x0C\xC2ei\x05vay\x92}ncennes\x93\x08inamac\x93\rolis\x92|vik\x90Yaluit\x90I\xC2au\x07maica\x91>\xC2jn\x04uy\x90\x07eau\x92~\xC3enr!(ntucky/\xC2lm\x0Bouisville\x93\x01onticello\x93\x03ox_in\x92\x7Falendijk\x90,\xC3aio\x06\n_paz\x90+ma\x92\x15\xC3suw\n\x13_angeles\x93\0isville\x93\x01er_princes\x92X\xC4loru\t\x1B.ace_bay\x90F\xC2do\x06thab\x91\x1Ase_bay\x90G\xC2ae\tnd_turk\x92[nada\x91\x13\xC2ay\x1C\xC3dty\x08\x0Feloupe\x91 emala\x91&aquil\x90}ana\x91)tarctica/\xC8cdmprstv\x05\x1A4:ATZasey\x89\xC2au\x04vis\x8Amontdurville\x8B\xC2ac\x10\xC2cw\x08quarie\x90\x1Bson\x8Cmurdo\x8Dalmer\x8Eothera\x8F\xC2oy\nuth_pole\x92\x11owa\x90\0roll\x90\x01ostok\x90\x02ctic/longyearbyen\x92Nia/\xE1u\0\0\0\0\0\0\0\0\0\0\0\0\0\x03\x01\x04\x04\x01\0\x02\x0B\0ijkgmnopqrstauvyfdbch\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x02\x02\x02\x02\x02\x02\x03\x03\x131\xA3\xA8\xD5\xF7\x03%AR\x99\xE3*c{\xA3\xAD\xDB\x1BU\xC2rs\x07kutsk\x920tanbul\x92f\xC2ae\x11\xC2ky\x06arta\x911apura\x910rusalem\x91<\xC5ahoru2:AL\xC5bmrst\x03\x0B\x11\x17ul\x82chatka\x92:achi\x92\x1Chgar\x90k\xC2hm\x07mandu\x92\x0Eandu\x92\x0Eandyga\x922lkata\x916asnoyarsk\x923\xC3acw\x0B\x11la_lumpur\x92\x04hing\x92\x03ait\x91Kaza\x91\x11\xC2au#\xC4cgkn\t\x0F\x16a\xC2ou\x02\x91l\x91ladan\x92/assar\x912ila\x92\x1Bscat\x92\x13\xC2io\x07cosia\x90svo\xC2ks\tuznetsk\x927ibirsk\x929\xC2mr\x04sk\x928al\x91S\xC3hoy\n\x13nom_penh\x91Cntianak\x913ongyang\x91I\xC3aoy\x05\rtar\x92'stanay\x91Qzylorda\x91R\xC2ai\x07ngoon\x91hyadh\x92F\xC5aehir\x1A\x1F'0\xC3ikm\x05\x0Cgon\x934halin\x92@arkand\x93-oul\x91Janghai\x90jngapore\x92Kednekolymsk\x92<\xC5abeho\x10\x17'4\xC2is\x05pei\x92ihkent\x93.ilisi\x91\x14\xC2hl\x05ran\x919_aviv\x91<im\xC2bp\x03u\x90>hu\x90>\xC2km\x04yo\x91@sk\x92=\xC7dlmnqst\x04\n\x0F\x15!3en\x938maty\x91Oman\x91?adyr\x92.t\xC2ao\x03u\x91Mbe\x91Nh\xC2gk\x06abat\x92chabad\x92cyrau\x91P\xC4jlrs\r#)ung_pandang\x912a\xC2an\tnbaatar\x91k_bator\x91kumqi\x90kt-nera\x92?\xC2il\tentiane\x91Tadivostok\x92B\xC2ae\x0F\xC2kn\x06utsk\x92Dgon\x91h\xC2kr\x0Caterinburg\x92Cevan\x86amagusta\x90r\xC4ahiu\x0F\x14\x18\xC2cm\x04ca\x90\"ascus\x92Yaka\x90\"li\x92b\xC2bs\x03ai\x81hanbe\x92`\xC4aeir%+2\xC5ghknr\x06\x0C\x0F\x15hdad\x918rain\x90&u\x90\x1Fgkok\x92_naul\x92,irut\x91Ushkek\x91Bunei\x90*\xC3aho\x08-lcutta\x916\xC3iou\x04\x17ta\x92-\xC2in\x08balsan\x91igqing\x90jngking\x90jlombo\x91X\xC3aeo\x06\x0Crbin\x90jbron\x91*\xC3_nv\n\x12chi_minh\x934g_kong\x91+d\x91jlantic/\xC8abcfjmrs\x07\x0F\"0:BLzores\x92$ermuda\x90)a\xC2np\x05ary\x91\x03e_verde\x90pa\xC2er\x05roe\x91\x0Eoe\x91\x0Ean_mayen\x92Nadeira\x92\"eykjavik\x91:\xC2ot\ruth_georgia\x91%\xC2_a\x08helena\x92Lnley\x91\nstralia/\xD0\x01\x01\0\0\x05\0\0\0\0\0\n\0\0\t\0\0\x0Faqbsdecwhyptlmnv\x0F\x1A0?FL^cju{\x84\x9E\xA8\xB4\xC2cd\x03t\x90\x1Delaide\x90\x11ueensland\x90\x13r\xC2io\x07sbane\x90\x13ken_hill\x90\x12\xC2oy\x05uth\x90\x11dney\x90\x1Darwin\x90\x14ucla\x90\x15\xC2au\x08nberra\x90\x1Drrie\x90\x17est\x90\x1Cobart\x90\x16ancowinna\x90\x12erth\x90\x1Casmania\x90\x16\xC3hio\x03\x0Bi\x90\x19ndeman\x90\x18rd_howe\x90\x19elbourne\x90\x1A\xC2os\x05rth\x90\x14w\x90\x1Dictoria\x90\x1A\xC4cnst\x03\x0C\x92t\x93\x11iversal\x93\x11/\xC8acehimps\x1B#7>Mao\xC2lr\x10\xC2ae\x05ska\x92vutian\x92tizona\x93\nentral\x92xast\xC2-e\tindiana\x92|rn\x93\x07awaii\x92{ndiana-starke\x92\x7F\xC2io\x08chigan\x92zuntain\x92yacific\x93\0-new\x93\0amoa\x90\x0Fc\x93\x11urkey\x92frazil/\xC4adew\x05\x0F\x14cre\x908enoronha\x903ast\x90:est\x905-su\x926\xE1egistu\0\0\0\0\x05\t\x12\xC5ypt\x90\x7Fre\x914t\x93$5edt\x91\x05c/\xC3guz\x88\xA6\xC2mr{t\x91\x1E\xC3+-04p\xCA0123456789\x02\x10\x12\x14\x16\x18\x1A\x1C\x1E\x91\x1E\x93 \xC3012\x02\x04\x93)\x93*\x93+\x93!\x93\"\x93#\x93$\x93%\x93&\x93'\x93(\xCA0123456789\x02\x18\x1A\x1C\x1E \"$&\x91\x1E\x93\x12\xC501234\x02\x04\x06\x08\x93\x1B\x93\x1C\x93\x1D\x93\x1E\x93\x1F\x93\x13\x93\x14\x93\x15\x93\x16\x93\x17\x93\x18\x93\x19\x93\x1A\x91\x1Eeenwich\x91\x1E\xC3cnt\x03\x15t\x93\x11\xC2ik\x08versal\x93\x11nown\x92sc\x93\x11ulu\x93\x11rope/\xE1u\0\0\0\0\0\0\0\0\0\0\0\0\0\x04\x01\x01\x04\0\0\0\x01\0ijklmnopurstawvbdzgch\0\0\0\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01\x01\x02\x02\x02\x16\x1D=d\x94\x9C\xA1\xBB\xD0\xDC&>dk\x97\xE5\xEC\x08\x1D3s\xC2lt\ne_of_man\x915anbul\x92fersey\x91=\xC3aiy\x0B\x16liningrad\x921\xC2er\x03v\x92kov\x925iv\x92k\xC4ijou\x06\x0F\x15sbon\x92#ubljana\x92Mndon\x91\x12xembourg\x91\\\xC3aio\x17\x1C\xC3dlr\x05\trid\x91\x04ta\x91riehamn\x91\x08nsk\x90@\xC2ns\x05aco\x91`cow\x926icosia\x90sslo\x92\r\xC3aor\x05\x0Eris\x91\x0Fdgorica\x91bague\x90t\xC2lz\tyanovsk\x92>hgorod\x92n\xC2io\x04ga\x91]me\x91;\xC5aikot\",27\xC3mnr\x05\x0Eara\x924_marino\x92Qa\xC2jt\x05evo\x90 ov\x92;mferopol\x92mopje\x91ffia\x90%ockholm\x92J\xC2ai\x07llinn\x90~ra\xC2ns\x02e\x85pol\x91a\xC4mnst\t\x0F\x18sterdam\x92\x0Cdorra\x80trakhan\x92+hens\x91$arsaw\x92\x1D\xC3aio\x0E\x1D\xC2dt\x04uz\x91Wican\x93/\xC2el\x05nna\x90\x10nius\x91[lgograd\x92A\xC3eru\x18,\xC2lr\x0F\xC2fg\x05ast\x91\x12rade\x92*lin\x90u\xC2au\ttislava\x92Ossels\x90#\xC3cds\x08\x0Fharest\x92)apest\x91/ingen\x90vublin\x914\xC2au\x12\xC2gp\x05reb\x91-orozhye\x92lrich\x90c\xC2iu\tbraltar\x91\x18ernsey\x91\x16\xC2ho\x08isinau\x91apenhagen\x90xelsinki\x91\x07\xC3bmr\t\x1A\x91\x12-eire\x91\x12t\x91\x1E\xC3+-0\x03\x060\x91\x1E0\x91\x1E\x91\x1Eeenwich\x91\x1E\xC4ahsup\x91\x98nada/\xC8acemnpsy\t\x11*3@HUtlantic\x90Hentral\x90Sast\xC2-e\x0Esaskatchewan\x90Nrn\x90Qountain\x90Cewfoundland\x90Oacific\x90Raskatchewan\x90Nukon\x90Zile/\xC2ce\x0Continental\x90hasterisland\x90ft6cdt\x90nba\x90oulu\x93\x11\xC2os\x08ngkong\x91+t\x93)a\xC2mp\x06aica\x91>an\x91@\xC4cnrs\x07x|eland\x91:dian/\xC5ackmr\r/9Zntananarivo\x91c\xC2ho\x11\xC2ar\x05gos\x917istmas\x90q\xC2cm\x04os\x90^oro\x91Gerguelen\x92]a\xC4hluy\x03\n\x12e\x92Hdives\x91tritius\x91sotte\x939eunion\x92(an\x919rael\x91<ibya\x91^\xC2es\"xico/\xC2bg\x11aja\xC2ns\x06orte\x92\x02ur\x91~eneral\x91{t\x93&7mdt\x91q\xC2az\x06vajo\x92y\x92\x11-chat\x92\x12wajalein\x91d\xE1daors\x01\x01\x01\xC6\xD6\xD9cific/\xE1q\0\x01\x01\x03\0\x02\x04\0\0\0\0\0\0\x01\x01\t\0\x01gfjychtmnkpasrbew\0\0\0\0\0\0\0\0\0\0\x01\x01\x01\x01\x01\x01&?HL[d\x85\xA0\xBE\xE5\x1F/=GTn\xC2au\x12\xC2lm\x08apagos\x90|bier\x92\x16a\xC2dm\talcanal\x92G\x91'\xC3aiu\x07\x0Bkaofo\x92aji\x91\tnafuti\x92hohnston\x92qap\x91\rh\xC2au\x06tham\x92\x12uk\x91\ronolulu\x92{\xC3aor\x0E\x17\xC2hr\x05iti\x92\x18awa\x91Fngatapu\x92euk\x91\r\xC2ai\x11\xC2jr\x05uro\x91equesas\x92\x17dway\x92r\xC3aio\x05\turu\x92\x0Fue\x92\x10\xC2ru\x06folk\x92\tmea\x92\x07\xC4aiow\x06\x10\x16nton\x91Eritimati\x91Dsrae\x91\x0Bajalein\x91d\xC3aio\x10\x18\xC2gl\x08o_pago\x90\x0Fau\x92%tcairn\x92\x1F\xC3hnr\x06\x0Bnpei\x91\x0Cape\x91\x0Ct_moresby\x92\x19\xC2pu\x04ia\x937ckland\x92\x11a\xC2im\x05pan\x91moa\x90\x0Farotonga\x90eougainville\x92\x1A\xC3afn\x06\x0Bster\x90fate\x935derbury\x91Ea\xC2kl\x03e\x92plis\x936\xC2lr\x05and\x92\x1Dtugal\x92#c\x90jt8pdt\x92!ingapore\x92K") } }.into_zerotrie(), bcp47_ids: unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"adalv\0\0\0aedxb\0\0\0afkbl\0\0\0aganu\0\0\0aiaxa\0\0\0altia\0\0\0amevn\0\0\0ancur\0\0\0aolad\0\0\0aqcas\0\0\0aqdav\0\0\0aqddu\0\0\0aqmaw\0\0\0aqmcm\0\0\0aqplm\0\0\0aqrot\0\0\0aqsyw\0\0\0aqtrl\0\0\0aqvos\0\0\0arbue\0\0\0arcor\0\0\0arctc\0\0\0arirj\0\0\0arjuj\0\0\0arluq\0\0\0armdz\0\0\0arrgl\0\0\0arsla\0\0\0artuc\0\0\0aruaq\0\0\0arush\0\0\0asppg\0\0\0atvie\0\0\0auadl\0\0\0aubhq\0\0\0aubne\0\0\0audrw\0\0\0aueuc\0\0\0auhba\0\0\0aukns\0\0\0auldc\0\0\0auldh\0\0\0aumel\0\0\0aumqi\0\0\0auper\0\0\0ausyd\0\0\0awaua\0\0\0azbak\0\0\0basjj\0\0\0bbbgi\0\0\0bddac\0\0\0bebru\0\0\0bfoua\0\0\0bgsof\0\0\0bhbah\0\0\0bibjm\0\0\0bjptn\0\0\0bmbda\0\0\0bnbwn\0\0\0bolpb\0\0\0bqkra\0\0\0braux\0\0\0brbel\0\0\0brbvb\0\0\0brcgb\0\0\0brcgr\0\0\0brern\0\0\0brfen\0\0\0brfor\0\0\0brmao\0\0\0brmcz\0\0\0brpvh\0\0\0brrbr\0\0\0brrec\0\0\0brsao\0\0\0brssa\0\0\0brstm\0\0\0bsnas\0\0\0btthi\0\0\0bwgbe\0\0\0bymsq\0\0\0bzbze\0\0\0cacfq\0\0\0caedm\0\0\0caffs\0\0\0cafne\0\0\0caglb\0\0\0cagoo\0\0\0cahal\0\0\0caiql\0\0\0camon\0\0\0canpg\0\0\0capnt\0\0\0careb\0\0\0careg\0\0\0casjf\0\0\0cathu\0\0\0cator\0\0\0cavan\0\0\0cawnp\0\0\0caybx\0\0\0caycb\0\0\0cayda\0\0\0caydq\0\0\0cayek\0\0\0cayev\0\0\0cayxy\0\0\0cayyn\0\0\0cayzf\0\0\0cayzs\0\0\0cccck\0\0\0cdfbm\0\0\0cdfih\0\0\0cfbgf\0\0\0cgbzv\0\0\0chzrh\0\0\0ciabj\0\0\0ckrar\0\0\0clipc\0\0\0clpuq\0\0\0clscl\0\0\0cmdla\0\0\0cnsha\0\0\0cnurc\0\0\0cobog\0\0\0crsjo\0\0\0cst6cdt\0cuhav\0\0\0cvrai\0\0\0cxxch\0\0\0cyfmg\0\0\0cynic\0\0\0czprg\0\0\0deber\0\0\0debsngn\0djjib\0\0\0dkcph\0\0\0dmdom\0\0\0dosdq\0\0\0dzalg\0\0\0ecgps\0\0\0ecgye\0\0\0eetll\0\0\0egcai\0\0\0eheai\0\0\0erasm\0\0\0esceu\0\0\0eslpa\0\0\0esmad\0\0\0est5edt\0etadd\0\0\0fihel\0\0\0fimhq\0\0\0fjsuv\0\0\0fkpsy\0\0\0fmksa\0\0\0fmpni\0\0\0fmtkk\0\0\0fotho\0\0\0frpar\0\0\0galbv\0\0\0gazastrpgblon\0\0\0gdgnd\0\0\0getbs\0\0\0gfcay\0\0\0gggci\0\0\0ghacc\0\0\0gigib\0\0\0gldkshvnglgoh\0\0\0globy\0\0\0glthu\0\0\0gmbjl\0\0\0gmt\0\0\0\0\0gncky\0\0\0gpbbr\0\0\0gpmsb\0\0\0gpsbh\0\0\0gqssg\0\0\0grath\0\0\0gsgrv\0\0\0gtgua\0\0\0gugum\0\0\0gwoxb\0\0\0gygeo\0\0\0hebron\0\0hkhkg\0\0\0hntgu\0\0\0hrzag\0\0\0htpap\0\0\0hubud\0\0\0iddjj\0\0\0idjkt\0\0\0idmak\0\0\0idpnk\0\0\0iedub\0\0\0imdgs\0\0\0inccu\0\0\0iodga\0\0\0iqbgw\0\0\0irthr\0\0\0isrey\0\0\0itrom\0\0\0jeruslm\0jesth\0\0\0jmkin\0\0\0joamm\0\0\0jptyo\0\0\0kenbo\0\0\0kgfru\0\0\0khpnh\0\0\0kicxi\0\0\0kipho\0\0\0kitrw\0\0\0kmyva\0\0\0knbas\0\0\0kpfnj\0\0\0krsel\0\0\0kwkwi\0\0\0kygec\0\0\0kzaau\0\0\0kzakx\0\0\0kzala\0\0\0kzguw\0\0\0kzksn\0\0\0kzkzo\0\0\0kzura\0\0\0lavte\0\0\0lbbey\0\0\0lccas\0\0\0livdz\0\0\0lkcmb\0\0\0lrmlw\0\0\0lsmsu\0\0\0ltvno\0\0\0lulux\0\0\0lvrix\0\0\0lytip\0\0\0macas\0\0\0mcmon\0\0\0mdkiv\0\0\0metgd\0\0\0mgtnr\0\0\0mhkwa\0\0\0mhmaj\0\0\0mkskp\0\0\0mlbko\0\0\0mmrgn\0\0\0mncoq\0\0\0mnhvd\0\0\0mnuln\0\0\0momfm\0\0\0mpspn\0\0\0mqfdf\0\0\0mrnkc\0\0\0msmni\0\0\0mst7mdt\0mtmla\0\0\0muplu\0\0\0mvmle\0\0\0mwblz\0\0\0mxchi\0\0\0mxcjs\0\0\0mxcun\0\0\0mxhmo\0\0\0mxmam\0\0\0mxmex\0\0\0mxmid\0\0\0mxmty\0\0\0mxmzt\0\0\0mxoji\0\0\0mxpvr\0\0\0mxstis\0\0mxtij\0\0\0mykch\0\0\0mykul\0\0\0mzmpm\0\0\0nawdh\0\0\0ncnou\0\0\0nenim\0\0\0nfnlk\0\0\0nglos\0\0\0nimga\0\0\0nlams\0\0\0noosl\0\0\0npktm\0\0\0nrinu\0\0\0nuiue\0\0\0nzakl\0\0\0nzcht\0\0\0ommct\0\0\0papty\0\0\0pelim\0\0\0pfgmr\0\0\0pfnhv\0\0\0pfppt\0\0\0pgpom\0\0\0pgraw\0\0\0phmnl\0\0\0pkkhi\0\0\0plwaw\0\0\0pmmqc\0\0\0pnpcn\0\0\0prsju\0\0\0pst8pdt\0ptfnc\0\0\0ptlis\0\0\0ptpdl\0\0\0pwror\0\0\0pyasu\0\0\0qadoh\0\0\0rereu\0\0\0robuh\0\0\0rsbeg\0\0\0ruasf\0\0\0rubax\0\0\0ruchita\0rudyr\0\0\0rugdx\0\0\0ruikt\0\0\0rukgd\0\0\0rukhndg\0rukra\0\0\0rukuf\0\0\0rukvx\0\0\0rumow\0\0\0runoz\0\0\0ruoms\0\0\0ruovb\0\0\0rupkc\0\0\0rurtw\0\0\0rusred\0\0rutof\0\0\0ruuly\0\0\0ruunera\0ruuus\0\0\0ruvog\0\0\0ruvvo\0\0\0ruyek\0\0\0ruyks\0\0\0rwkgl\0\0\0saruh\0\0\0sbhir\0\0\0scmaw\0\0\0sdkrt\0\0\0sesto\0\0\0sgsin\0\0\0shshn\0\0\0silju\0\0\0sjlyr\0\0\0skbts\0\0\0slfna\0\0\0smsai\0\0\0sndkr\0\0\0somgq\0\0\0srpbm\0\0\0ssjub\0\0\0sttms\0\0\0svsal\0\0\0sxphi\0\0\0sydam\0\0\0szqmn\0\0\0tcgdt\0\0\0tdndj\0\0\0tfpfr\0\0\0tglfw\0\0\0thbkk\0\0\0tjdyu\0\0\0tkfko\0\0\0tldil\0\0\0tmasb\0\0\0tntun\0\0\0totbu\0\0\0trist\0\0\0ttpos\0\0\0tvfun\0\0\0twtpe\0\0\0tzdar\0\0\0uaiev\0\0\0uaozh\0\0\0uasip\0\0\0uauzh\0\0\0ugkla\0\0\0umawk\0\0\0umjon\0\0\0ummdy\0\0\0unk\0\0\0\0\0usadk\0\0\0usaeg\0\0\0usanc\0\0\0usboi\0\0\0uschi\0\0\0usden\0\0\0usdet\0\0\0ushnl\0\0\0usind\0\0\0usinvev\0usjnu\0\0\0usknx\0\0\0uslax\0\0\0uslui\0\0\0usmnm\0\0\0usmoc\0\0\0usmtm\0\0\0usndcnt\0usndnsl\0usnyc\0\0\0usoea\0\0\0usome\0\0\0usphx\0\0\0ussit\0\0\0ustel\0\0\0uswlz\0\0\0uswsq\0\0\0usxul\0\0\0usyak\0\0\0utc\0\0\0\0\0utce01\0\0utce02\0\0utce03\0\0utce04\0\0utce05\0\0utce06\0\0utce07\0\0utce08\0\0utce09\0\0utce10\0\0utce11\0\0utce12\0\0utce13\0\0utce14\0\0utcw01\0\0utcw02\0\0utcw03\0\0utcw04\0\0utcw05\0\0utcw06\0\0utcw07\0\0utcw08\0\0utcw09\0\0utcw10\0\0utcw11\0\0utcw12\0\0uymvd\0\0\0uzskd\0\0\0uztas\0\0\0vavat\0\0\0vcsvd\0\0\0veccs\0\0\0vgtov\0\0\0vistt\0\0\0vnsgn\0\0\0vuvli\0\0\0wfmau\0\0\0wsapw\0\0\0yeade\0\0\0ytmam\0\0\0zajnb\0\0\0zmlun\0\0\0zwhre\0\0\0") }, bcp47_ids_checksum: 11638891890028489555u64 };
        }
        #[clippy::msrv = "1.66"]
        impl icu_provider::DataProvider<icu::timezone::provider::names::IanaToBcp47MapV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::timezone::provider::names::IanaToBcp47MapV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_TIME_ZONE_IANA_TO_BCP47_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::timezone::provider::names::IanaToBcp47MapV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}