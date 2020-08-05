use nakamoto_common::block::{Bits, Height, Time};

/// Difficulty retargeting test vector, from bitcoin main chain.
/// Each row represents a difficulty change.
///
///   (height, prev-timestamp, prev-bits, timestamp, bits)
///
pub(crate) const TARGETS: &[(Height, Time, Bits, Time, Bits)] = &[
    (2016, 1233061996, 0x1d00ffff, 1233063531, 0x1d00ffff),
    (4032, 1234465122, 0x1d00ffff, 1234466190, 0x1d00ffff),
    (6048, 1235965467, 0x1d00ffff, 1235966513, 0x1d00ffff),
    (8064, 1237507400, 0x1d00ffff, 1237508786, 0x1d00ffff),
    (10080, 1239054978, 0x1d00ffff, 1239055463, 0x1d00ffff),
    (12096, 1240599092, 0x1d00ffff, 1240599098, 0x1d00ffff),
    (14112, 1242098000, 0x1d00ffff, 1242098425, 0x1d00ffff),
    (16128, 1243735798, 0x1d00ffff, 1243737085, 0x1d00ffff),
    (18144, 1246050840, 0x1d00ffff, 1246051973, 0x1d00ffff),
    (20160, 1248481522, 0x1d00ffff, 1248481816, 0x1d00ffff),
    (22176, 1252066931, 0x1d00ffff, 1252069298, 0x1d00ffff),
    (24192, 1254454291, 0x1d00ffff, 1254454028, 0x1d00ffff),
    (26208, 1257000207, 0x1d00ffff, 1257002900, 0x1d00ffff),
    (28224, 1259358448, 0x1d00ffff, 1259358667, 0x1d00ffff),
    (30240, 1261128623, 0x1d00ffff, 1261130161, 0x1d00ffff),
    (32256, 1262152739, 0x1d00ffff, 1262153464, 0x1d00d86a),
    (34272, 1263249842, 0x1d00d86a, 1263250117, 0x1d00c428),
    (36288, 1264424481, 0x1d00c428, 1264424879, 0x1d00be71),
    (38304, 1265318937, 0x1d00be71, 1265319794, 0x1d008cc3),
    (40320, 1266190073, 0x1d008cc3, 1266191579, 0x1c654657),
    (42336, 1267000203, 0x1c654657, 1267000864, 0x1c43b3e5),
    (44352, 1268010273, 0x1c43b3e5, 1268010873, 0x1c387f6f),
    (46368, 1269211443, 0x1c387f6f, 1269212064, 0x1c381375),
    (48384, 1270119474, 0x1c381375, 1270120042, 0x1c2a1115),
    (50400, 1271061370, 0x1c2a1115, 1271061586, 0x1c20bca7),
    (52416, 1271886653, 0x1c20bca7, 1271886772, 0x1c16546f),
    (54432, 1272966003, 0x1c16546f, 1272966376, 0x1c13ec53),
    (56448, 1274278387, 0x1c13ec53, 1274278435, 0x1c159c24),
    (58464, 1275140649, 0x1c159c24, 1275141448, 0x1c0f675c),
    (60480, 1276297992, 0x1c0f675c, 1276298786, 0x1c0eba64),
    (62496, 1277382263, 0x1c0eba64, 1277382446, 0x1c0d3142),
    (64512, 1278381204, 0x1c0d3142, 1278381464, 0x1c0ae493),
    (66528, 1279007808, 0x1c0ae493, 1279008237, 0x1c05a3f4),
    (68544, 1279297671, 0x1c05a3f4, 1279297779, 0x1c0168fd),
    (70560, 1280196974, 0x1c0168fd, 1280198558, 0x1c010c5a),
    (72576, 1281037393, 0x1c010c5a, 1281037595, 0x1c00ba18),
    (74592, 1281869965, 0x1c00ba18, 1281870671, 0x1c00800e),
    (76608, 1282863700, 0x1c00800e, 1282864403, 0x1b692098),
    (78624, 1283922146, 0x1b692098, 1283922289, 0x1b5bede6),
    (80640, 1284861793, 0x1b5bede6, 1284861847, 0x1b4766ed),
    (82656, 1285703762, 0x1b4766ed, 1285703908, 0x1b31b2a3),
    (84672, 1286861405, 0x1b31b2a3, 1286861705, 0x1b2f8e9d),
    (86688, 1287637343, 0x1b2f8e9d, 1287637995, 0x1b1e7eca),
    (88704, 1288478771, 0x1b1e7eca, 1288479527, 0x1b153263),
    (90720, 1289303926, 0x1b153263, 1289305768, 0x1b0e7256),
    (92736, 1290104845, 0x1b0e7256, 1290105874, 0x1b098b2a),
    (94752, 1291134100, 0x1b098b2a, 1291135075, 0x1b081cd2),
    (96768, 1291932610, 0x1b081cd2, 1291933202, 0x1b055953),
    (98784, 1292956393, 0x1b055953, 1292956443, 0x1b04864c),
    (100800, 1294030806, 0x1b04864c, 1294031411, 0x1b0404cb),
    (102816, 1295101259, 0x1b0404cb, 1295101567, 0x1b038dee),
    (104832, 1296114735, 0x1b038dee, 1296116171, 0x1b02fa29),
    (106848, 1297140342, 0x1b02fa29, 1297140800, 0x1b028552),
    (108864, 1298003311, 0x1b028552, 1298006152, 0x1b01cc26),
    (110880, 1298799509, 0x1b01cc26, 1298800760, 0x1b012dcd),
    (112896, 1299683275, 0x1b012dcd, 1299684355, 0x1b00dc31),
    (114912, 1301020485, 0x1b00dc31, 1301020785, 0x1b00f339),
    (116928, 1302034036, 0x1b00f339, 1302034197, 0x1b00cbbd),
    (118944, 1303112797, 0x1b00cbbd, 1303112976, 0x1b00b5ac),
    (120960, 1304131540, 0x1b00b5ac, 1304131980, 0x1b0098fa),
    (122976, 1304974694, 0x1b0098fa, 1304975844, 0x1a6a93b3),
    (124992, 1305755857, 0x1a6a93b3, 1305756287, 0x1a44b9f2),
    (127008, 1306435280, 0x1a44b9f2, 1306435316, 0x1a269421),
    (129024, 1307362613, 0x1a269421, 1307363105, 0x1a1d932f),
    (131040, 1308145551, 0x1a1d932f, 1308145774, 0x1a132185),
    (133056, 1308914894, 0x1a132185, 1308915923, 0x1a0c2a12),
    (135072, 1309983257, 0x1a0c2a12, 1309984546, 0x1a0abbcf),
    (137088, 1311102675, 0x1a0abbcf, 1311103389, 0x1a09ec04),
    (139104, 1312186259, 0x1a09ec04, 1312186279, 0x1a08e1e5),
    (141120, 1313451537, 0x1a08e1e5, 1313451894, 0x1a094a86),
    (143136, 1314680496, 0x1a094a86, 1314681303, 0x1a096fe3),
    (145152, 1315906303, 0x1a096fe3, 1315906316, 0x1a098ea5),
    (147168, 1317163240, 0x1a098ea5, 1317163624, 0x1a09ee5d),
    (149184, 1318555415, 0x1a09ee5d, 1318556675, 0x1a0b6d4b),
    (151200, 1320032359, 0x1a0b6d4b, 1320032534, 0x1a0df0ca),
    (153216, 1321253256, 0x1a0df0ca, 1321253770, 0x1a0e119a),
    (155232, 1322576247, 0x1a0e119a, 1322576420, 0x1a0f61b1),
    (157248, 1323718660, 0x1a0f61b1, 1323718955, 0x1a0e8668),
    (159264, 1324923455, 0x1a0e8668, 1324925005, 0x1a0e76ba),
    (161280, 1326046766, 0x1a0e76ba, 1326047176, 0x1a0d69d7),
    (163296, 1327204081, 0x1a0d69d7, 1327204504, 0x1a0cd43f),
    (165312, 1328351050, 0x1a0cd43f, 1328351561, 0x1a0c290b),
    (167328, 1329564101, 0x1a0c290b, 1329564255, 0x1a0c309c),
    (169344, 1330676346, 0x1a0c309c, 1330676736, 0x1a0b350c),
    (171360, 1331885274, 0x1a0b350c, 1331885394, 0x1a0b3287),
    (173376, 1332999614, 0x1a0b3287, 1332999707, 0x1a0a507e),
    (175392, 1334246594, 0x1a0a507e, 1334246689, 0x1a0aa1e3),
    (177408, 1335511874, 0x1a0aa1e3, 1335512370, 0x1a0b1ef7),
    (179424, 1336565211, 0x1a0b1ef7, 1336565313, 0x1a09ae02),
    (181440, 1337882969, 0x1a09ae02, 1337883029, 0x1a0a8b5f),
    (183456, 1339098664, 0x1a0a8b5f, 1339099525, 0x1a0a98d6),
    (185472, 1340208670, 0x1a0a98d6, 1340208964, 0x1a09b78a),
    (187488, 1341401376, 0x1a09b78a, 1341401841, 0x1a099431),
    (189504, 1342536951, 0x1a099431, 1342537166, 0x1a08fd2e),
    (191520, 1343645636, 0x1a08fd2e, 1343647577, 0x1a083cc9),
    (193536, 1344772046, 0x1a083cc9, 1344772855, 0x1a07a85e),
    (195552, 1345858666, 0x1a07a85e, 1345859199, 0x1a06dfbe),
    (197568, 1346955024, 0x1a06dfbe, 1346955037, 0x1a063a38),
    (199584, 1348092805, 0x1a063a38, 1348092851, 0x1a05db8b),
    (201600, 1349227021, 0x1a05db8b, 1349226660, 0x1a057e08),
    (203616, 1350429295, 0x1a057e08, 1350428168, 0x1a0575ef),
    (205632, 1351552830, 0x1a0575ef, 1351556195, 0x1a0513c5),
    (207648, 1352742671, 0x1a0513c5, 1352743186, 0x1a04faeb),
    (209664, 1353928117, 0x1a04faeb, 1353928229, 0x1a04e0ea),
    (211680, 1355162497, 0x1a04e0ea, 1355162613, 0x1a04fa62),
    (213696, 1356530758, 0x1a04fa62, 1356530740, 0x1a05a16b),
    (215712, 1357639870, 0x1a05a16b, 1357641634, 0x1a0529b1),
    (217728, 1358965635, 0x1a0529b1, 1358966487, 0x1a05a6b1),
    (219744, 1360062830, 0x1a05a6b1, 1360063146, 0x1a051f3c),
    (221760, 1361148326, 0x1a051f3c, 1361148470, 0x1a04985c),
    (223776, 1362159549, 0x1a04985c, 1362159764, 0x1a03d74b),
    (225792, 1363249652, 0x1a03d74b, 1363249946, 0x1a0375fa),
    (227808, 1364125673, 0x1a0375fa, 1364126425, 0x1a02816e),
    (229824, 1365181981, 0x1a02816e, 1365183643, 0x1a022fbe),
    (231840, 1366217849, 0x1a022fbe, 1366218134, 0x1a01de94),
    (233856, 1367295455, 0x1a01de94, 1367296471, 0x1a01aa3d),
    (235872, 1368385955, 0x1a01aa3d, 1368386123, 0x1a017fe9),
    (237888, 1369499565, 0x1a017fe9, 1369499746, 0x1a016164),
    (239904, 1370441773, 0x1a016164, 1370442318, 0x1a011337),
    (241920, 1371418407, 0x1a011337, 1371418654, 0x1a00de15),
    (243936, 1372515090, 0x1a00de15, 1372515725, 0x1a00c94e),
    (245952, 1373502151, 0x1a00c94e, 1373502163, 0x1a00a429),
    (247968, 1374514657, 0x1a00a429, 1374515827, 0x1a008968),
    (249984, 1375526943, 0x1a008968, 1375527115, 0x1972dbf2),
    (252000, 1376417294, 0x1972dbf2, 1376417490, 0x19548732),
    (254016, 1377352245, 0x19548732, 1377353319, 0x19415257),
    (256032, 1378268176, 0x19415257, 1378268460, 0x1931679c),
    (258048, 1379202097, 0x1931679c, 1379202248, 0x19262222),
    (260064, 1380117691, 0x19262222, 1380118146, 0x191cdc20),
    (262080, 1381069174, 0x191cdc20, 1381070552, 0x1916b0ca),
    (264096, 1381925718, 0x1916b0ca, 1381925788, 0x19100ab6),
    (266112, 1382754194, 0x19100ab6, 1382754272, 0x190afc85),
    (268128, 1383679776, 0x190afc85, 1383681123, 0x190867f3),
    (270144, 1384695132, 0x190867f3, 1384699499, 0x19070bfb),
    (272160, 1385741656, 0x19070bfb, 1385742648, 0x19061242),
    (274176, 1386684666, 0x19061242, 1386684686, 0x1904ba6e),
    (276192, 1387615098, 0x1904ba6e, 1387617112, 0x1903a30c),
    (278208, 1388624139, 0x1903a30c, 1388624318, 0x1903071f),
    (280224, 1389583107, 0x1903071f, 1389583220, 0x19026666),
    (282240, 1390569911, 0x19026666, 1390570126, 0x1901f52c),
    (284256, 1391582444, 0x1901f52c, 1391584456, 0x1901a36e),
    (286272, 1392597647, 0x1901a36e, 1392597839, 0x19015f53),
    (288288, 1393589930, 0x19015f53, 1393590585, 0x19012026),
    (290304, 1394676535, 0x19012026, 1394676764, 0x190102b1),
    (292320, 1395703577, 0x190102b1, 1395703832, 0x1900db99),
    (294336, 1396693489, 0x1900db99, 1396694478, 0x1900b3aa),
    (296352, 1397755194, 0x1900b3aa, 1397755646, 0x19009d8c),
    (298368, 1398810754, 0x19009d8c, 1398811175, 0x1900896c),
    (300384, 1399904296, 0x1900896c, 1399904311, 0x187c3053),
    (302400, 1400928544, 0x187c3053, 1400928750, 0x18692842),
    (304416, 1402004511, 0x18692842, 1402004993, 0x185d859a),
    (306432, 1403061308, 0x185d859a, 1403061280, 0x1851aba2),
    (308448, 1404029522, 0x1851aba2, 1404029556, 0x18415fd1),
    (310464, 1405203024, 0x18415fd1, 1405205894, 0x183f6be6),
    (312480, 1406325104, 0x183f6be6, 1406325092, 0x183aaea2),
    (314496, 1407473800, 0x183aaea2, 1407474112, 0x1837ba62),
    (316512, 1408474964, 0x1837ba62, 1408475518, 0x182e1c58),
    (318528, 1409527066, 0x182e1c58, 1409527152, 0x182815ee),
    (320544, 1410639387, 0x182815ee, 1410638896, 0x1824dbe9),
    (322560, 1411679882, 0x1824dbe9, 1411680080, 0x181fb893),
    (324576, 1412877894, 0x181fb893, 1412877866, 0x181f6973),
    (326592, 1414054419, 0x181f6973, 1414055393, 0x181e8dc0),
    (328608, 1415154489, 0x181e8dc0, 1415154631, 0x181bc330),
    (330624, 1416343330, 0x181bc330, 1416345124, 0x181b4861),
    (332640, 1417563570, 0x181b4861, 1417563705, 0x181b7b74),
    (334656, 1418790160, 0x181b7b74, 1418791024, 0x181bdd7c),
    (336672, 1419965406, 0x181bdd7c, 1419965588, 0x181b0dca),
    (338688, 1421083565, 0x181b0dca, 1421084073, 0x1819012f),
    (340704, 1422372768, 0x1819012f, 1422372946, 0x181aa3c0),
    (342720, 1423495952, 0x181aa3c0, 1423496415, 0x1818bb87),
    (344736, 1424648263, 0x1818bb87, 1424648937, 0x18178d3a),
    (346752, 1425839583, 0x18178d3a, 1425840165, 0x18172ec0),
    (348768, 1427068149, 0x18172ec0, 1427068411, 0x181788f2),
    (350784, 1428211256, 0x181788f2, 1428211345, 0x18163c71),
    (352800, 1429467587, 0x18163c71, 1429467906, 0x181717f0),
    (354816, 1430676673, 0x181717f0, 1430677341, 0x181713dd),
    (356832, 1431858092, 0x181713dd, 1431858433, 0x181686f5),
    (358848, 1433098989, 0x181686f5, 1433099185, 0x18171a8b),
    (360864, 1434257600, 0x18171a8b, 1434257763, 0x18162043),
    (362880, 1435474473, 0x18162043, 1435475246, 0x1816418e),
    (364896, 1436645194, 0x1816418e, 1436646286, 0x181586c8),
    (366912, 1437828076, 0x181586c8, 1437828285, 0x18150815),
    (368928, 1439028210, 0x18150815, 1439028930, 0x1814dd04),
    (370944, 1440203823, 0x1814dd04, 1440204583, 0x181443c4),
    (372960, 1441356822, 0x181443c4, 1441357507, 0x18134dc1),
    (374976, 1442518636, 0x18134dc1, 1442519404, 0x181287ba),
    (376992, 1443699609, 0x181287ba, 1443700390, 0x18121472),
    (379008, 1444908588, 0x18121472, 1444908751, 0x18120f14),
    (381024, 1446091729, 0x18120f14, 1446092706, 0x1811a954),
];
