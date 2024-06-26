use std::collections::HashMap;
use crate::file_reading::{TickerSymbolData};
use once_cell::sync::Lazy;

pub static TICKER_MAP: Lazy<HashMap<String, TickerSymbolData>> = Lazy::new(|| {
    read_csv()
});
/**
 * Read the CSV file and return a HashMap of TickerSymbolData
 */
pub fn read_csv() -> HashMap<String, TickerSymbolData> {
    let mut map: HashMap<String, TickerSymbolData> = HashMap::new();

    for line in FILE_DATA.lines() {
        let split: Vec<&str> = line.split(',').map(|s| s.trim()).collect(); // Trim each field to remove whitespace
        if split.len() >= 3 { // Ensure there are enough fields
            let ticker_data = TickerSymbolData::new(split[1], split[2]);
            map.insert(split[0].to_string(), ticker_data);
        } else {
            eprintln!("Skipping invalid line: {}", line);
        }
    }
    return map;
}

/**
Symbol, Name, Exchange
 */
pub const FILE_DATA: &str = "
ONE,01 Communique Laboratory Inc.,TSXV
EFF,1844 Resources Inc.,TSXV
AUMB,1911 Gold Corporation,TSXV
FNR,49 North Resources Inc.,TSXV
FIVD.P,5D Acquisition Corp.,TSXV
ALAB.P,A-Labs Capital II Corp.,TSXV
ALCC.P,A-Labs Capital IV Corp.,TSXV
AIS,A.I.S. Resources Limited,TSXV
AZC.P,A2ZCryptocap Inc.,TSXV
AAAJ.P,AAJ Capital 3 Corp.,TSXV
ACCB.P,Aardvark 2 Capital Corp.,TSXV
AME,Abacus Mining & Exploration Corporation,TSXV
ABA,Abasca Resources Inc.,TSXV
ABI,Abcourt Mines Inc.,TSXV
ABM,Aben Minerals Ltd.,TSXV
ABRA,AbraSilver Resource Corp.,TSXV
AXE,Acceleware Ltd.,TSXV
AKR,Ackroo Inc.,TSXV
ADJ.P,AD4 Capital Corp.,TSXV
ADZ,Adamera Minerals Corp.,TSXV
ADE,Adex Mining Inc.,TSXV
AWI,Advent-AWI Holdings Inc.,TSXV
ADZN,Adventus Mining Corporation,TSXV
ADYA,ADYA Inc.,TSXV
ADY,Adyton Resources Corporation,TSXV
AQS,Aequus Pharmaceuticals Inc.,TSXV
AERO,Aero Energy Limited,TSXV
AF.P,AF2 Capital Corp.,TSXV
AFR,AFR NuVenture Resources Inc.,TSXV
AFE,Africa Energy Corp.,TSXV
CUCO,African Energy Metals Inc.,TSXV
AAG,Aftermath Silver Ltd.,TSXV
AGET,AGEDB Technology Ltd.,TSXV
AIVC,AI Artificial Intelligence Ventures Inc.,TSXV
AIME.P,AIM5 Ventures Inc.,TSXV
AIMF.P,AIM6 Ventures Inc.,TSXV
AIP.U,AIP Realty Trust,TSXV
IQ,AirIQ Inc.,TSXV
AISX,Aisix Solutions Inc.,TSXV
AML,Akwaaba Mining Ltd.,TSXV
AEMC,Alaska Energy Metals Corporation,TSXV
ALBT.P,Albatros Acquisition Corporation Inc.,TSXV
ALDD.P,ALDD Ventures Corp.,TSXV
ALDE,Aldebaran Resources Inc.,TSXV
AUAU,Allegiant Gold Ltd.,TSXV
ALM,Alliance Mining Corp.,TSXV
DEX,Almadex Minerals Ltd.,TSXV
AORO,Aloro Mining Corp.,TSXV
ALEX,Alpha Exploration Ltd.,TSXV
AFM,Alphamin Resources Corp.,TSXV
KSUM,Alset Capital Inc.,TSXV
ATI,Altai Resources Inc.,TSXV
AVX,Altair Resources Inc.,TSXV
ALTA,Altamira Gold Corp.,TSXV
ALTR.P,Alterego Ventures 24 Corp.,TSXV
ARH,Altima Resources Ltd.,TSXV
ALTN.P,Altina Capital Corp.,TSXV
APN,Altiplano Metals Inc.,TSXV
ALT,Alturas Minerals Corp.,TSXV
AUUA,Aluula Composites Inc.,TSXV
ALV,Alvopetro Energy Ltd.,TSXV
AL,ALX Resources Corp.,TSXV
AHR,Amarc Resources Ltd.,TSXV
AMRQ,Amaroq Minerals Ltd.,TSXV
AMYA.P,Amaya Big Sky Capital Corp.,TSXV
MEGO.P,Amego Capital Corp.,TSXV
AMK,American Creek Resources Ltd.,TSXV
AE,American Eagle Gold Corp.,TSXV
LI,American Lithium Corp.,TSXV
AMX,Amex Exploration Inc.,TSXV
AMG.P,AMG Acquisition Corp,TSXV
ALY,AnalytixInsight Inc.,TSXV
APM,Andean Precious Metals Corp.,TSXV
EESH.P,Aneesh Capital Corp.,TSXV
AWM,Angel Wing Metals Inc.,TSXV
ANK,Angkor Resources Corp.,TSXV
GUS,Angus Gold Inc.,TSXV
AUNK.P,Ankh II Capital Inc.,TSXV
ANOR,AnorTech Inc.,TSXV
AVII.P,Antera Ventures II Corp.,TSXV
ANTL,Antler Gold Inc.,TSXV
APX,Apex Resources Inc.,TSXV
APMI,Apogee Minerals Ltd.,TSXV
APGO,Apollo Silver Corp.,TSXV
AIV.P,Apolo IV Acquisition Corp.,TSXV
AGT,Applied Graphite Technologies Corporation,TSXV
APL,Appulse Corporation,TSXV
ARJN,Aranjin Resources Ltd.,TSXV
ABR,Arbor Metals Corp.,TSXV
ARCH,Arch Biopartners Inc.,TSXV
ACS,Archon Minerals Limited,TSXV
ARC,ARCpoint Inc.,TSXV
ADD,Arctic Star Exploration Corp.,TSXV
AWX,Arcwest Exploration Inc.,TSXV
LIT,Argentina Lithium & Energy Corp.,TSXV
ASL,Argentum Silver Corp.,TSXV
AROC.P,Argo Opportunity Corp.,TSXV
ART,ARHT Media Inc.,TSXV
DAN,Arianne Phosphate Inc.,TSXV
AZS,Arizona Gold & Silver Inc.,TSXV
ARD,Armada Data Corporation,TSXV
ARK,Arras Minerals Corp.,TSXV
AXL,Arrow Exploration Corp.,TSXV
ARTG,Artemis Gold Inc.,TSXV
AOCC.P,Artrari One Capital Corp.,TSXV
RBZ,Arya Resources Ltd.,TSXV
ABZ,AsiaBaseMetals Inc.,TSXV
SAT,Asian Television Network International Ltd.,TSXV
ATR.P,Aster Acquisition Corp.,TSXV
BAY,Aston Bay Holdings Ltd.,TSXV
ASTR,Astra Exploration Inc.,TSXV
AST,Astron Connect Inc.,TSXV
ACOP,Atacama Copper Corporation,TSXV
ATX,ATEX Resources Inc.,TSXV
SASK,Atha Energy Corp.,TSXV
AAT,ATI Airtest Technologies Inc.,TSXV
ATY,Atico Mining Corporation,TSXV
AEP,Atlas Engineered Products Ltd.,TSXV
ACAP.P,Atlas One Capital Corporation,TSXV
SALT,Atlas Salt Inc.,TSXV
ATOM,Atomic Minerals Corporation,TSXV
AAN,Aton Resources Inc.,TSXV
AUGC,Au Gold Corp.,TSXV
AUK.P,Auka Capital Corp.,TSXV
AUQ,AuQ Gold Mining Inc.,TSXV
ARU,Aurania Resources Ltd.,TSXV
RES,Auric Resources Corp.,TSXV
AU,Aurion Resources Ltd.,TSXV
ACU,Aurora Solar Technologies Inc.,TSXV
ASG,Aurora Spine Corporation,TSXV
ARL,Aurum Lake Mining Corporation,TSXV
ASTN.P,Auston Capital Corp.,TSXV
AGLD,Austral Gold Limited,TSXV
XX,Avante Corp.,TSXV
AVN,Avanti Helium Corp.,TSXV
AVR,Avaron Mining Corp.,TSXV
AVG,Avidian Gold Corp.,TSXV
AVCR,Avricore Health Inc.,TSXV
AVU,Avrupa Minerals Ltd.,TSXV
ARIC,Awale Resources Limited,TSXV
AXET.P,Axe2 Acquisitions Inc.,TSXV
AZR,Azarga Metals Corp.,TSXV
AZM,Azimut Exploration Inc.,TSXV
AAZ,Azincourt Energy Corp.,TSXV
AZT,Aztec Minerals Corp.,TSXV
AMZ,Azucar Minerals Ltd.,TSXV
BP,Backstageplay Inc.,TSXV
YVR.P,Badger Capital Corp.,TSXV
BLDS,Badlands Resources Inc.,TSXV
BLTC.P,Baltic I Acquisition Corp.,TSXV
BNXA,Banxa Holdings Inc.,TSXV
BYN,Banyan Gold Corp.,TSXV
BRO,Barksdale Resources Corp.,TSXV
BGS,Baroyeca Gold & Silver Inc.,TSXV
BARU,Baru Gold Corp.,TSXV
FIND,Baselode Energy Corp.,TSXV
BAT,Batero Gold Corp.,TSXV
BMV,Bathurst Metals Corp.,TSXV
BMR,Battery Mineral Resources Corp.,TSXV
BHS,Bayhorse Silver Inc.,TSXV
BM,BC Moly Ltd.,TSXV
B,BCM Resources Corporation,TSXV
BCM,Bear Creek Mining Corporation,TSXV
BGF,Beauce Gold Fields Inc.,TSXV
BFM,Bedford Metals Corp.,TSXV
BCU,Bell Copper Corporation,TSXV
BEA,Belmont Resources Inc.,TSXV
BMET,Bemetals Corp.,TSXV
BEX,Benton Resources Inc.,TSXV
BZ,Benz Mining Corp.,TSXV
BST,Bessor Minerals Inc.,TSXV
BEW,BeWhere Holdings Inc.,TSXV
BRAU,Big Ridge Gold Corp.,TSXV
BIGT,Big Tree Carbon Inc.,TSXV
STAK.P,Bigstack Opportunities I Inc.,TSXV
BGA,BioNeutra Global Corporation,TSXV
BRM,Biorem Inc.,TSXV
RX,BioSyent Inc.,TSXV
BTCW,Bitcoin Well Inc.,TSXV
BTT,Bitterroot Resources Ltd.,TSXV
BMM,Black Mammoth Metals Corporation,TSXV
SWAN,Black Swan Graphene Inc.,TSXV
BRC,Blackrock Silver Corp.,TSXV
BWCG,Blackwolf Copper and Gold Ltd.,TSXV
BAG,Blende Silver Corp.,TSXV
BITK,Blockchaink2 Corp.,TSXV
MATE,Blockmate Ventures Inc.,TSXV
BKMT,Blockmint Technologies Inc.,TSXV
MOON,Blue Moon Metals Inc.,TSXV
BGE,Blue Sky Global Energy Corp.,TSXV
BSK,Blue Sky Uranium Corp.,TSXV
BAU,Blue Star Gold Corp.,TSXV
BLUE,Blue Thunder Mining Inc.,TSXV
BTV,Bluerush Inc.,TSXV
BSR,Bluestone Resources Inc.,TSXV
BLM,BluMetric Environmental Inc.,TSXV
BMEX,BMEX Gold Inc.,TSXV
BWLK,Boardwalktech Software Corp.,TSXV
BOCA,Bocana Resources Corp.,TSXV
BOLD.P,Bold Capital Enterprises Ltd.,TSXV
BOL,Bold Ventures Inc.,TSXV
BNZ,Bonanza Mining Corporation,TSXV
BTR,BonTerra Resources Inc.,TSXV
BONE,Boron One Holdings Inc.,TSXV
BLCC.P,Bow Lake Capital Corp.,TSXV
BQE,BQE Water Inc.,TSXV
BRCB.P,Brachium2 Capital Corp.,TSXV
BHLI,Bradda Head Lithium Limited,TSXV
BES,Braille Energy Systems Inc.,TSXV
BVA,Bravada Gold Corporation,TSXV
BRVO,Bravo Mining Corp.,TSXV
BRG,Brigadier Gold Limited,TSXV
BBB,Brixton Metals Corporation,TSXV
BRN.PR.A,Brookfield Investments Corporation,TSXV
BRW,Brunswick Exploration Inc.,TSXV
BTU,BTU Metals Corp.,TSXV
BILD,BuildDirect.Com Technologies Inc.,TSXV
BCF,Builders Capital Mortgage Corp.,TSXV
ZLTO,BULGOLD Inc.,TSXV
AMMO,Bullet Exploration Inc.,TSXV
BGD,Bullion Gold Resources Corp.,TSXV
BNKR,Bunker Hill Mining Corp.,TSXV
BUZH.P,Buzz Capital 2 Inc.,TSXV
BUZ.P,Buzz Capital Inc.,TSXV
BWR,BWR Exploration Inc.,TSXV
CMI,C-COM Satellite Systems Inc.,TSXV
CCCM,C3 Metals Inc.,TSXV
CBR,Cabral Gold Inc.,TSXV
CNO,California Nanotechnologies Corp.,TSXV
CNX,Callinex Mines Inc.,TSXV
COR,Camino Minerals Corporation,TSXV
CCB,Canada Carbon Inc.,TSXV
CNC,Canada Nickel Company Inc.,TSXV
CONE,Canada One Mining Corp.,TSXV
LL,Canada Rare Earth Corp.,TSXV
CANB,CanadaBis Capital Inc.,TSXV
CCMI,Canadian Critical Minerals Inc.,TSXV
CGC,Canadian Gold Corp.,TSXV
NET.UN,Canadian Net Real Estate Investment Trust,TSXV
CNRI,Canadian North Resources Inc.,TSXV
CPS,Canadian Premium Sand Inc.,TSXV
SPI,Canadian Spirit Resources Inc.,TSXV
CAF,Canaf Investments Inc.,TSXV
CVV,CanAlaska Uranium Ltd.,TSXV
CEC,Canasia Energy Corp.,TSXV
CAND,Candelaria Mining Corp.,TSXV
CANX,Canex Metals Inc,TSXV
CML,CaNickel Mining Limited,TSXV
NIS.P,Cann-is Capital Corp.,TSXV
RCR.P,Canna 8 Investment Trust,TSXV
LOVE,Cannara Biotech Inc.,TSXV
CLV,Canoe Mining Ventures Corp.,TSXV
CSOC.A,Canso Select Opportunities Corporation,TSXV
ROX,Canstar Resources Inc.,TSXV
CTM,Canterra Minerals Corporation,TSXV
CD,Cantex Mine Development Corp.,TSXV
CDA,Canuc Resources Corporation,TSXV
CYF,Canyon Creek Food Company Ltd.,TSXV
CMIL,Capella Minerals Limited,TSXV
CAI,Capitan Investment Ltd.,TSXV
CAPT,Capitan Silver Corp.,TSXV
CAPL.P,Caplink Ventures Inc.,TSXV
AUTO,Carbeeza Inc.,TSXV
KLX,Carbon Done Right Developments Inc.,TSXV
CT.P,CarbonTech Capital Corp.,TSXV
EKG,\"CardioComm Solutions, Inc.\",TSXV
CRBK,Carebook Technologies Inc.,TSXV
CRB,Cariboo Rose Resources Ltd.,TSXV
CGD,Carlin Gold Corporation,TSXV
RUSH,Carolina Rush Corporation,TSXV
ECR,Cartier Resources Inc.,TSXV
CASA,CASA Minerals Inc.,TSXV
CCD,Cascadero Copper Corporation,TSXV
CAM,Cascadia Minerals Ltd.,TSXV
GLDC,Cassiar Gold Corp.,TSXV
CBAR.P,Castlebar Capital Corp.,TSXV
CSTL.P,Castlecap Capital Inc.,TSXV
CBIT,Cathedra Bitcoin Inc.,TSXV
CVY.P,Cavalry Capital Corp.,TSXV
CBLT,CBLT Inc.,TSXV
CEBI,CE Brands Inc.,TSXV
CZO,Ceapro Inc.,TSXV
CES.P,Celestial Acquisition Corp.,TSXV
CVX,Cematrix Corporation,TSXV
CTA,Centaurus Energy Inc.,TSXV
CTG,Centenario Gold Corp.,TSXV
CIO,Central Iron Ore Limited,TSXV
CTN,Centurion Minerals Ltd.,TSXV
LCE,Century Lithium Corp.,TSXV
CYL,Ceylon Graphite Corp.,TSXV
CFY,CF Energy Corp.,TSXV
OYL,CGX Energy Inc.,TSXV
PERU,Chakana Copper Corp.,TSXV
CBA,Champion Bear Resources Ltd.,TSXV
YES,Char Technologies Ltd.,TSXV
CH,Charbone Hydrogen Corporation,TSXV
NZP,Chatham Rock Phosphate Limited,TSXV
CKG,Chesapeake Gold Corp.,TSXV
CBG,Chibougamau Independent Mines Inc.,TSXV
CCIC.P,Chicane Capital I Corp.,TSXV
CRI,Churchill Resources Inc.,TSXV
CMC,Cielo Waste Solutions Corp.,TSXV
CAC.P,Cinaport Acquisition Corp. III,TSXV
AIR,Clean Air Metals Inc.,TSXV
CTEK,Cleantek Industries Inc.,TSXV
CBLU,Clear Blue Technologies International Inc.,TSXV
CZZ,Cleghorn Minerals Ltd.,TSXV
CEP,Cliffside Capital Ltd.,TSXV
CLIP,Clip Money Inc.,TSXV
CDX,Cloud DX Inc.,TSXV
CMB,CMC Metals Ltd.,TSXV
CNJ.P,CNJ Capital Investments Inc.,TSXV
COCO,Coast Copper Corp.,TSXV
CBV,Cobra Venture Corporation,TSXV
CCPC.P,Coco Pool Corp.,TSXV
CEI,Coelacanth Energy Inc.,TSXV
COHO,Coho Collective Kitchens Inc,TSXV
CBI,Colibri Resource Corporation,TSXV
COLL.P,Collingwood Resources Corp.,TSXV
CAD,Colonial Coal International Corp.,TSXV
CLUS,Colossus Resources Corp.,TSXV
TIE,Coloured Ties Capital Inc.,TSXV
CMU,Comet Industries Ltd.,TSXV
CLIC,Comet Lithium Corporation,TSXV
CMD,Commander Resources Ltd.,TSXV
CCE,Commerce Resources Corp.,TSXV
CVB,Compass Gold Corporation,TSXV
CVI.P,Compass Venture Inc.,TSXV
CAG,Composite Alliance Group Inc.,TSXV
CHS,Comprehensive Healthcare Systems Inc.,TSXV
CN,Condor Resources Inc.,TSXV
COS,Coniagas Battery Metals Inc.,TSXV
CQR,Conquest Resources Limited,TSXV
FFP,Consolidated Firstfund Capital Corp.,TSXV
CLM,Consolidated Lithium Metals Inc.,TSXV
CNST.P,Constellation Capital Corp.,TSXV
CNS,Contagious Gaming Inc.,TSXV
CPAU,Copaur Minerals Inc.,TSXV
CUU,Copper Fox Metals Inc.,TSXV
CPL,Copper Lake Resources Ltd.,TSXV
CRD,Copper Road Resources Inc.,TSXV
CPER,CopperCorp Resources Inc.,TSXV
CUEX,CopperEx Resources Corporation,TSXV
CDB,Cordoba Minerals Corp.,TSXV
CUSN,Cornish Metals Inc.,TSXV
CSO,Corsa Coal Corp.,TSXV
COSA,Cosa Resources Corp.,TSXV
CTH,CoTec Holdings Corp.,TSXV
COV,Covalon Technologies Ltd.,TSXV
CRAN.P,Cranstown Capital Corp,TSXV
CEQ,Criterium Energy Ltd.,TSXV
CRE,Critical Elements Lithium Corporation,TSXV
CWV,Crown Point Energy Inc.,TSXV
CSTR,Cryptostar Corp.,TSXV
CUB,Cubicfarm Systems Corp.,TSXV
WATR,Current Water Technologies Inc.,TSXV
CCII.P,Cuspis Capital II Ltd.,TSXV
CVW,CVW CleanTech Inc.,TSXV
CYBE,\"CyberCatch Holdings, Inc.\",TSXV
CYM,Cymat Technologies Ltd.,TSXV
CYTO,Cytophage Technologies Ltd.,TSXV
DTWO,D2 Lithium Corp.,TSXV
DMR,Damara Gold Corp.,TSXV
DAR,Darelle Online Solutions Inc.,TSXV
DAC,Datable Technology Corporation,TSXV
DM,Datametrex AI Limited,TSXV
DUR.P,Daura Capital Corp.,TSXV
DTEA,DAVIDsTEA Inc.,TSXV
DEC,Decade Resources Ltd.,TSXV
DB,Decibel Cannabis Company Inc.,TSXV
DE,Decisive Dividend Corporation,TSXV
DKL,Decklar Resources Inc.,TSXV
MKT,DeepMarkit Corp.,TSXV
DEFN,Defense Metals Corp.,TSXV
DEF,Defiance Silver Corp.,TSXV
DHB,Delivra Health Brands Inc.,TSXV
DELX,DelphX Capital Markets Inc.,TSXV
DLTA,Delta Resources Limited,TSXV
DBC.P,Departure Bay Capital Corp.,TSXV
DAU,Desert Gold Ventures Inc.,TSXV
DME,Desert Mountain Energy Corp.,TSXV
DSY,Destiny Media Technologies Inc.,TSXV
FARM,Deveron Corp.,TSXV
GSD,Devonian Health Group Inc.,TSXV
DFR,DFR Gold Inc.,TSXV
DGL.P,DGL Investments No.1 Inc,TSXV
DGTL,DGTL Holdings Inc.,TSXV
ADK,Diagnos Inc.,TSXV
DMI,Diamcor Mining Inc.,TSXV
DWS,Diamond Estates Wines & Spirits Inc.,TSXV
DGHI,Digihost Technology Inc.,TSXV
DNO,Dinero Ventures Ltd.,TSXV
DOS,Dios Exploration Inc.,TSXV
DHR,Discovery Harbour Resources Corp.,TSXV
DCY,Discovery-Corp Enterprises Inc.,TSXV
DCOP,District Copper Corp.,TSXV
DMX,District Metals Corp.,TSXV
DG,Dixie Gold Inc.,TSXV
DLC,DLC Holdings Corp.,TSXV
DLP,DLP Resources Inc.,TSXV
DMGI,DMG Blockchain Solutions Inc.,TSXV
DV,Dolly Varden Silver Corporation,TSXV
DAQ.P,Dominus Acquisition Corp.,TSXV
DCMC,Dore Copper Mining Corp.,TSXV
DBG,Doubleview Gold Corp.,TSXV
DRAX.P,Draxos Capital Corp.,TSXV
FLT,Drone Delivery Canada Corp.,TSXV
DVX.P,Drummond Ventures Corp.,TSXV
DRY,Dryden Gold Corp.,TSXV
DGO,Durango Resources Inc.,TSXV
DYG,Dynasty Gold Corp.,TSXV
REE,E-Tech Resources Inc.,TSXV
ETU,E2Gold Inc.,TSXV
ETL,E3 Lithium Ltd.,TSXV
EPL,Eagle Plains Resources Ltd.,TSXV
EAC,Earth Alive Clean Technologies Inc.,TSXV
SPOT,EarthLabs Inc,TSXV
EWK,Earthworks Industries Inc.,TSXV
EAM,East Africa Metals Inc.,TSXV
EW,East West Petroleum Corp.,TSXV
ETF,Eastfield Resources Ltd.,TSXV
EBM,Eastwood Bio-Medical Canada Inc.,TSXV
ECCF.P,ECC Ventures 4 Corp.,TSXV
ECCV.P,ECC Ventures 5 Corp.,TSXV
ECCS.P,ECC Ventures 6 Corp.,TSXV
EOG,Eco (Atlantic) Oil & Gas Ltd.,TSXV
ECM,Ecolomondo Corporation,TSXV
EDY,Eddy Smart Home Solutions Ltd.,TSXV
CTRL,Edge Total Intelligence Inc.,TSXV
YFI,Edgewater Wireless Systems Inc.,TSXV
EDDY,Edison Lithium Corp.,TSXV
EDM,EDM Resources Inc.,TSXV
EGR,EGR Exploration Ltd.,TSXV
EGT,Eguana Technologies Inc.,TSXV
ERA,Elcora Advanced Materials Corp.,TSXV
ELBM,Electra Battery Materials Corporation,TSXV
EML,Electric Metals (USA) Limited,TSXV
ELEC,Electric Royalties Ltd.,TSXV
ELY,Electrum Discovery Corp.,TSXV
ECU,Element 29 Resources Inc.,TSXV
ELM,Element Lifestyle Retirement Inc.,TSXV
ELE,Elemental Altus Royalties Corp.,TSXV
ELVT,Elevation Gold Mining Corporation,TSXV
ELC,Elysee Development Corp.,TSXV
ECOM,Emerge Commerce Ltd.,TSXV
EMR,Emergent Metals Corp.,TSXV
EMO,Emerita Resources Corp.,TSXV
EMNT,Eminent Gold Corp.,TSXV
EMPR,Empress Royalty Corp.,TSXV
EMX,EMX Royalty Corporation,TSXV
ENA,Enablence Technologies Inc.,TSXV
EU,Encore Energy Corp.,TSXV
ECAP.P,Endurance Capital Corp.,TSXV
EDG,Endurance Gold Corporation,TSXV
ENDR,Enduro Metals Corporation,TSXV
ENEV,Enerev5 Metals Inc.,TSXV
EAU,Engineer Gold Mines Ltd.,TSXV
EGM,Engold Mines Ltd.,TSXV
ENTG,Entourage Health Corp.,TSXV
EWS,Environmental Waste International Inc.,TSXV
ENW,EnWave Corporation,TSXV
EON,EON Lithium Corp.,TSXV
EQ,EQ Inc.,TSXV
EQTY,Equity Metals Corporation,TSXV
ERC,Eros Resources Corp.,TSXV
ESE,ESE Entertainment Inc.,TSXV
ESK,Eskay Mining Corp.,TSXV
ESS,Esstra Industries Inc.,TSXV
EBCD.P,Eureka Capital Corp.,TSXV
EMN,Euro Manganese Inc.,TSXV
EOX,Euromax Resources Ltd.,TSXV
EUP,Europacific Metals Inc.,TSXV
EVX,European Electric Metals Inc.,TSXV
FIN,European Energy Metals Corp.,TSXV
EVNI,EV Nickel Inc.,TSXV
EVGN,Evergen Infrastructure Corp.,TSXV
EVER,Evergold Corp.,TSXV
ELL,Everybody Loves Languages Corp.,TSXV
EPF,Everyday People Financial Corp.,TSXV
EVOC.P,Evocati Capital Resources Inc.,TSXV
OKAI,EvokAI Creative Labs Inc.,TSXV
EVMT,Evome Medical Technologies Inc.,TSXV
EVP.P,EVP Capital Inc.,TSXV
XCAP.P,Exelerate Capital Corp.,TSXV
EXG,ExGen Resources Inc.,TSXV
FUU,F3 Uranium Corp.,TSXV
FBF,Fab-Form Industries Ltd.,TSXV
FINV.P,Faction Investment Group Corp.,TSXV
FAIR,Fairchild Gold Corp.,TSXV
FPY.P,Fairplay Ventures Inc.,TSXV
FPC,Falco Resources Ltd.,TSXV
FG,Falcon Gold Corp.,TSXV
FO,Falcon Oil & Gas Ltd.,TSXV
FNC,Fancamp Exploration Ltd.,TSXV
FRS.P,Farstarcap Investment Corp.,TSXV
FMN,Fidelity Minerals Corp.,TSXV
FFC.P,Fife Capital Corp.,TSXV
FDI,Findev Inc.,TSXV
FYL,Finlay Minerals Ltd.,TSXV
FTEC,Fintech Select Ltd.,TSXV
FFOX,Firefox Gold Corp.,TSXV
FWZ,Fireweed Metals Corp.,TSXV
FCA.DB,Firm Capital Apartment Real Estate Investment Trust,TSXV
FGCC.P,First and Goal Capital Corp.,TSXV
FAS,First Andes Silver Ltd.,TSXV
FAN,First Atlantic Nickel Corp.,TSXV
HELI,First Helium Inc.,TSXV
FHYD,First Hydrogen Corp.,TSXV
FNM,First Nordic Metals Corp.,TSXV
AAA.P,First Tidal Acquisition Corp.,TSXV
FTZ,Fitzroy Minerals Inc.,TSXV
FEX,Fjordland Exploration Inc.,TSXV
FONC.P,Florence One Capital Inc.,TSXV
FW,Flow Capital Corp.,TSXV
FLY,FLYHT Aerospace Solutions Ltd.,TSXV
FLYN,Flying Nickel Mining Corp.,TSXV
FOBI,Fobi AI Inc.,TSXV
FMS,Focus Graphite Inc.,TSXV
FKM,Fokus Mining Corporation,TSXV
FTJ,Fort St. James Nickel Corp.,TSXV
FOR,Fortune Bay Corp.,TSXV
FMC,Forum Energy Metals Corp.,TSXV
FWTC,Forward Water Technologies Corp.,TSXV
FDR,Founders Metals Inc.,TSXV
FA,Fountain Asset Corp.,TSXV
FUN.P,Fountainhall Capital Corp.,TSXV
AROW.P,Four Arrows Capital Corp.,TSXV
FP,FP Newspapers Inc.,TSXV
FPX,FPX Nickel Corp.,TSXV
FMAC.P,Fraser Mackenzie Accelerator Corp.,TSXV
FRED,Fredonia Mining Inc.,TSXV
FMAN,Freeman Gold Corp.,TSXV
FRI,Freeport Resources Inc.,TSXV
FRE,Fremont Gold Ltd.,TSXV
FREQ,Frequency Exchange Corp.,TSXV
FRNT,FRNT Financial Inc.,TSXV
FL,Frontier Lithium Inc.,TSXV
FRXI,FRX Innovations Inc.,TSXV
FTI,FTI Foodtech International Inc.,TSXV
NHHH,FuelPositive Corporation,TSXV
FCLI,Full Circle Lithium Corp.,TSXV
FMM,Full Metal Minerals Ltd.,TSXV
FUSE,Fuse Battery Metals Inc.,TSXV
GETT,G.E.T.T. Gold Inc.,TSXV
GTM.P,G2M Cap Corp.,TSXV
GGG,G6 Materials Corp.,TSXV
GAB,Gabo Mining Ltd.,TSXV
GBU,Gabriel Resources Ltd.,TSXV
GAL,Galantas Gold Corporation,TSXV
GXY.P,Galaxy Ventures Inc.,TSXV
GGO,Galleon Gold Corp.,TSXV
GRI,Galore Resources Inc.,TSXV
GWM,Galway Metals Inc.,TSXV
GGI,Garibaldi Resources Corp.,TSXV
GSI,Gatekeeper Systems Inc.,TSXV
GKO,Geekco Technologies Corporation,TSXV
GA,General Assembly Holdings Limited,TSXV
GEN,Generation Uranium Inc.,TSXV
REBL.P,Genesis Acquisition Corp.,TSXV
GNFI,Genifi Inc.,TSXV
GENI,Genius Metals Inc.,TSXV
GENX,Genix Pharmaceuticals Corporation,TSXV
GSP,Gensource Potash Corporation,TSXV
GMA,Geomega Resources Inc.,TSXV
GTC,Getty Copper Inc.,TSXV
GFG,GFG Resources Inc.,TSXV
GGL,GGL Resources Corp.,TSXV
GGX,GGX Gold Corp.,TSXV
GIGA,Giga Metals Corporation,TSXV
GOK,GINSMS Inc.,TSXV
GIT,Gitennes Exploration Inc.,TSXV
EMM,Giyani Metals Corp.,TSXV
GLI,Glacier Lake Resources Inc.,TSXV
GLAD,Gladiator Metals Corp.,TSXV
GBML,Global Battery Metals Ltd.,TSXV
GEMC,Global Energy Metals Corporation,TSXV
GMV,GMV Minerals Inc.,TSXV
GG,Golconda Gold Ltd.,TSXV
GXX,Gold Basin Resources Corporation,TSXV
GBRC,Gold Bull Resources Corp.,TSXV
GRZ,Gold Reserve Inc.,TSXV
YGT,Gold Terra Resource Corp.,TSXV
AUU,Gold79 Mines Ltd.,TSXV
GLB,Goldbank Mining Corporation,TSXV
GCN,Goldcliff Resource Corporation,TSXV
GRG,Golden Arrow Resources Corporation,TSXV
GNG,Golden Goliath Resources Ltd.,TSXV
GHML,Golden Horse Minerals Limited,TSXV
GDP,Golden Pursuit Resources Ltd.,TSXV
GLDN,Golden Ridge Resources Ltd.,TSXV
GSH,Golden Share Resources Corporation,TSXV
AUEN,Golden Sky Minerals Corp.,TSXV
GCV.P,Golden Star Capital Ventures Inc.,TSXV
GDX,Goldex Resources Corporation,TSXV
GOFL,Goldflare Exploration Inc.,TSXV
GGA,Goldgroup Mining Inc.,TSXV
GHL,Goldhills Holding Ltd.,TSXV
GLD,GoldON Resources Ltd.,TSXV
GQC,GoldQuest Mining Corp.,TSXV
GSHR,Goldshore Resources Inc.,TSXV
GXS,Goldsource Mines Inc.,TSXV
GSTM,Goldstorm Metals Corp.,TSXV
GOT,Goliath Resources Limited,TSXV
GOOD,Good Gamer Entertainment Inc.,TSXV
GDNP,Good Natured Products Inc.,TSXV
GFOR.P,Good2Go4 Corp.,TSXV
GODB.P,Goodbridge Capital Corp.,TSXV
GSS,Gossan Resources Limited,TSXV
GXU,GoviEx Uranium Inc.,TSXV
GWA,Gowest Gold Ltd.,TSXV
GPM,GPM Metals Inc.,TSXV
GRSL,GR Silver Mining Ltd.,TSXV
GGM,Granada Gold Mine Inc.,TSXV
GPG,Grande Portage Resources Ltd.,TSXV
GCX,Granite Creek Copper Ltd.,TSXV
GEL,Graphano Energy Ltd.,TSXV
GMG,Graphene Manufacturing Group Ltd.,TSXV
GPH,Graphite One Inc.,TSXV
GRAT,Gratomic Inc.,TSXV
GR,Great Atlantic Resources Corp.,TSXV
GPAC,Great Pacific Gold Corp.,TSXV
GQ,Great Quest Fertilizer Ltd.,TSXV
GEM,Green Battery Minerals Inc.,TSXV
GIP,Green Impact Partners Inc.,TSXV
GPCC.P,Green Panda Capital Corp.,TSXV
GRF,Green Rise Foods Inc.,TSXV
GCOM,Green Shift Commodities Ltd.,TSXV
GRB,Greenbriar Sustainable Living Inc.,TSXV
VGN,Greencastle Resources Ltd.,TSXV
GPV,GreenPower Motor Company Inc.,TSXV
WOLF,Grey Wolf Animal Health Corp.,TSXV
CELL,Grid Battery Metals Inc.,TSXV
GRDM,Grid Metals Corp.,TSXV
GZD,Grizzly Discoveries Inc.,TSXV
GRVA.P,Grosvenor CPC I Inc.,TSXV
GVR,Grosvenor Resource Corporation,TSXV
GRD,Grounded Lithium Corp.,TSXV
ZNG,Group Eleven Resources Corp.,TSXV
GSPR,GSP Resource Corp.,TSXV
GT,GT Resources Inc.,TSXV
GSVR,Guanajuato Silver Company Ltd.,TSXV
GX,Guardian Exploration Inc.,TSXV
GUF,Gulf & Pacific Equities Corp.,TSXV
GUG,Gungnir Resources Inc.,TSXV
GUN,Gunpoint Exploration Ltd.,TSXV
HO.P,H2 Ventures 1 Inc.,TSXV
HAKK.P,Hakken Capital Corp.,TSXV
HPM,Halcones Precious Metals Corp.,TSXV
HMT,Halmont Properties Corporation,TSXV
HFC,Hampton Financial Corporation,TSXV
HANK,Hank Payments Corp.,TSXV
HAN,Hannan Metals Ltd.,TSXV
HCO.P,Hansco Capital Corp.,TSXV
HANS,Hanstone Gold Corp.,TSXV
HAPB,\"Hapbee Technologies, Inc.\",TSXV
HPY,Happy Creek Minerals Ltd.,TSXV
HAR,Harfang Exploration inc.,TSXV
MONY.P,Harmony Acquisitions Corp.,TSXV
HVG,Harvest Gold Corp.,TSXV
HEC.P,Haviland Enviro Corp.,TSXV
HAW.P,HAW Capital 2 Corp.,TSXV
HAWK,Hawkeye Gold & Diamond Inc.,TSXV
HSTR,Heliostar Metals Ltd.,TSXV
HEVI,Helium Evolution Incorporated,TSXV
HHH,Helius Minerals Limited,TSXV
HME,Hemisphere Energy Corporation,TSXV
HMLO,Hemlo Explorers Inc.,TSXV
HEM,Hemostemix Inc.,TSXV
HEMP,Hempalta Corp.,TSXV
BIG,Hercules Silver Corp.,TSXV
HMCC.P,High Mountain 2 Capital Corporation,TSXV
HITI,High Tide Inc.,TSXV
HBK,Highbank Resources Ltd.,TSXV
HIGH,Highgold Mining Inc.,TSXV
HI,Highland Copper Company Inc.,TSXV
HWY,Highway 50 Gold Corp.,TSXV
HAM,Highwood Asset Management Ltd.,TSXV
HILL,Hill Incorporated,TSXV
ESPN,Hispania Resources Inc.,TSXV
HIVE,HIVE Digital Technologies Ltd.,TSXV
SHL,Homeland Nickel Inc.,TSXV
HMR,Homerun Resources Inc.,TSXV
TUF,Honey Badger Silver Inc.,TSXV
HVII.P,Hopefield Ventures Two Inc.,TSXV
HCU,Horizon Copper Corp.,TSXV
HRC.P,Hoshi Resource Corp.,TSXV
HCH,Hot Chili Limited,TSXV
HPQ,HPQ Silicon Inc.,TSXV
HTC,HTC Purenergy Inc.,TSXV
HMAN,Huntsman Exploration Inc.,TSXV
HIDE.P,Hydaway Ventures Corp.,TSXV
NURS,Hydreight Technologies Inc,TSXV
HC,Hypercharge Networks Corp.,TSXV
IB,IBC Advanced Alloys Corp.,TSXV
ICRS,Icarus Capital Corp.,TSXV
ICM,Iconic Minerals Ltd.,TSXV
ICWY.P,ICWHY Capital Ventures Inc.,TSXV
ID,Identillect Technologies Corp.,TSXV
IRI,IEMR Resources Inc.,TSXV
IKC.P,Ikigai Capital Corp.,TSXV
IFX,Imaflex Inc.,TSXV
ILI,Imagine Lithium Inc.,TSXV
IDL,Imaging Dynamics Company Ltd.,TSXV
IMR,iMetal Resources Inc.,TSXV
IMPC.P,Impact Acquisitions Corp.,TSXV
IPT,IMPACT Silver Corp.,TSXV
IEI,Imperial Equities Inc.,TSXV
IGP,Imperial Ginseng Products Ltd.,TSXV
INCA,Inca One Gold Corp.,TSXV
IGO,Independence Gold Corp.,TSXV
IXI,Indigo Exploration Inc.,TSXV
NDVA,Indiva Limited,TSXV
INEO,INEO Tech Corp.,TSXV
INFD,Infield Minerals Corp.,TSXV
INFM,Infinico Metals Corp.,TSXV
INFI,Infinitum Copper Corp.,TSXV
IOT,Innovotech Inc.,TSXV
MINE,Inomin Mines Inc.,TSXV
INSP,Inspire Semiconductor Holdings Inc.,TSXV
ISGI,Insuraguest Technologies Inc.,TSXV
ITR,Integra Resources Corp.,TSXV
IRO,Inter-Rock Minerals Inc.,TSXV
IFR,International Frontier Resources Corporation,TSXV
ICON,International Iconic Gold Exploration Corp.,TSXV
ILC,International Lithium Corp.,TSXV
IMM,International Metals Mining Corp.,TSXV
IPD,International Parkside Products Inc.,TSXV
IZZ,International Prospect Ventures Ltd.,TSXV
IZ,International Zeolite Corp.,TSXV
TIDE.P,Intertidal Capital Corp.,TSXV
INX,Intouch Insight Ltd.,TSXV
INTR,Intrepid Metals Corp.,TSXV
IVX,Inventronics Limited,TSXV
IVS,Inventus Mining Corp.,TSXV
IZN,InZinc Mining Ltd.,TSXV
ICY.P,Iocaste Ventures Inc.,TSXV
ISO,IsoEnergy Ltd.,TSXV
IFOS,Itafos Inc.,TSXV
JJJJ.P,J4 Ventures Inc.,TSXV
JAB.P,Jabbo Capital Corp.,TSXV
JNH,Jack Nathan Medical Corp.,TSXV
JJ,Jackpot Digital Inc.,TSXV
JADE,Jade Leader Corp.,TSXV
JG,Japan Gold Corp.,TSXV
JPIM,Jasper Commerce Inc.,TSXV
JAX,Jaxon Mining Inc.,TSXV
JDN,Jayden Resources Inc.,TSXV
JTC,Jemtec Inc.,TSXV
JEV,Jericho Energy Ventures Inc.,TSXV
JRV,Jervois Global Limited,TSXV
JES.P,Jesmond Capital Ltd.,TSXV
JOJO.P,Jo-Jo Capital Canada Ltd.,TSXV
JUB,Jubilee Gold Exploration Ltd.,TSXV
JUGR,Juggernaut Exploration Ltd.,TSXV
JEC,Jura Energy Corporation,TSXV
JVR.P,JVR Ventures Inc.,TSXV
JZR,JZR Gold Inc.,TSXV
KTO,K2 Gold Corporation,TSXV
KNC,K9 Gold Corp.,TSXV
KDSX,Kadestone Capital Corp.,TSXV
KALM.P,Kalma Capital Corp.,TSXV
KALO,Kalo Gold Corp.,TSXV
KAC.P,Kalon Acquisition Corp.,TSXV
KNE,Kane Biotech Inc.,TSXV
KAPA,Kapa Gold Inc.,TSXV
FUND,Katipult Technology Corp.,TSXV
KDA,KDA Group Inc.,TSXV
KKL.P,Kelly Ventures Ltd.,TSXV
KLD,Kenorland Minerals Ltd.,TSXV
KLM,Kermode Resources Ltd.,TSXV
KES,Kesselrun Resources Ltd.,TSXV
KGC,Kestrel Gold Inc.,TSXV
KIB,Kiboko Gold Inc.,TSXV
KIDZ,Kidoz Inc.,TSXV
KCC,Kincora Copper Limited,TSXV
KING,King Global Ventures Inc.,TSXV
KFR,Kingfisher Metals Corp.,TSXV
KGS,Kingman Minerals Ltd.,TSXV
KNG,Kingsmen Resources Ltd.,TSXV
KTR,Kintavar Exploration Inc.,TSXV
KIP,Kiplin Metals Inc.,TSXV
KLDC,Kirkland Lake Discoveries Corp.,TSXV
KG,Klondike Gold Corp.,TSXV
KS,Klondike Silver Corp.,TSXV
KRI,Kobo Resources Inc.,TSXV
KDK,Kodiak Copper Corp.,TSXV
KTRI,Kootenay Resources Inc.,TSXV
KTN,Kootenay Silver Inc.,TSXV
KORE,Kore Mining Ltd.,TSXV
KRY,Koryx Copper Inc.,TSXV
KOVO,Kovo HealthTech Corporation,TSXV
KPEN.P,KP3993 Resources Inc.,TSXV
PNG,Kraken Robotics Inc.,TSXV
KUAI.P,Kua Investments Inc.,TSXV
KBRA,Kubera Gold Corp.,TSXV
KC,Kutcho Copper Corp.,TSXV
KWE,KWESST Micro Systems Inc.,TSXV
LAB,Labrador Gold Corp.,TSXV
LTX,Labrador Resources Inc.,TSXV
LG,Lahontan Gold Corp.,TSXV
LVG,Lake Victoria Gold Ltd.,TSXV
LWR,Lake Winn Resources Corp.,TSXV
LRT.UN,Lanesborough Real Estate Investment Trust,TSXV
LRA,Lara Exploration Ltd.,TSXV
VAND,Largo Physical Vanadium Corp,TSXV
LMS,Latin Metals Inc.,TSXV
LME,Laurion Mineral Exploration Inc.,TSXV
LGC,Lavras Gold Corp.,TSXV
LDB.P,LDB Capital Corp.,TSXV
LEM,Leading Edge Materials Corp.,TSXV
LFC.P,Left Field Capital Corp.,TSXV
LPS,Legend Power Systems Inc.,TSXV
LTV,LeoNovus Inc.,TSXV
LVX,Leviathan Gold Ltd.,TSXV
LIFT,Li-FT Power Ltd.,TSXV
LILI,Li3 Lithium Corp.,TSXV
LBC,Libero Copper & Gold Corporation,TSXV
SCAN,\"Liberty Defense Holdings, Ltd.\",TSXV
LFST,Lifeist Wellness Inc.,TSXV
LMG,Lincoln Gold Mining Inc.,TSXV
LEO,Lion Copper and Gold Corp.,TSXV
LIO,Lion One Metals Limited,TSXV
ROAR,Lion Rock Resources Inc.,TSXV
LBI,Lions Bay Capital Inc.,TSXV
LTE,Lite Access Technologies Inc.,TSXV
LITH,Lithium Chile Inc.,TSXV
LEXI,Lithium Energi Exploration Inc.,TSXV
ION,Lithium ION Energy Ltd.,TSXV
LTH,Lithium Ionic Corp.,TSXV
LONE,Lithium One Metals Inc.,TSXV
LIS,Lithium South Development Corporation,TSXV
LBNK,LithiumBank Resources Corp.,TSXV
LILL.P,Little Fish Acquisition I Corp.,TSXV
LNGE,LNG Energy Group Corp.,TSXV
LOD,Lode Gold Resources Inc.,TSXV
LSTR,Lodestar Battery Metals Corp,TSXV
LGN,Logan Energy Corp.,TSXV
LOG.P,Logica Ventures Corp.,TSXV
LMR,Lomiko Metals Inc.,TSXV
LEX,Longhorn Exploration Corp.,TSXV
LPC,Lorne Park Capital Partners Inc.,TSXV
LA,Los Andes Copper Ltd.,TSXV
LQWD,LQWD Technologies Corp.,TSXV
LSL,LSL Pharma Group Inc.,TSXV
LUCA,Luca Mining Corp.,TSXV
LOU,Lucero Energy Corp.,TSXV
LUM,Lumina Gold Corp.,TSXV
LMN,Lumine Group Inc.,TSXV
LPK,Lupaka Gold Corp.,TSXV
LCX,Lycos Energy Inc.,TSXV
MCT.P,M3 Capital Corp.,TSXV
MT,M3 Metals Corp.,TSXV
MMS,Macarthur Minerals Limited,TSXV
BMK,MacDonald Mines Exploration Ltd.,TSXV
MDM,Madoro Metals Corp.,TSXV
NICU,Magna Mining Inc.,TSXV
MTT,Magna Terra Minerals Inc.,TSXV
MNC,Magnetic North Acquisition Corp.,TSXV
MGI,Magnum Goldcorp Inc.,TSXV
MJS,Majestic Gold Corp.,TSXV
MKO,Mako Mining Corp.,TSXV
MTH,Mammoth Resources Corp.,TSXV
MAN.P,Mandala Capital Inc.,TSXV
MAND.P,Mandeville Ventures Inc.,TSXV
MN,Manganese X Energy Corp.,TSXV
MNX,Manitex Capital Inc.,TSXV
MGM,Maple Gold Mines Ltd.,TSXV
MAP,Maple Peak Investments Inc.,TSXV
DIA,Margaret Lake Diamonds Inc.,TSXV
MAE,Maritime Resources Corp.,TSXV
MAH,Marksmen Energy Inc.,TSXV
MTLO,Martello Technologies Group Inc.,TSXV
MRVL,Marvel Biosciences Corp.,TSXV
MARV,Marvel Discovery Corp.,TSXV
MAR.UN,Marwest Apartment Real Estate Investment Trust,TSXV
MAS,Mas Gold Corp.,TSXV
MASS,Masivo Silver Corp.,TSXV
LLG,Mason Resources Inc.,TSXV
MCM.A,\"Matachewan Consolidated Mines, Limited\",TSXV
MAW,Mawson Gold Limited,TSXV
MAX,MAX Resource Corp.,TSXV
MFA.P,Mayfair Acquisition Corporation,TSXV
MFG,Mayfair Gold Corp.,TSXV
MCS,McChip Resources Inc.,TSXV
MCF,MCF Energy Ltd.,TSXV
MDK.P,MDK Acquisition Inc.,TSXV
MPH,Medicure Inc.,TSXV
MDCX,Medicus Pharma Ltd.,TSXV
MIR,MedMira Inc.,TSXV
MDX,MedX Health Corp.,TSXV
MEED.P,Meed Growth Corp.,TSXV
MKR,Melkior Resources Inc.,TSXV
MENE,Mene Inc.,TSXV
MRKI.P,\"Meraki Acquisition One, Inc.\",TSXV
MERG,Metal Energy Corp.,TSXV
MLO,Metalero Mining Corp.,TSXV
MTX,Metalex Ventures Ltd.,TSXV
MTA,Metalla Royalty & Streaming Ltd.,TSXV
MMG,Metallic Minerals Corp.,TSXV
MTS,Metallis Resources Inc.,TSXV
MET,Metalore Resources Limited,TSXV
MQM,Metalquest Mining Inc.,TSXV
MEK,Metals Creek Resources Corp.,TSXV
MEX,Mexican Gold Mining Corp.,TSXV
MCCP.P,Michichi Capital Corp.,TSXV
MD,Midland Exploration Inc.,TSXV
MMA,Midnight Sun Mining Corp.,TSXV
MEEC,Midwest Energy Emissions Corp,TSXV
MILL,Millbank Mining Corp.,TSXV
MLP,Millennial Potash Corp.,TSXV
MPM.WT,Millennial Precious Metals Corp.,TSXV
MSC,Millennium Silver Corp.,TSXV
MIM,MiMedia Holdings Inc.,TSXV
MSP,Minaean SP Construction Corp.,TSXV
MGG,Minaurum Gold Inc.,TSXV
MMM,Minco Capital Corp.,TSXV
MHUB,Minehub Technologies Inc.,TSXV
MAI,Minera Alamos Inc.,TSXV
MHI,Mineral Hill Industries Ltd.,TSXV
MNLX,Miniluxe Holding Corp.,TSXV
MINK,Mink Ventures Corporation,TSXV
MCI,Minnova Corp.,TSXV
MSR,Minsud Resources Corp.,TSXV
MRZ,Mirasol Resources Ltd.,TSXV
MIZA.P,Miza III Ventures Inc.,TSXV
MKA,Mkango Resources Ltd.,TSXV
MBO,Mobio Technologies Inc.,TSXV
EIRE.P,Monaghan Capital Fund Ltd.,TSXV
MMN,Monarca Minerals Inc.,TSXV
MONA.P,Monarch West Ventures Inc.,TSXV
YAK,Mongolia Growth Group Ltd.,TSXV
MAU,Montage Gold Corp.,TSXV
MTK,Montauk Metals Inc.,TSXV
MON,Montero Mining and Exploration Ltd.,TSXV
MONT,Montfort Capital Corp.,TSXV
MMY,Monument Mining Limited,TSXV
MNRG,Monumental Energy Corp.,TSXV
MOO,Moon River Moly Ltd.,TSXV
MVY,Moovly Media Inc.,TSXV
MOX,Morien Resources Corp.,TSXV
MTB,MTB Metals Corp.,TSXV
MUN,Mundoro Capital Inc.,TSXV
MUR,Murchison Minerals Ltd.,TSXV
MGRO,MustGrow Biologics Corp.,TSXV
NMI,Namibia Critical Metals Inc.,TSXV
CTZ,Namsys Inc.,TSXV
NSCI,Nanalysis Scientific Corp.,TSXV
NAV,NAVCO Pharmaceuticals Inc.,TSXV
NAQ.P,Navigator Acquisition Corp.,TSXV
NVN.P,Navion Capital II Inc.,TSXV
NBM,NEO Battery Materials Ltd.,TSXV
NTX,NeoTerrex Minerals Inc.,TSXV
NDA,Neptune Digital Assets Corp.,TSXV
NGEN,NervGen Pharma Corp.,TSXV
NTE,Network Media Group Inc.,TSXV
NPTH,Neupath Health Inc.,TSXV
NGE,Nevada Exploration Inc.,TSXV
NKG,Nevada King Gold Corp.,TSXV
NEV,Nevada Sunrise Metals Corporation,TSXV
NAU,Nevgold Corp.,TSXV
NAM,New Age Metals Inc.,TSXV
NED,New Destiny Mining Corp.,TSXV
ENRG,New Energy Metals Corp.,TSXV
NFG,New Found Gold Corp.,TSXV
NEME.P,New Media Capital 2.0 Inc.,TSXV
NSE,New Stratus Energy Inc.,TSXV
NTB,New Tymbal Resources Ltd.,TSXV
NZ,New Zealand Energy Corp.,TSXV
NCAU,Newcore Gold Ltd.,TSXV
NEWO,NewOrigin Gold Corp.,TSXV
NWX,Newport Exploration Ltd.,TSXV
NEXE,Nexe Innovations Inc.,TSXV
NGY,Nexera Energy Inc.,TSXV
NXG,NexgenRx Inc.,TSXV
NXLV,NexLiving Communities Inc.,TSXV
NHT.U,NexPoint Hospitality Trust,TSXV
NXH,Next Hydrogen Solutions Inc.,TSXV
NXS,Nexus Gold Corp.,TSXV
GASX,NG Energy International Corp.,TSXV
NICN,NICAN Limited,TSXV
NKL,Nickel 28 Capital Corp.,TSXV
NNX,Nickel North Exploration Corp.,TSXV
NICK,Nickelex Resource Corporation,TSXV
NIM,Nicola Mining Inc.,TSXV
NIO,Nio Strategic Metals Inc.,TSXV
NBY,Niobay Metals Inc.,TSXV
NLII.P,NL2 Capital Inc.,TSXV
NOAL,NOA Lithium Brines Inc,TSXV
NBLC,Nobel Resources Corp.,TSXV
NOB,Noble Mineral Exploration Inc.,TSXV
NRM,Noram Lithium Corp.,TSXV
NTH,Nord Precious Metals Mining Inc.,TSXV
NOCR,Norden Crown Metals Corporation,TSXV
NORR,Norrland Gold Corp.,TSXV
NVT,Nortec Minerals Corp.,TSXV
NAR,North Arrow Minerals Inc.,TSXV
NPR,North Peak Resources Ltd.,TSXV
NSU,North Shore Uranium Ltd.,TSXV
NGC,Northern Graphite Corporation,TSXV
NL,Northern Lion Gold Corp.,TSXV
NRN,Northern Shield Resources Inc.,TSXV
SUP,Northern Superior Resources Inc.,TSXV
NFD.A,Northfield Capital Corporation,TSXV
NCX,NorthIsle Copper and Gold Inc.,TSXV
ROOF,Northstar Clean Technologies Inc.,TSXV
BET,NorthStar Gaming Holdings Inc.,TSXV
NWST,Northwest Copper Corp.,TSXV
NOU,Nouveau Monde Graphite Inc.,TSXV
NLH,Nova Leap Health Corp.,TSXV
NVI,Novra Technologies Inc.,TSXV
NOW,NowVertical Group Inc.,TSXV
NCI,NTG Clarity Networks Inc.,TSXV
NBVA,Nubeva Technologies Ltd.,TSXV
NBR,Nubian Resources Ltd.,TSXV
NGMD,Nugen Medical Devices Inc.,TSXV
NUG,NuLegacy Gold Corporation,TSXV
NRX,NurExone Biologic Inc.,TSXV
NVX,NV Gold Corporation,TSXV
OIII,O3 Mining Inc.,TSXV
OAC.P,Oa Capital Corp.,TSXV
OCAP.P,Ocean Shore Capital Corp.,TSXV
FEO,Oceanic Iron Ore Corp.,TSXV
OSIX,Oceansix Future Paths Ltd.,TSXV
OVT,Oculus VisionTech Inc.,TSXV
OTC,Ocumetics Technology Corp.,TSXV
ODD,Odd Burger Corporation,TSXV
ALFA.P,Odessa Capital Ltd.,TSXV
OC,Olive Resource Capital Inc.,TSXV
OLV,Olivut Resources Ltd.,TSXV
OMG,Omai Gold Mines Corp.,TSXV
OMM,Omineca Mining and Metals Ltd.,TSXV
OML,Omni-Lite Industries Canada Inc.,TSXV
OSS,OneSoft Solutions Inc.,TSXV
ONAU,OnGold Resources Ltd.,TSXV
ONYX,Onyx Gold Corp.,TSXV
OPW,Opawica Explorations Inc.,TSXV
OPEN.P,OpenSesame Acquisition Corp.,TSXV
OPHR,Ophir Gold Corp.,TSXV
OPTG,Optegra Ventures Inc.,TSXV
OOR,Opus One Gold Corporation,TSXV
ORCL,Oracle Commodity Holding Corp.,TSXV
ORC.A,Orca Energy Group Inc.,TSXV
OCI,Orecap Invest Corp.,TSXV
ORS,Orestone Mining Corp.,TSXV
REX,Orex Minerals Inc.,TSXV
OGO,Organto Foods Inc.,TSXV
OCO,Oroco Resource Corp.,TSXV
OGN,Orogen Royalties Inc.,TSXV
OMI,Orosur Mining Inc.,TSXV
OSI,Osino Resources Corp.,TSXV
ODV,Osisko Development Corp.,TSXV
OM,Osisko Metals Incorporated,TSXV
COO,Ostrom Climate Solutions Inc.,TSXV
OZ,Outback Goldfields Corp.,TSXV
OCG,Outcrop Silver & Gold Corporation,TSXV
OAM,OverActive Media Corp.,TSXV
PGLD,P2 Gold Inc.,TSXV
PBM,Pacific Bay Minerals Ltd.,TSXV
BKM,Pacific Booker Minerals Inc.,TSXV
PEMC,Pacific Empire Minerals Corp.,TSXV
PPM,Pacific Imperial Mines Inc.,TSXV
PEX,Pacific Ridge Exploration Ltd.,TSXV
PA,Palamina Corp.,TSXV
PALI,Palisades Goldcorp Ltd.,TSXV
PNN,Pambili Natural Resources Corporation,TSXV
PGZ,Pan Global Resources Inc.,TSXV
PAN,Pangolin Diamonds Corp.,TSXV
PANO.P,Panorama Capital Corp.,TSXV
PML,Panoro Minerals Ltd.,TSXV
PNTR,Pantera Silver Corp.,TSXV
PDVN.P,Pardus Ventures Inc.,TSXV
PKT,Parkit Enterprise Inc.,TSXV
PVF.PR.U,Partners Value Investments L.P.,TSXV
PVIS,Parvis Invest Inc.,TSXV
VEIN,Pasofino Gold Limited,TSXV
PGDC,Patagonia Gold Corp.,TSXV
RV,Pathfinder Ventures Inc.,TSXV
PCAA.P,PC 1 Corp.,TSXV
PRH,Pearl River Holdings Limited,TSXV
PEGA,Pegasus Resources Inc.,TSXV
PX,Pelangio Exploration Inc.,TSXV
PEM.P,Penbar Capital Ltd.,TSXV
PTF,Pender Growth Fund Inc.,TSXV
PCP.P,Pender Street Capital Corp.,TSXV
PNTI.P,Pentagon I Capital Corp.,TSXV
PINK,\"Perimeter Medical Imaging AI, Inc.\",TSXV
PRSN,Personas Social Incorporated,TSXV
PER,Peruvian Metals Corp.,TSXV
PESO,Pesorama Inc.,TSXV
VRY,Petro-Victory Energy Corp.,TSXV
PFC,PetroFrontier Corp.,TSXV
PCQ,Petrolympic Ltd.,TSXV
PTC,Petrox Resources Corp.,TSXV
PHNM,Phenom Resources Corp.,TSXV
PINE.U,Pine Trail Real Estate Investment Trust,TSXV
PINN,Pinnacle Silver and Gold Corp.,TSXV
PTE,Pioneering Technology Corp.,TSXV
PVT,Pivotree Inc.,TSXV
PJX,PJX Resources Inc.,TSXV
XOX.P,Planet X Capital Corp.,TSXV
PLXX.P,Planet X II Capital Corp.,TSXV
PTFY,\"Plantify Foods, Inc.\",TSXV
PLA,Plata Latina Minerals Corporation,TSXV
PGC,Plato Gold Corp.,TSXV
PLY,Playfair Mining Ltd.,TSXV
DEAL,Playgon Games Inc.,TSXV
PLRB,Pluribus Technologies Corp.,TSXV
PLUR,Plurilock Security Inc.,TSXV
POC.P,POCML 7 Inc.,TSXV
PTK,POET Technologies Inc.,TSXV
POND,Pond Technologies Holdings Inc.,TSXV
POOL,Pool Safe Inc.,TSXV
INIK,PopReach Corporation,TSXV
POR,Portofino Resources Inc.,TSXV
PGP,Power Group Projects Corp.,TSXV
PWM,Power Metals Corp.,TSXV
PNPN,Power Nickel Inc.,TSXV
PWRO,Power One Resources Corp.,TSXV
PBX,Powerband Solutions Inc.,TSXV
PPX,PPX Mining Corp.,TSXV
PRG,Precipitate Gold Corp.,TSXV
PUR,Premier American Uranium Inc.,TSXV
PHA,Premier Health of America Inc.,TSXV
PNRL,Premium Nickel Resources Ltd.,TSXV
PWIK.P,Prestwick Capital Corporation Limited,TSXV
PTEC,Principal Technologies Inc.,TSXV
PMX,ProAm Explorations Corporation,TSXV
PLAN,Progressive Planet Solutions Inc.,TSXV
PPP,Prospector Metals Corp.,TSXV
PGX,Prosper Gold Corp.,TSXV
PEI,Prospera Energy Inc.,TSXV
MAPS,ProStar Holdings Inc.,TSXV
PTN.P,Proton Capital Corp.,TSXV
PHD,Providence Gold Mines Inc.,TSXV
TORO,Pucara Gold Ltd.,TSXV
PLSR,Pulsar Helium Inc.,TSXV
PUL,Pulse Oil Corp.,TSXV
PUMA,Puma Exploration Inc.,TSXV
PE,Pure Energy Minerals Limited,TSXV
PTU,Purepoint Uranium Group Inc.,TSXV
QGR,Q-Gold Resources Ltd.,TSXV
QTWO,Q2 Metals Corp.,TSXV
QCCU,QC Copper and Gold Inc.,TSXV
QCX,QCX Gold Corp.,TSXV
QMC,QMC Quantum Minerals Corp.,TSXV
QRO,Quadro Resources Ltd.,TSXV
QNC,Quantum Emotion Corp.,TSXV
QZM,Quartz Mountain Resources Ltd.,TSXV
QPM,Quebec Precious Metals Corporation,TSXV
QPT,Quest PharmaTech Inc.,TSXV
QST,Questor Technology Inc.,TSXV
Q,Quetzal Copper Corp.,TSXV
QUIS,Quisitive Technology Solutions Inc.,TSXV
QIS,Quorum Information Technologies Inc.,TSXV
QURI,Quri-Mayu Developments Ltd.,TSXV
QYOU,Qyou Media Inc.,TSXV
RRR.UN,R&R Real Estate Investment Trust,TSXV
RAK,Rackla Metals Inc.,TSXV
RDS,Radisson Mining Resources Inc.,TSXV
RDU,Radius Gold Inc.,TSXV
RRCC.P,Raging Rhino Capital Corp.,TSXV
RLT.P,Railtown Capital Corp,TSXV
RMO,Rainy Mountain Royalty Corp.,TSXV
RKV,Rakovina Therapeutics Inc.,TSXV
RAMP,Ramp Metals Inc.,TSXV
RNCH,Ranchero Gold Corp.,TSXV
RTH,Rathdowney Resources Ltd.,TSXV
RE,RE Royalties Ltd.,TSXV
RGI,Reco International Group Inc.,TSXV
RECO,Reconnaissance Energy Africa Ltd.,TSXV
REC,Record Resources Inc.,TSXV
AMY,RecycLiCo Battery Materials Inc.,TSXV
RPX,Red Pine Exploration Inc.,TSXV
KUT,Redishred Capital Corp.,TSXV
REEM.P,Reem Capital Corp.,TSXV
GIII,ReGen III Corp.,TSXV
RSMX,Regency Silver Corp.,TSXV
RPP,Regent Pacific Properties Inc.,TSXV
REG,Regulus Resources Inc.,TSXV
RET,Reitmans (Canada) Limited,TSXV
MYID,Reklaim Ltd.,TSXV
REKO,Reko International Group Inc.,TSXV
RGC,Relevant Gold Corp.,TSXV
RAGE,Renegade Gold Inc.,TSXV
RW,RenoWorks Software Inc.,TSXV
RSS,Resaas Services Inc.,TSXV
RRL,Resolute Resources Ltd.,TSXV
RSM,Resouro Strategic Metals Inc.,TSXV
RGD,Reunion Gold Corporation,TSXV
RVG,Revival Gold Inc.,TSXV
REVO,RevoluGROUP Canada Inc.,TSXV
REVV,Revolve Renewable Power Corp.,TSXV
OWN,Rex Resources Corp.,TSXV
REYG,Reyna Gold Corp.,TSXV
RSLV,Reyna Silver Corp.,TSXV
RYE,Rhyolite Resources Ltd.,TSXV
RMD,Richmond Minerals Inc.,TSXV
RDG,Ridgeline Minerals Corp.,TSXV
RMI,Ridgestone Mining Inc.,TSXV
LITT,Right Season Investments Corp.,TSXV
RLYG,Riley Gold Corp.,TSXV
RYO,Rio Silver Inc.,TSXV
RIO,Rio2 Limited,TSXV
RVLY,Rivalry Corp.,TSXV
RRI,Riverside Resources Inc.,TSXV
RAC.P,Riverwalk Acquisition Corp.,TSXV
RIWI,RIWI Corp.,TSXV
RJX.A,RJK Explorations Ltd.,TSXV
RBX,Robex Resources Inc.,TSXV
RCT,Rochester Resources Ltd.,TSXV
RCK,Rock Tech Lithium Inc.,TSXV
RK,Rockhaven Resources Ltd.,TSXV
RSC.P,Rockmount Capital Corporation,TSXV
R.P,Rockport Capital Corp.,TSXV
ROCK,Rockridge Resources Ltd.,TSXV
RUM,Rocky Mountain Liquor Inc.,TSXV
RRS,Rogue Resources Inc.,TSXV
ROK,ROK Resources Inc.,TSXV
RKR,Rokmaster Resources Corp.,TSXV
RMR,Rome Resources Ltd.,TSXV
RG,Romios Gold Resources Inc.,TSXV
RVC.P,Ronin Ventures Corp.,TSXV
ROS,Roscan Gold Corporation,TSXV
ROSH.P,Roshni Capital Inc.,TSXV
ROI,Route1 Inc.,TSXV
ROVR,Rover Critical Minerals Corp.,TSXV
RHC,Royal Helium Ltd.,TSXV
RYR,Royal Road Minerals Limited,TSXV
RTM,RT Minerals Corp.,TSXV
ROMJ,Rubicon Organics Inc.,TSXV
RUG,Rugby Resources Ltd.,TSXV
RMB,Rumbu Holdings Ltd.,TSXV
RUCC.P,Rupert's Crossing Capital Inc.,TSXV
RML,Rusoro Mining Ltd.,TSXV
SBIO,Sabio Holdings Inc.,TSXV
SAE,Sable Resources Ltd.,TSXV
SAGE,Sage Potash Corp.,TSXV
FISH,Sailfish Royalty Corp.,TSXV
SRL,Salazar Resources Limited,TSXV
SME,Sama Resources Inc.,TSXV
SSS.P,Samurai Capital Corp.,TSXV
SLG,San Lorenzo Gold Corp.,TSXV
STA,Sanatana Resources Inc.,TSXV
SFR,Sandfire Resources America Inc.,TSXV
SCZ,Santacruz Silver Mining Ltd.,TSXV
SWA,Sarama Resources Ltd.,TSXV
SATO,SATO Technologies Corp.,TSXV
SAC.P,Savanna Capital Corp.,TSXV
SRE,Saville Resources Inc.,TSXV
SAWC.P,Sayward Capital Corp.,TSXV
SKAL.P,Scaling Capital 1 Corp.,TSXV
SCD,Scandium Canada Ltd.,TSXV
SGN,Scorpio Gold Corporation,TSXV
SCOT,Scottie Resources Corp.,TSXV
SLX.P,Searchlight Innovations Inc.,TSXV
SCLT,Searchlight Resources Inc.,TSXV
SGZ,Sego Resources Inc.,TSXV
SEND,Sendero Resources Corp.,TSXV
SEVN.P,Seven Oaks Capital Corp.,TSXV
SNM,ShaMaran Petroleum Corp.,TSXV
SHLL.P,Shellron Capital Ltd.,TSXV
SHRP,Sherpa II Holdings Corp.,TSXV
RENT.P,Shine Box Capital Corp.,TSXV
SNYB,Shiny Health & Wellness Corp.,TSXV
SSSS.P,Shooting Star Acquisition Corp.,TSXV
SIE,Sienna Resources Inc.,TSXV
SM,Sierra Madre Gold and Silver Ltd.,TSXV
SGML,Sigma Lithium Corporation,TSXV
SGU,Signature Resources Ltd.,TSXV
SBMI,Silver Bullet Mines Corp.,TSXV
SVG,Silver Grail Resources Ltd.,TSXV
AGMR,Silver Mountain Resources Inc.,TSXV
SNAG,Silver North Resources Ltd.,TSXV
SVE,Silver One Resources Inc.,TSXV
SPD,Silver Predator Corp.,TSXV
SNG,Silver Range Resources Ltd.,TSXV
SSE,Silver Spruce Resources Inc.,TSXV
SVRS,Silver Storm Mining Ltd.,TSXV
SLVR,Silver Tiger Metals Inc.,TSXV
SILV,Silver Valley Metals Corp.,TSXV
VIPR,Silver Viper Minerals Corp.,TSXV
SWLF,Silver Wolf Exploration Ltd.,TSXV
AGX,Silver X Mining Corp.,TSXV
SBBC,Simply Better Brands Corp.,TSXV
HASH,Simply Solventless Concentrates Ltd.,TSXV
SEI,Sintana Energy Inc.,TSXV
SOI,Sirios Resources Inc.,TSXV
SBM,Sirona Biochem Corp.,TSXV
SIG,Sitka Gold Corp.,TSXV
SKRR,SKRR Exploration Inc.,TSXV
SKYG,Sky Gold Corp.,TSXV
SYH,Skyharbour Resources Ltd.,TSXV
SXL,SLAM Exploration Ltd.,TSXV
SSX.P,Sleeping Giant Capital Corp.,TSXV
SMAR.P,Smartset Services Inc.,TSXV
SMTH.P,Smithe Resources Corp.,TSXV
SMRV,Smooth Rock Ventures Corp.,TSXV
SPN,Snipp Interactive Inc.,TSXV
SGD,Snowline Gold Corp.,TSXV
SIC,Sokoman Minerals Corp.,TSXV
SOLR,Solar Alliance Energy Inc.,TSXV
SOLI.P,Solid Impact Investments Corp.,TSXV
SLMN,Solis Minerals Ltd.,TSXV
SGC,Solstice Gold Corp.,TSXV
SOMA,Soma Gold Corp.,TSXV
SNI.PR.A,Sonor Investments Limited,TSXV
SDCU,Sonoran Desert Copper Corporation,TSXV
SNV,Sonoro Energy Ltd.,TSXV
SGO,Sonoro Gold Corp.,TSXV
SRR,Source Rock Royalties Ltd.,TSXV
SAO,South Atlantic Gold Inc.,TSXV
SPMC,South Pacific Metals Corp.,TSXV
STS,South Star Battery Metals Corp.,TSXV
SMP,Southern Empire Resources Corp.,TSXV
SOU,Southern Energy Corp.,TSXV
SSV,Southern Silver Exploration Corp.,TSXV
SGQ,SouthGobi Resources Ltd,TSXV
SML,Southstone Minerals Limited,TSXV
SPSA.P,SP Strategic Acquisition Corp.,TSXV
YSK.P,Space Kingdom Digital Capital Corp.,TSXV
SQG,Spackman Equities Group Inc.,TSXV
SPA,Spanish Mountain Gold Ltd.,TSXV
SPRQ,Sparq Systems Inc.,TSXV
SAY,Sparta Capital Ltd.,TSXV
SRI,Sparton Resources Inc.,TSXV
SPC,SPC Nickel Corp.,TSXV
SSA,Spectra Products Inc.,TSXV
SEV,Spectra7 Microsystems Inc.,TSXV
SOO.P,Spectre Capital Corp.,TSXV
SRG,SRG Mining Inc.,TSXV
SRQ,SRQ Resources Inc.,TSXV
SECU,SSC Security Services Corp.,TSXV
SDCI.P,St. Davids Capital Inc.,TSXV
LORD,St. James Gold Corp.,TSXV
SRC,Stakeholder Gold Corp.,TSXV
STUD,Stallion Uranium Corp.,TSXV
SDI,Stampede Drilling Inc.,TSXV
STMP,Stamper Oil & Gas Corp.,TSXV
SLI,Standard Lithium Ltd.,TSXV
STND,Standard Uranium Ltd.,TSXV
STRR,Star Royalties Ltd.,TSXV
SCPT.A,Starlight U.S. Multi-Family (No.2) Core Plus Fund,TSXV
SURF.A,Starlight U.S. Residential Fund,TSXV
STE,Starr Peak Mining Ltd.,TSXV
SPX,Stellar AfricaGold Inc.,TSXV
STH,Stelmine Canada Ltd.,TSXV
SAG,Sterling Metals Corp.,TSXV
PGE,Stillwater Critical Minerals Corp.,TSXV
STNG,Stinger Resources Inc.,TSXV
STRM,Storm Exploration Inc.,TSXV
SGE,Strategem Capital Corporation,TSXV
SMD,Strategic Metals Ltd.,TSXV
SR,Strategic Resources Inc.,TSXV
SUU,Strathmore Plus Uranium Corp.,TSXV
SRA,Stria Lithium Inc.,TSXV
SKP,Strikepoint Gold Inc.,TSXV
SKK,Strikewell Energy Corp.,TSXV
SDR,Stroud Resources Ltd.,TSXV
STU,Stuhini Exploration Ltd,TSXV
STUV,Stuve Gold Corp.,TSXV
SUG,Sucro Limited,TSXV
SSVR,Summa Silver Corp.,TSXV
PEAK,Sun Peak Metals Corp.,TSXV
SRES,Sun Residential Real Estate Investment Trust,TSXV
SMN,Sun Summit Minerals Corp.,TSXV
SPZ,SuperBuzz Inc.,TSXV
SUI,Superior Mining International Corporation,TSXV
NILI,Surge Battery Metals Inc.,TSXV
SURG,Surge Copper Corp.,TSXV
SYG,Sylla Gold Corp.,TSXV
TWO,T2 Metals Corp.,TSXV
RARE,Tactical Resources Corp.,TSXV
TAO,TAG Oil Ltd.,TSXV
TAJ,Tajiri Resources Corp.,TSXV
TRO,Taranis Resources Inc.,TSXV
TKU,Tarku Resources Ltd.,TSXV
TORA,Taura Gold Inc.,TSXV
TDG,TDG Gold Corp.,TSXV
TEA,Tearlach Resources Limited,TSXV
TECT,Tectonic Metals Inc.,TSXV
TELO,Telo Genomics Corp.,TSXV
TPC,Tenth Avenue Petroleum Corp.,TSXV
TES,Tesoro Minerals Corp.,TSXV
TPL,Tethys Petroleum Limited,TSXV
TUO,Teuton Resources Corp.,TSXV
FRSH,The Fresh Factory B.C. Ltd.,TSXV
MUSH,The Good Shroom Co Inc.,TSXV
HMPG,\"The Hempshire Group, Inc.\",TSXV
BOAT,The Limestone Boat Company Limited,TSXV
MYLK,The Planting Hope Company Inc.,TSXV
WED,The Westaim Corporation,TSXV
WI,The Western Investment Company of Canada Limited,TSXV
MAC,Themac Resources Group Limited,TSXV
TLT,Theralase Technologies Inc.,TSXV
THRM,Therma Bright Inc.,TSXV
TMG,Thermal Energy International Inc.,TSXV
TAU,Thesis Gold Inc,TSXV
TTI,\"Thiogenesis Therapeutics, Corp.\",TSXV
THX,Thor Explorations Ltd.,TSXV
YAY,THS Maple Holdings Ltd.,TSXV
TGOL,Thunder Gold Corp.,TSXV
THM,\"Thunder Mountain Gold, Inc.\",TSXV
TBRD,Thunderbird Entertainment Group Inc.,TSXV
BIRD,Thunderbird Minerals Corp.,TSXV
AWE,Thunderstruck Resources Ltd.,TSXV
TSLV,Tier One Silver Inc.,TSXV
TIL,Till Capital Corporation,TSXV
TBR,Timberline Resources Corporation,TSXV
TLC.P,Timeless Capital Corp.,TSXV
TIN,Tincorp Metals Inc.,TSXV
TK,Tinka Resources Limited,TSXV
TORC,TinOne Resources Inc.,TSXV
TTS,Tintina Mines Ltd.,TSXV
TINY,Tiny Ltd.,TSXV
TLA,Titan Logix Corp.,TSXV
TNR,TNR Gold Corp.,TSXV
COIN,Tokens.com Corp.,TSXV
LOT,TomaGold Corporation,TSXV
TBLL,Tombill Mines Limited,TSXV
TOI,Topicus.com Inc.,TSXV
TLX.P,Torchlight Innovations Inc.,TSXV
TGH,Tornado Global Hydrovacs Ltd.,TSXV
TORQ,Torq Resources Inc.,TSXV
TMET,Torr Metals Inc.,TSXV
TORR,Torrent Capital Ltd.,TSXV
TOH,Total Helium Ltd.,TSXV
TTZ,Total Telcom Inc.,TSXV
THP,Totally Hip Technologies Inc.,TSXV
TOTC.P,Totec Resources Ltd.,TSXV
TWR,Tower Resources Ltd.,TSXV
TBLZ.P,Trail Blazer Capital Corp.,TSXV
TBK,Trailbreaker Resources Ltd.,TSXV
TTG,Trans Canada Gold Corp.,TSXV
TCO,Transatlantic Mining Corp.,TSXV
TCG.P,Transcontinental Gold Corp.,TSXV
XTM,Transition Metals Corp.,TSXV
TOP.P,Transition Opportunities Corp.,TSXV
TMC,Trench Metals Corp.,TSXV
TRS,Tres-Or Resources Ltd.,TSXV
TRV.P,Treviso Capital Corp.,TSXV
TRBE,Tribe Property Technologies Inc.,TSXV
TRBC,Tribeca Resources Corporation,TSXV
TG,Trifecta Gold Ltd.,TSXV
TM,Trigon Metals Inc.,TSXV
TCK.P,Trillium Acquisition Corp.,TSXV
TSG,TriStar Gold Inc.,TSXV
TIG,Triumph Gold Corp.,TSXV
TR,Troubadour Resources Inc.,TSXV
TRU,TRU Precious Metals Corp.,TSXV
TGX,True North Gems Inc.,TSXV
TBIX,TrustBIX Inc.,TSXV
TSD,Tsodilo Resources Limited,TSXV
TUD,Tudor Gold Corp.,TSXV
TUK,Tuktu Resources Ltd.,TSXV
TUP.P,TUP Capital Inc.,TSXV
TBX,Turmalina Metals Corp.,TSXV
TTGI,Turnium Technology Group Inc.,TSXV
GYM,TUT Fitness Group Inc.,TSXV
TVI,Tvi Pacific Inc.,TSXV
UCU,Ucore Rare Metals Inc.,TSXV
UGE,UGE International Ltd.,TSXV
ULT,Ultra Lithium Inc.,TSXV
UGD,Unigold Inc.,TSXV
USS,Uniserve Communications Corporation,TSXV
IBO,Universal Ibogaine Inc.,TSXV
UPT.P,Upstart Investments Inc.,TSXV
UIG,Urban Infrastructure Group Inc.,TSXV
UFC,Urbanfund Corp.,TSXV
USCU,US Copper Corp.,TSXV
USHA,Usha Resources Ltd.,TSXV
VTEN.P,V Ten Capital Corp.,TSXV
VZZ,Val-d'Or Mining Corporation,TSXV
VAL.P,Valencia Capital Inc.,TSXV
VMXX,Valhalla Metals Inc.,TSXV
VVR,Valleyview Resources Ltd.,TSXV
VO,Valore Metals Corp.,TSXV
VRB,Vanadiumcorp Resource Inc.,TSXV
VAX,Vantex Resources Ltd.,TSXV
VCV,Vatic Ventures Corp.,TSXV
VXL,Vaxil Bio Ltd.,TSXV
VLC,Velocity Minerals Ltd.,TSXV
VLX,Velox Energy Materials Inc.,TSXV
VTT,Vendetta Mining Corp.,TSXV
VLV,Venerable Ventures Ltd.,TSXV
VPT,VentriPoint Diagnostics Ltd.,TSXV
VENZ,Venzee Technologies Inc.,TSXV
VTX,Vertex Resource Group Ltd.,TSXV
VERT,Vertical Exploration Inc.,TSXV
VCC.P,Veteran Capital Corp.,TSXV
VMC,Vicinity Motor Corp.,TSXV
VOC.P,Victory Opportunities 1 Corp.,TSXV
VIO,Vior Inc.,TSXV
VIP,VIP Entertainment Technologies Inc.,TSXV
VML,Viscount Mining Corp.,TSXV
VGD,Visible Gold Mines Inc.,TSXV
VLI,Vision Lithium Inc.,TSXV
VIZ,Visionary Metals Corp.,TSXV
VIS,Visionstate Corp.,TSXV
VUX,Vital Energy Inc.,TSXV
VPI,Vitality Products Inc.,TSXV
VCI,Vitreous Glass Inc.,TSXV
VAU,Viva Gold Corp.,TSXV
VCU,Vizsla Copper Corp.,TSXV
VZLA,Vizsla Silver Corp.,TSXV
VOL,Volatus Aerospace Corp.,TSXV
VG,Volcanic Gold Mines Inc,TSXV
VCT,Volt Carbon Technologies Inc.,TSXV
VLT,Volt Lithium Corp.,TSXV
VMS,Vortex Metals Inc.,TSXV
VXTR,Voxtur Analytics Corp.,TSXV
VM,Voyageur Pharmaceuticals Ltd.,TSXV
VRR,VR Resources Ltd.,TSXV
VUL,Vulcan Minerals Inc.,TSXV
VVC,VVC Exploration Corporation,TSXV
WRR,Walker River Resources Corp.,TSXV
WRI,Waseco Resources Inc.,TSXV
WWT,Water Ways Technologies Inc.,TSXV
WAVE,Waverley Pharma Inc.,TSXV
WML,Wealth Minerals Ltd.,TSXV
WFLD,Wellfield Technologies Inc.,TSXV
WCE,Wescan Energy Corp.,TSXV
WGF,Wescan Goldfields Inc.,TSXV
WHY,West High Yield (W.H.Y.) Resources Ltd.,TSXV
WRLG,West Red Lake Gold Mines Ltd.,TSXV
WVM,West Vault Mining Inc.,TSXV
WBE,WestBond Enterprises Corporation,TSXV
WEB,Westbridge Renewable Energy Corp.,TSXV
WAM,Western Alaska Minerals Corp.,TSXV
WA,Western Atlas Resources Inc,TSXV
WEX,Western Exploration Inc.,TSXV
WGLD,Western Gold Exploration Ltd.,TSXV
WMS,Western Metallica Resources Corp.,TSXV
WP,Western Pacific Trust Company,TSXV
WHN,Westhaven Gold Corp.,TSXV
WKG,WestKam Gold Corp.,TSXV
WAT.P,Whatcom Capital II Corp,TSXV
WGO,White Gold Corp.,TSXV
WMK,Whitemud Resources Inc.,TSXV
WWA.P,Whitewater Acquisition Corp.,TSXV
YTY,Wi2Wi Corporation,TSXV
CANS,Wildpack Beverage Inc.,TSXV
WSK,Wildsky Resources Inc.,TSXV
WIL,Wilton Resources Inc.,TSXV
WIN,Windfall Geotek Inc.,TSXV
WINS,Winshear Gold Corp.,TSXV
WISH,Wishpond Technologies Ltd.,TSXV
WITT.P,Wittering Capital Corp.,TSXV
WLF,Wolfden Resources Corporation,TSXV
WOOD.P,Woodbridge Ventures II Inc.,TSXV
WCU,World Copper Ltd.,TSXV
XGC,Xali Gold Corp.,TSXV
XND,Xander Resources Inc.,TSXV
GIG,XAU Resources Inc.,TSXV
XIM,Ximen Mining Corp.,TSXV
XRTX,XORTX Therapeutics Inc.,TSXV
XPLR,Xplore Resources Corp.,TSXV
XL,XXL Energy Corp.,TSXV
XYBN,Xybion Digital Inc.,TSXV
YERB.U,Yerbae Brands Corp.,TSXV
YNV,Ynvisible Interactive Inc.,TSXV
YORK,York Harbour Metals Inc.,TSXV
YEG,Yorkton Equity Group Inc.,TSXV
ZAC,Zacatecas Silver Corp.,TSXV
ZBNI,Zeb Nickel Corp.,TSXV
ZDC,Zedcor Inc.,TSXV
ZENI.P,Zenith Capital Corporation,TSXV
ZEN,Zentek Ltd.,TSXV
ZFR,Zephyr Minerals Ltd.,TSXV
ZC,Zimtu Capital Corp.,TSXV
ZNX,Zincx Resources Corp.,TSXV
ZAU,Zodiac Gold Inc.,TSXV
ZON,Zonte Metals Inc.,TSXV
ZMA,ZoomAway Technologies Inc.,TSXV
ZOMD,Zoomd Technologies Ltd.,TSXV
ZUM,ZoomerMedia Limited,TSXV
ZYUS,ZYUS Life Sciences Corporation,TSXV
IGBT.UN,2028 Investment Grade Bond Trust,TSX
BTCQ,3iQ Bitcoin ETF,TSX
ETHQ,3iQ Ether Staking ETF,TSX
VNP,5N Plus Inc.,TSX
AW.UN,A&W Revenue Royalties Income Fund,TSX
AAB,Aberdeen International Inc.,TSX
FAP,abrdn Asia-Pacific Income Fund VCC,TSX
ADN,Acadian Timber Corp.,TSX
HDGE,Accelerate Absolute Return Fund,TSX
ARB,Accelerate Arbitrage Fund,TSX
ATSX,Accelerate Canadian Long Short Equity Fund,TSX
INCM,Accelerate Diversified Credit Income Fund,TSX
ONEC,Accelerate OneChoice Alternative Portfolio ETF,TSX
ACD,Accord Financial Corp.,TSX
ARA,Aclara Resources Inc.,TSX
ADCO,Adcore Inc.,TSX
ADEN,ADENTRA Inc.,TSX
DRX,ADF Group Inc.,TSX
AAV,Advantage Energy Ltd.,TSX
ARE,Aecon Group Inc.,TSX
AEG,Aegis Brands Inc.,TSX
AEZS,AEterna Zentaris Inc.,TSX
AOI,Africa Oil Corp.,TSX
AFN,Ag Growth International Inc.,TSX
AGF.B,AGF Management Limited,TSX
QCD,AGF Systematic Canadian Equity ETF,TSX
QEM,AGF Systematic Emerging Markets Equity ETF,TSX
QIE,AGF Systematic International Equity ETF,TSX
QUS,AGF Systematic US Equity ETF,TSX
QBTL,AGF US Market Neutral Anti-Beta CAD-Hedged ETF,TSX
AEM,Agnico Eagle Mines Limited,TSX
AGRI.RT.U,Agrinam Acquisition Corporation,TSX
AIM,Aimia Inc.,TSX
AC,Air Canada,TSX
BOS,AirBoss of America Corp.,TSX
AKT.A,Akita Drilling Ltd.,TSX
AGI,Alamos Gold Inc.,TSX
AD.DB,Alaris Equity Partners Income Trust,TSX
ALC,Algoma Central Corporation,TSX
ASTL,Algoma Steel Group Inc.,TSX
AQN,Algonquin Power & Utilities Corp.,TSX
ATD,Alimentation Couche-Tard Inc.,TSX
ALYA,Alithya Group inc.,TSX
AAUC,Allied Gold Corporation,TSX
AP.UN,Allied Properties Real Estate Investment Trust,TSX
AMM,Almaden Minerals Ltd.,TSX
AII,Almonty Industries Inc.,TSX
ATCU,Alta Copper Corp.,TSX
ALA,AltaGas Ltd.,TSX
ALS,Altius Minerals Corporation,TSX
ARR,Altius Renewable Royalties Corp.,TSX
AIF,Altus Group Limited,TSX
HOT.DB.V,American Hotel Income Properties REIT LP,TSX
USA,Americas Gold and Silver Corporation,TSX
ARG,Amerigo Resources Ltd.,TSX
ANRG,Anaergia Inc.,TSX
AND,Andlauer Healthcare Group Inc.,TSX
ADW.A,Andrew Peller Limited/Andrew Peller Limitee,TSX
APLI,Appili Therapeutics Inc.,TSX
APS,Aptose Biosciences Inc.,TSX
ARX,ARC Resources Ltd.,TSX
AR,Argonaut Gold Inc.,TSX
ARIS,Aris Mining Corporation,TSX
ATZ,Aritzia Inc.,TSX
AMC,Arizona Metals Corp.,TSX
ASCU,Arizona Sonoran Copper Company Inc.,TSX
ADIV,Arrow EC Equity Advantage Alternative Fund,TSX
RATE,Arrow EC Income Advantage Alternative Fund,TSX
ACAA,Arrow Long/Short Alternative Class,TSX
AX.PR.E,Artis Real Estate Investment Trust,TSX
ASND,Ascendant Resources Inc.,TSX
AOT,Ascot Resources Ltd.,TSX
ACO.X,ATCO Ltd.,TSX
ATH,Athabasca Oil Corporation,TSX
AI,Atrium Mortgage Investment Corporation,TSX
ATS,ATS Corporation,TSX
G,Augusta Gold Corp.,TSX
ORA,Aura Minerals Inc.,TSX
ACB,Aurora Cannabis Inc.,TSX
ACQ,AutoCanada Inc.,TSX
APR.UN,Automotive Properties Real Estate Investment Trust,TSX
XLY,Auxly Cannabis Group Inc.,TSX
AVL,Avalon Advanced Materials Inc.,TSX
AVNT,Avant Brands Inc.,TSX
AVCN,Avicanna Inc.,TSX
ASM,Avino Silver & Gold Mines Ltd.,TSX
AXIS,Axis Auto Finance Inc.,TSX
AYA,Aya Gold & Silver Inc.,TSX
BTO,B2Gold Corp.,TSX
BDGI,Badger Infrastructure Solutions Ltd.,TSX
BLDP,Ballard Power Systems Inc.,TSX
BMO,Bank of Montreal,TSX
BNS,Bank of Nova Scotia (The),TSX
ABX,Barrick Gold Corporation,TSX
BLCO,Bausch + Lomb Corporation,TSX
BHC,Bausch Health Companies Inc.,TSX
BYL,Baylin Technologies Inc.,TSX
BTE,Baytex Energy Corp.,TSX
BCE,BCE Inc.,TSX
BEK.B,Becker Milk Company Ltd. (The),TSX
BSX,Belo Sun Mining Corp.,TSX
BNG,Bengal Energy Ltd.,TSX
HGD,BetaPro Canadian Gold Miners -2x Daily Bear ETF,TSX
HGU,BetaPro Canadian Gold Miners 2x Daily Bull ETF,TSX
HOD,BetaPro Crude Oil Inverse Leveraged Daily Bear ETF,TSX
HOU,BetaPro Crude Oil Leveraged Daily Bull ETF,TSX
HBKD,BetaPro Equal Weight Canadian Bank -2x Daily Bear ETF,TSX
HBKU,BetaPro Equal Weight Canadian Bank 2x Daily Bull ETF,TSX
HRED,BetaPro Equal Weight Canadian REIT -2x Daily Bear ETF,TSX
HREU,BetaPro Equal Weight Canadian REIT 2x Daily Bull ETF,TSX
HBD,BetaPro Gold Bullion -2x Daily Bear ETF,TSX
HBU,BetaPro Gold Bullion 2x Daily Bull ETF,TSX
BITI,BetaPro Inverse Bitcoin ETF,TSX
HQD,BetaPro NASDAQ-100 -2x Daily Bear ETF,TSX
HQU,BetaPro NASDAQ-100 2x Daily Bull ETF,TSX
HND,BetaPro Natural Gas Inverse Leveraged Daily Bear ETF,TSX
HNU,BetaPro Natural Gas Leveraged Daily Bull ETF,TSX
HSD,BetaPro S&P 500 -2x Daily Bear ETF,TSX
HSU,BetaPro S&P 500 2x Daily Bull ETF,TSX
HIU,BetaPro S&P 500 Daily Inverse ETF,TSX
HUV,BetaPro S&P 500 VIX Short-Term Futures ETF,TSX
HXD,BetaPro S&P/TSX 60 -2x Daily Bear ETF,TSX
HXU,BetaPro S&P/TSX 60 2x Daily Bull ETF,TSX
HIX,BetaPro S&P/TSX 60 Daily Inverse ETF,TSX
HED,BetaPro S&P/TSX Capped Energy -2x Daily Bear ETF,TSX
HEU,BetaPro S&P/TSX Capped Energy 2x Daily Bull ETF,TSX
HFD,BetaPro S&P/TSX Capped Financials -2x Daily Bear ETF,TSX
HFU,BetaPro S&P/TSX Capped Financials 2x Daily Bull ETF,TSX
HZD,BetaPro Silver -2x Daily Bear ETF,TSX
HZU,BetaPro Silver 2x Daily Bull ETF,TSX
BNK,Big Banc Split Corp.,TSX
PRM,Big Pharma Split Corp.,TSX
BR,Big Rock Brewery Inc.,TSX
BIK.PR.A,BIP Investment Corporation,TSX
BIR,Birchcliff Energy Ltd.,TSX
BDT,Bird Construction Inc.,TSX
QBTC,\"Bitcoin Fund, The\",TSX
BITF,Bitfarms Ltd.,TSX
BDI,Black Diamond Group Limited,TSX
BKI,Black Iron Inc.,TSX
BB,BlackBerry Limited,TSX
BLN,Blackline Safety Corp.,TSX
HBLK,Blockchain Technologies ETF,TSX
BLB.UN,Bloom Select Income Fund,TSX
RBN.UN,Blue Ribbon Income Fund,TSX
ZAG,BMO Aggregate Bond Index ETF,TSX
ZEQT,BMO All-Equity ETF,TSX
ARKG,BMO ARK Genomic Revolution Fund,TSX
ARKK,BMO ARK Innovation Fund,TSX
ARKW,BMO ARK Next Generation Internet Fund,TSX
ZESG,BMO Balanced ESG ETF,TSX
ZBAL,BMO Balanced ETF,TSX
ZBBB,BMO BBB Corporate Bond Index ETF,TSX
TOWR,BMO Brookfield Global Real Estate Tech Fund,TSX
GRNI,BMO Brookfield Global Renewables Infrastructure Fund,TSX
ZBI,BMO Canadian Bank Income Index ETF,TSX
ZDV,BMO Canadian Dividend ETF,TSX
ZWC,BMO Canadian High Dividend Covered Call ETF,TSX
ZMBS,BMO Canadian MBS Index ETF,TSX
ZCLN,BMO Clean Energy Index ETF,TSX
ZCON,BMO Conservative ETF,TSX
ZCPB,BMO Core Plus Bond Fund,TSX
ZCB,BMO Corporate Bond Index ETF,TSX
ZCDB,BMO Corporate Discount Bond ETF,TSX
ZWB,BMO Covered Call Canadian Banks ETF,TSX
ZWA,BMO Covered Call Dow Jones Industrial Average Hedged to CAD ETF,TSX
ZWEN,BMO Covered Call Energy ETF,TSX
ZWHC,BMO Covered Call Health Care ETF,TSX
ZWT,BMO Covered Call Technology ETF,TSX
ZWK,BMO Covered Call US Banks ETF,TSX
ZWU,BMO Covered Call Utilities ETF,TSX
ZDB,BMO Discount Bond Index ETF,TSX
ZDJ,BMO Dow Jones Industrial Average Hedged to CAD Index ETF,TSX
ZEF,BMO Emerging Markets Bond Hedged to CAD Index ETF,TSX
ZEB,BMO Equal Weight Banks Index ETF,TSX
ZMT,BMO Equal Weight Global Base Metals Hedged to CAD Index ETF,TSX
ZGD,BMO Equal Weight Global Gold Index ETF,TSX
ZIN,BMO Equal Weight Industrials Index ETF,TSX
ZEO,BMO Equal Weight Oil & Gas Index ETF,TSX
ZRE,BMO Equal Weight REITs Index ETF,TSX
ZUB,BMO Equal Weight US Banks Hedged to CAD Index ETF,TSX
ZBK,BMO Equal Weight US Banks Index ETF,TSX
ZUH,BMO Equal Weight US Health Care Hedged to CAD Index ETF,TSX
ZHU,BMO Equal Weight US Health Care Index ETF,TSX
ZUT,BMO Equal Weight Utilities Index ETF,TSX
ESGB,BMO ESG Corporate Bond Index ETF,TSX
ESGH,BMO ESG High Yield US Corporate Bond Index ETF,TSX
ESGF,BMO ESG US Corporate Bond Hedged to CAD Index ETF,TSX
ZWP,BMO Europe High Dividend Covered Call ETF,TSX
ZWE,BMO Europe High Dividend Covered Call Hedged to CAD ETF,TSX
ZFH,BMO Floating Rate High Yield ETF,TSX
ZEAT,BMO Global Agriculture ETF,TSX
COMM,BMO Global Communications Index ETF,TSX
DISC,BMO Global Consumer Discretionary Hedged to CAD Index ETF,TSX
STPL,BMO Global Consumer Staples Hedged to CAD Index ETF,TSX
ZWQT,BMO Global Enhanced Income Fund,TSX
ZWG,BMO Global High Dividend Covered Call ETF,TSX
ZGI,BMO Global Infrastructure Index ETF,TSX
ZGSB,BMO Global Strategic Bond Fund,TSX
ZGLD,BMO Gold Bullion ETF,TSX
ZGLH,BMO Gold Bullion Hedged to CAD ETF,TSX
ZGB,BMO Government Bond Index ETF,TSX
ZGRO,BMO Growth ETF,TSX
ZQB,BMO High Quality Corporate Bond Index ETF,TSX
ZHY,BMO High Yield US Corporate Bond Hedged to CAD Index ETF,TSX
ZJK,BMO High Yield US Corporate Bond Index ETF,TSX
ZDI,BMO International Dividend ETF,TSX
ZDH,BMO International Dividend Hedged to CAD ETF,TSX
ZJPN,BMO Japan Index ETF,TSX
ZJG,BMO Junior Gold Index ETF,TSX
ZPR,BMO Laddered Preferred Share Index ETF,TSX
ZLC,BMO Long Corporate Bond Index ETF,TSX
ZFL,BMO Long Federal Bond Index ETF,TSX
ZPL,BMO Long Provincial Bond Index ETF,TSX
ZLSC,BMO Long Short Canadian Equity ETF,TSX
ZLSU,BMO Long Short US Equity ETF,TSX
ZLB,BMO Low Volatility Canadian Equity ETF,TSX
ZLE,BMO Low Volatility Emerging Markets Equity ETF,TSX
ZLI,BMO Low Volatility International Equity ETF,TSX
ZLD,BMO Low Volatility International Equity Hedged to CAD ETF,TSX
ZLU,BMO Low Volatility US Equity ETF,TSX
ZLH,BMO Low Volatility US Equity Hedged to CAD ETF,TSX
ZCM,BMO Mid Corporate Bond Index ETF,TSX
ZFM,BMO Mid Federal Bond Index ETF,TSX
ZMP,BMO Mid Provincial Bond Index ETF,TSX
ZMU,BMO Mid-Term US IG Corporate Bond Hedged to CAD Index ETF,TSX
ZIC,BMO Mid-Term US IG Corporate Bond Index ETF,TSX
ZMMK,BMO Money Market Fund,TSX
ZMI,BMO Monthly Income ETF,TSX
ZGRN,BMO MSCI ACWI Paris Aligned Climate Equity Index ETF,TSX
ZGQ,BMO MSCI All Country World High Quality Index ETF,TSX
ESGA,BMO MSCI Canada ESG Leaders Index ETF,TSX
ZVC,BMO MSCI Canada Value Index ETF,TSX
ZCH,BMO MSCI China ESG Leaders Index ETF,TSX
ESGE,BMO MSCI EAFE ESG Leaders Index ETF,TSX
ZDM,BMO MSCI EAFE Hedged to CAD Index ETF,TSX
ZEA,BMO MSCI EAFE Index ETF,TSX
ZEM,BMO MSCI Emerging Markets Index ETF,TSX
ZEQ,BMO MSCI Europe High Quality Hedged to CAD Index ETF,TSX
ESGG,BMO MSCI Global ESG Leaders Index ETF,TSX
ZID,BMO MSCI India ESG Leaders Index ETF,TSX
ESGY,BMO MSCI USA ESG Leaders Index ETF,TSX
ZUQ,BMO MSCI USA High Quality Index ETF,TSX
ZVU,BMO MSCI USA Value Index ETF,TSX
ZQQ,BMO Nasdaq 100 Equity Hedged To CAD Index ETF,TSX
ZNQ,BMO Nasdaq 100 Equity Index ETF,TSX
ZPAY,BMO Premium Yield ETF,TSX
ZRR,BMO Real Return Bond Index ETF,TSX
ZUE,BMO S&P 500 Hedged to CAD Index ETF,TSX
ZSP,BMO S&P 500 Index ETF,TSX
ZMID,BMO S&P US Mid Cap Index ETF,TSX
ZSML,BMO S&P US Small Cap Index ETF,TSX
ZIU,BMO S&P/TSX 60 Index ETF,TSX
ZCN,BMO S&P/TSX Capped Composite Index ETF,TSX
ZCS,BMO Short Corporate Bond Index ETF,TSX
ZFS,BMO Short Federal Bond Index ETF,TSX
ZPS,BMO Short Provincial Bond Index ETF,TSX
ZSB,BMO Short-Term Bond Index ETF,TSX
ZSDB,BMO Short-Term Discount Bond ETF,TSX
ZSU,BMO Short-Term US IG Corporate Bond Hedged to CAD Index ETF,TSX
ZTIP,BMO Short-Term US TIPS Index ETF,TSX
ZFC,BMO SIA Focused Canadian Equity Fund,TSX
ZFN,BMO SIA Focused North American Equity Fund,TSX
ZMSB,BMO Sustainable Global Multi-Sector Bond Fund,TSX
ZZZD,BMO Tactical Dividend ETF Fund,TSX
ZACE,BMO U.S. All Cap Equity Fund,TSX
ZUGE,BMO U.S. Equity Growth MFR Fund,TSX
ZUVE,BMO U.S. Equity Value MFR Fund,TSX
ZST,BMO Ultra Short-Term Bond ETF,TSX
ZUS.U,BMO Ultra Short-Term US Bond ETF,TSX
ZUAG,BMO US Aggregate Bond Index ETF,TSX
ZDY,BMO US Dividend ETF,TSX
ZUD,BMO US Dividend Hedged to CAD ETF,TSX
ZWH,BMO US High Dividend Covered Call ETF,TSX
ZWS,BMO US High Dividend Covered Call Hedged to CAD ETF,TSX
ZHP,BMO US Preferred Share Hedged to CAD Index ETF,TSX
ZUP,BMO US Preferred Share Index ETF,TSX
ZPW,BMO US Put Write ETF,TSX
ZPH,BMO US Put Write Hedged to CAD ETF,TSX
TIPS,BMO US TIPS Index ETF,TSX
ZUCM,BMO USD Cash Management ETF,TSX
WOMN,BMO Women in Leadership Fund,TSX
GBT,BMTC Group Inc.,TSX
BEI.UN,Boardwalk Real Estate Investment Trust,TSX
BRMI,Boat Rocker Media Inc.,TSX
BBD.A,Bombardier Inc.,TSX
BNE,Bonterra Energy Corp.,TSX
BLX,Boralex Inc.,TSX
BPF.UN,Boston Pizza Royalties Income Fund,TSX
BYD,Boyd Group Services Inc.,TSX
BRAG,Bragg Gaming Group Inc.,TSX
BRY,Bri-Chem Corp.,TSX
BCT,BriaCell Therapeutics Corp.,TSX
BRE,Bridgemarq Real Estate Services Inc.,TSX
BGC,Bristol Gate Concentrated Canadian Equity ETF,TSX
BGU,Bristol Gate Concentrated US Equity ETF,TSX
ESP,Brompton Energy Split Corp.,TSX
BMAX,Brompton Enhanced Multi-Asset Income ETF,TSX
EDGF,Brompton European Dividend Growth ETF,TSX
BEPR,Brompton Flaherty & Crumrine Enhanced Investment Grade Preferred ETF,TSX
BPRF,Brompton Flaherty & Crumrine Investment Grade Preferred ETF,TSX
BDIV,Brompton Global Dividend Growth ETF,TSX
HIG,Brompton Global Healthcare Income & Growth ETF,TSX
LCS,Brompton Lifeco Split Corp.,TSX
BFIN,Brompton North American Financials Dividend ETF,TSX
BLOV,Brompton North American Low Volatility Dividend ETF,TSX
SBC,Brompton Split Banc Corp.,TSX
SPLT,Brompton Split Corp. Preferred Share ETF,TSX
BREA,Brompton Sustainable Real Assets Dividend ETF,TSX
TLF,Brompton Tech Leaders Income ETF,TSX
BAM,Brookfield Asset Management Ltd.,TSX
BBUC,Brookfield Business Corporation,TSX
BBU.UN,Brookfield Business Partners L.P.,TSX
BN,Brookfield Corporation,TSX
BGI.UN,Brookfield Global Infrastructure Securities Income Fund,TSX
BIPC,Brookfield Infrastructure Corporation,TSX
BIP.PR.A,Brookfield Infrastructure Partners L.P.,TSX
BPO.PR.A,Brookfield Office Properties Inc.,TSX
BPYP.PR.A,Brookfield Property Preferred L.P.,TSX
BPS.PR.A,Brookfield Property Split Corp.,TSX
BNRE,Brookfield Reinsurance Ltd.,TSX
BEPC,Brookfield Renewable Corporation,TSX
BEP.PR.G,Brookfield Renewable Partners L.P.,TSX
BRF.PR.A,Brookfield Renewable Power Preferred Equity Inc.,TSX
DOO,BRP Inc.,TSX
HOM.DB.U,BSR Real Estate Investment Trust,TSX
BTB.DB.G,BTB Real Estate Investment Trust,TSX
BUI,Buhler Industries Inc.,TSX
BU,Burcon Nutrascience Corporation,TSX
CAE,CAE Inc.,TSX
CWL,Caldwell Partners International Inc. (The),TSX
UDA,Caldwell U.S. Dividend Advantage Fund,TSX
CFW,Calfrac Well Services Ltd.,TSX
CGY,Calian Group Ltd.,TSX
CXB,Calibre Mining Corp.,TSX
CCO,Cameco Corporation,TSX
CF,Canaccord Genuity Group Inc.,TSX
CNE,Canacol Energy Ltd.,TSX
GOOS,Canada Goose Holdings Inc.,TSX
CAR.UN,Canadian Apartment Properties Real Estate Investment Trust,TSX
BK,Canadian Banc Corp.,TSX
CGI,\"Canadian General Investments, Limited\",TSX
CIQ.UN,Canadian High Income Equity Fund,TSX
CM,Canadian Imperial Bank Of Commerce,TSX
NPS,Canadian Large Cap Leaders Split Corp.,TSX
LFE,Canadian Life Companies Split Corp.,TSX
CNR,Canadian National Railway Company,TSX
CNQ,Canadian Natural Resources Limited,TSX
CP,Canadian Pacific Kansas City Limited,TSX
CTC,\"Canadian Tire Corporation, Limited\",TSX
CU,Canadian Utilities Limited,TSX
CWB,Canadian Western Bank,TSX
CCM,Canagold Resources Ltd.,TSX
CFP,Canfor Corporation,TSX
CFX,Canfor Pulp Products Inc.,TSX
ICE,Canlan Ice Sports Corp.,TSX
EIT.PR.A,Canoe EIT Income Fund,TSX
WEED,Canopy Growth Corporation,TSX
PBY.UN,Canso Credit Income Fund,TSX
CPX,Capital Power Corporation,TSX
CS,Capstone Copper Corp.,TSX
CSE.PR.A,Capstone Infrastructure Corporation,TSX
CJ,Cardinal Energy Ltd.,TSX
CRDL,Cardiol Therapeutics Inc.,TSX
CRRX,CareRx Corporation,TSX
CJT,Cargojet Inc.,TSX
CUP.U,\"Caribbean Utilities Company, Ltd.\",TSX
CAS,Cascades Inc.,TSX
CET,Cathedral Energy Services Ltd.,TSX
CCL.A,CCL Industries Inc.,TSX
CLS,Celestica Inc.,TSX
CVE,Cenovus Energy Inc.,TSX
CEE,Centamin plc,TSX
CG,Centerra Gold Inc.,TSX
CNT,Century Global Commodities Corporation,TSX
CRP,Ceres Global Ag Corp.,TSX
CEU,CES Energy Solutions Corp.,TSX
GIB.A,CGI Inc.,TSX
CIA,Champion Iron Limited,TSX
CWEB,\"Charlotte's Web Holdings, Inc.\",TSX
CSH.UN,Chartwell Retirement Residences,TSX
CHE.DB.E,Chemtrade Logistics Income Fund,TSX
CHW,Chesswood Group Limited,TSX
CGG,China Gold International Resources Corp. Ltd.,TSX
CHP.UN,Choice Properties Real Estate Investment Trust,TSX
CHR,Chorus Aviation Inc.,TSX
BXF,CI 1-5 Year Laddered Government Strip Bond Index ETF,TSX
CMDO,CI Alternative Diversified Opportunities Fund,TSX
CRED,CI Alternative Investment Grade Credit Fund,TSX
CNAO,CI Alternative North American Opportunities Fund,TSX
CCOM,CI Auspice Broad Commodity Fund,TSX
CBAL,CI Balanced Asset Allocation ETF,TSX
CBGR,CI Balanced Growth Asset Allocation ETF,TSX
CBIN,CI Balanced Income Asset Allocation ETF,TSX
CDNA,CI Bio-Revolution Index ETF,TSX
CAGG,CI Canadian Aggregate Bond Index ETF,TSX
CIC,CI Canadian Banks Covered Call Income Class ETF,TSX
CXF,CI Canadian Convertible Bond ETF,TSX
RIT,CI Canadian REIT ETF,TSX
CAGS,CI Canadian Short-Term Aggregate Bond Index ETF,TSX
CCNV,CI Conservative Asset Allocation ETF,TSX
CBUG,CI Digital Security Index ETF,TSX
CDLB,CI DoubleLine Total Return Bond US$ Fund,TSX
CIEM,CI Emerging Markets Alpha ETF,TSX
NXF,CI Energy Giants Covered Call ETF,TSX
FGO,CI Enhanced Government Bond ETF,TSX
FSB,CI Enhanced Short Duration Bond Fund,TSX
CEQT,CI Equity Asset Allocation ETF,TSX
CIX,CI Financial Corp.,TSX
CFRT,CI Floating Rate Income Fund,TSX
BTCX.B,CI Galaxy Bitcoin ETF,TSX
CBCX,CI Galaxy Blockchain Index ETF,TSX
ETHX.B,CI Galaxy Ethereum ETF,TSX
CMVX,CI Galaxy Metaverse Index ETF,TSX
CMCX.B,CI Galaxy Multi-Crypto ETF,TSX
CINV,CI Global Alpha Innovation ETF,TSX
CIAI,CI Global Artificial Intelligence ETF,TSX
CGAA,CI Global Asset Allocation Private Pool,TSX
CLML,CI Global Climate Leaders Fund,TSX
FSF,CI Global Financial Sector ETF,TSX
CGRB,CI Global Green Bond Fund,TSX
CGHY,CI Global High Yield Credit Private Pool,TSX
CINF,CI Global Infrastructure Private Pool,TSX
CGIN,CI Global Investment Grade ETF,TSX
LONG,CI Global Longevity Economy Fund,TSX
CGDV,CI Global Minimum Downside Volatility Index ETF,TSX
CGRA,CI Global Real Asset Private Pool,TSX
CGRE,CI Global REIT Private Pool,TSX
CGRN,CI Global Sustainable Infrastructure Fund,TSX
VALT,CI Gold Bullion Fund,TSX
CGXF,CI Gold+ Giants Covered Call ETF,TSX
CGRO,CI Growth Asset Allocation ETF,TSX
FHI,CI Health Care Giants Covered Call ETF,TSX
CSAV,CI High Interest Savings ETF,TSX
CHNA.B,CI ICBCCS S&P China 500 Index ETF,TSX
FIG,CI Investment Grade Bond ETF,TSX
CMAR,CI Marret Alternative Absolute Return Bond Fund,TSX
CMEY,CI Marret Alternative Enhanced Yield Fund,TSX
CMNY,CI Money Market ETF,TSX
WXM,CI Morningstar Canada Momentum Index ETF,TSX
FXM,CI Morningstar Canada Value Index ETF,TSX
ZXM,CI Morningstar International Momentum Index ETF,TSX
VXM,CI Morningstar International Value Index ETF,TSX
QXM,CI Morningstar National Bank Québec Index ETF,TSX
CMAG,CI Munro Alternative Global Growth Fund,TSX
CMGG,CI Munro Global Growth Equity Fund,TSX
ONEQ,CI ONE Global Equity ETF,TSX
ONEB,CI ONE North American Core Plus Bond ETF,TSX
FPR,CI Preferred Share ETF,TSX
FGB,CI Short Term Government Bond Index Class ETF,TSX
TXF,CI Tech Giants Covered Call ETF,TSX
FLI,CI U.S. & Canada Lifeco Covered Call ETF,TSX
CMOM,CI U.S. Enhanced Momentum Index ETF,TSX
CVLU,CI U.S. Enhanced Value Index ETF,TSX
CUDV,CI U.S. Minimum Downside Volatility Index ETF,TSX
UMNY.U,CI U.S. Money Market ETF,TSX
CUTL,CI Utilities Giants Covered Call ETF,TSX
DGRC,CI WisdomTree Canada Quality Dividend Growth Index ETF,TSX
EMV.B,CI WisdomTree Emerging Markets Dividend Index ETF,TSX
EHE,CI WisdomTree Europe Hedged Equity Index ETF,TSX
IQD,CI WisdomTree International Quality Dividend Growth Index ETF,TSX
JAPN,CI WisdomTree Japan Equity Index ETF,TSX
UMI,CI WisdomTree U.S. MidCap Dividend Index ETF,TSX
DGR,CI WisdomTree U.S. Quality Dividend Growth Index ETF,TSX
CACB,CIBC Active Investment Grade Corporate Bond ETF,TSX
CAFR,CIBC Active Investment Grade Floating Rate Bond ETF,TSX
CCBI,CIBC Canadian Bond Index ETF,TSX
CCEI,CIBC Canadian Equity Index ETF,TSX
CSBI,CIBC Canadian Short-Term Bond Index ETF,TSX
CCNS,CIBC Conservative Fixed Income Pool,TSX
CCRE,CIBC Core Fixed Income Pool,TSX
CPLS,CIBC Core Plus Fixed Income Pool,TSX
CEMI,CIBC Emerging Markets Equity Index ETF,TSX
CFLX,CIBC Flexible Yield ETF (CAD-Hedged),TSX
CGBI,CIBC Global Bond ex-Canada Index ETF (CAD-Hedged),TSX
CGLO,CIBC Global Growth ETF,TSX
CINT,CIBC International Equity ETF,TSX
CIEI,CIBC International Equity Index ETF,TSX
CIEH,CIBC International Equity Index ETF (CAD-Hedged),TSX
CUEI,CIBC U.S. Equity Index ETF,TSX
CUEH,CIBC U.S. Equity Index ETF (CAD-Hedged),TSX
CGX,Cineplex Inc.,TSX
CPH,Cipher Pharmaceuticals Inc.,TSX
CTF.UN,Citadel Income Fund,TSX
CVG,Clairvest Group Inc.,TSX
CKI,Clarke Inc.,TSX
CCS.PR.C,Co-operators General Insurance Company,TSX
CCA,Cogeco Communications Inc.,TSX
CGO,Cogeco Inc.,TSX
GCL,Colabor Group Inc.,TSX
CNL,Collective Mining Ltd.,TSX
CIGI,Colliers International Group Inc.,TSX
YCM,Commerce Split Corp.,TSX
CMG,Computer Modelling Group Ltd.,TSX
CDR,Condor Energies Inc.,TSX
COG,Condor Gold plc,TSX
CFF,Conifex Timber Inc.,TSX
CSU,Constellation Software Inc.,TSX
CTS,Converge Technology Solutions Corp.,TSX
CPLF,Copperleaf Technologies Inc.,TSX
CSW.A,Corby Spirit and Wine Limited,TSX
CJR.B,Corus Entertainment Inc.,TSX
CVO,Coveo Solutions Inc.,TSX
CTX,Crescita Therapeutics Inc.,TSX
CR,Crew Energy Inc.,TSX
CRR.UN,Crombie Real Estate Investment Trust,TSX
CRON,Cronos Group Inc.,TSX
CRWN,Crown Capital Partners Inc.,TSX
CRT.UN,CT Real Estate Investment Trust,TSX
CIU.PR.A,CU Inc.,TSX
CURA,\"Curaleaf Holdings, Inc.\",TSX
CXI,\"Currency Exchange International, Corp.\",TSX
CYB,Cymbria Corporation,TSX
DBO,D-Box Technologies Inc.,TSX
DTOL,D2L Inc.,TSX
DCM,Data Communications Management Corp.,TSX
DAY,\"Dayforce, Inc.\",TSX
DFY,Definity Financial Corporation,TSX
DN,Delta 9 Cannabis Inc.,TSX
DML,Denison Mines Corp.,TSX
DNTL,dentalcorp Holdings Ltd.,TSX
DSG,Descartes Systems Group Inc. (The),TSX
DCC,Desjardins 1-5 year Laddered Canadian Corporate Bond Index ETF,TSX
DCG,Desjardins 1-5 year Laddered Canadian Government Bond Index ETF,TSX
DANC,Desjardins Alt Long/Short Equity Market Neutral ETF,TSX
DAMG,Desjardins Alt Long/Short Global Equity Markets ETF,TSX
DMEU,Desjardins American Equity Index ETF,TSX
DCBC,Desjardins Canadian Corporate Bond Index ETF,TSX
DMEC,Desjardins Canadian Equity Index ETF,TSX
DCP,Desjardins Canadian Preferred Share Index ETF,TSX
DCS,Desjardins Canadian Short Term Bond Index ETF,TSX
DCU,Desjardins Canadian Universe Bond Index ETF,TSX
DMEI,Desjardins International Equity Index ETF,TSX
DRCU,Desjardins RI Active Canadian Bond - Net-Zero Emissions Pathway ETF,TSX
DRMC,Desjardins RI Canada - Net-Zero Emissions Pathway ETF,TSX
DRFC,Desjardins RI Canada Multifactor - Net-Zero Emissions Pathway ETF,TSX
DRMD,Desjardins RI Developed ex-USA ex-Canada - Net-Zero Emissions Pathway ETF,TSX
DRME,Desjardins RI Emerging Markets - Net-Zero Emissions Pathway ETF,TSX
DRFE,Desjardins RI Emerging Markets Multifactor - Net-Zero Emissions Pathway ETF,TSX
DRFG,Desjardins RI Global Multifactor - Fossil Fuel Reserves Free ETF,TSX
DRMU,Desjardins RI USA - Net-Zero Emissions Pathway ETF,TSX
DRFU,Desjardins RI USA Multifactor - Net-Zero Emissions Pathway ETF,TSX
DSAE,Desjardins Sustainable American Equity ETF,TSX
DRFD,DesjardinsRIDeveloped ex-USAex-Canada Multifactor-Net-ZeroEmissionsPathwayETF,TSX
DXT,Dexterra Group Inc.,TSX
DRT,DIRTT Environmental Solutions Ltd.,TSX
DSV,Discovery Silver Corp.,TSX
DIV,Diversified Royalty Corp.,TSX
DFN,Dividend 15 Split Corp.,TSX
DF,Dividend 15 Split Corp. II,TSX
DGS,Dividend Growth Split Corp.,TSX
DS,Dividend Select 15 Corp.,TSX
DCBO,Docebo Inc.,TSX
DOL,Dollarama Inc.,TSX
DBM,Doman Building Materials Group Ltd.,TSX
DLCG,Dominion Lending Centres Inc.,TSX
DII.A,Dorel Industries Inc.,TSX
MPCT.DB,Dream Impact Trust,TSX
DIR.UN,Dream Industrial Real Estate Investment Trust,TSX
D.UN,Dream Office Real Estate Investment Trust,TSX
DRR.U,Dream Residential Real Estate Investment Trust,TSX
DRM,DREAM Unlimited Corp.,TSX
DHT.U,DRI Healthcare Trust,TSX
DC.A,Dundee Corporation,TSX
DPM,Dundee Precious Metals Inc.,TSX
DND,Dye & Durham Limited,TSX
DYA,dynaCERT Inc.,TSX
DNG,Dynacor Group Inc.,TSX
DXBC,Dynamic Active Canadian Bond ETF,TSX
DXC,Dynamic Active Canadian Dividend ETF,TSX
DXO,Dynamic Active Crossover Bond ETF,TSX
DXDB,Dynamic Active Discount Bond ETF,TSX
DXEM,Dynamic Active Emerging Markets ETF,TSX
DXET,Dynamic Active Energy Evolution ETF,TSX
DXQ,Dynamic Active Enhanced Yield Covered Options ETF,TSX
DXG,Dynamic Active Global Dividend ETF,TSX
DXGE,Dynamic Active Global Equity Income ETF,TSX
DXF,Dynamic Active Global Financial Services ETF,TSX
DXN,Dynamic Active Global Infrastructure ETF,TSX
DXW,Dynamic Active International Dividend ETF,TSX
DXIF,Dynamic Active International ETF,TSX
DXV,Dynamic Active Investment Grade Floating Rate ETF,TSX
DXP,Dynamic Active Preferred Shares ETF,TSX
DXR,Dynamic Active Retirement Income ETF,TSX
DXB,Dynamic Active Tactical Bond ETF,TSX
DXU,Dynamic Active U.S. Dividend ETF,TSX
DXUS,Dynamic Active U.S. Equity ETF,TSX
DXBU,Dynamic Active U.S. Investment Grade Corporate Bond ETF,TSX
DXZ,Dynamic Active U.S. Mid-Cap ETF,TSX
ENS,E Split Corp.,TSX
ELF,E-L Financial Corporation Limited,TSX
EAGR,East Side Games Group Inc.,TSX
ELR,Eastern Platinum Limited,TSX
ECN,ECN Capital Corp.,TSX
EVT,Economic Investment Trust Limited,TSX
ECOR,Ecora Resources plc,TSX
ECO,EcoSynthetix Inc.,TSX
ELD,Eldorado Gold Corporation,TSX
ELVA,Electrovaya Inc.,TSX
EFN,Element Fleet Management Corp.,TSX
ELO,Eloro Resources Ltd.,TSX
BABY,Else Nutrition Holdings Inc.,TSX
EMA,Emera Incorporated,TSX
EMP.A,Empire Company Limited,TSX
ENB,Enbridge Inc.,TSX
EDV,Endeavour Mining plc,TSX
EDR,Endeavour Silver Corp.,TSX
EFX,Enerflex Ltd.,TSX
EFR,Energy Fuels Inc.,TSX
ENI.UN,Energy Income Fund,TSX
ERF,Enerplus Corporation,TSX
ENGH,Enghouse Systems Limited,TSX
ESI,Ensign Energy Services Inc.,TSX
E,\"Enterprise Group, Inc.\",TSX
EGLX,Enthusiast Gaming Holdings Inc.,TSX
ETG,Entree Resources Ltd.,TSX
EQB,EQB Inc.,TSX
EQX,Equinox Gold Corp.,TSX
ERD,Erdene Resource Development Corporation,TSX
ERO,Ero Copper Corp.,TSX
EPRX,Eupraxia Pharmaceuticals Inc.,TSX
ESM,Euro Sun Mining Inc.,TSX
ERE.UN,European Residential Real Estate Investment Trust,TSX
ET,Evertz Technologies Limited,TSX
DIVS,Evolve Active Canadian Preferred Share Fund,TSX
EARN,Evolve Active Global Fixed Income Fund,TSX
ARTI,Evolve Artificial Intelligence Fund,TSX
CARS,Evolve Automobile Innovation Index Fund,TSX
EBIT,Evolve Bitcoin ETF,TSX
BANK,Evolve Canadian Banks and Lifecos Enhanced Yield Index Fund,TSX
DATA,Evolve Cloud Computing Index Fund,TSX
ETC,Evolve Cryptocurrencies ETF,TSX
CYBR,Evolve Cyber Security Index Fund,TSX
HERO,Evolve E-Gaming Index ETF,TSX
BOND,Evolve Enhanced Yield Bond Fund,TSX
ETHR,Evolve Ether ETF,TSX
EBNK,Evolve European Banks Enhanced Yield ETF,TSX
TECH,Evolve FANGMA Index ETF,TSX
LEAD,Evolve Future Leadership Fund,TSX
LIFE,Evolve Global Healthcare Enhanced Yield Fund,TSX
BASE,Evolve Global Materials & Mining Enhanced Yield Index ETF,TSX
EDGE,Evolve Innovation Index Fund,TSX
QQQY,Evolve NASDAQ Technology Enhanced Yield Index Fund,TSX
QQQT,Evolve NASDAQ Technology Index Fund,TSX
ESPX,Evolve S&P 500® Enhanced Yield Fund,TSX
ETSX,Evolve S&P/TSX 60 Enhanced Yield Fund,TSX
CALL,Evolve US Banks Enhanced Yield Fund,TSX
EVO,Evovest Global Equity ETF,TSX
EXN,Excellon Resources Inc.,TSX
MIN,Excelsior Mining Corp.,TSX
EIF,Exchange Income Corporation,TSX
XTC,Exco Technologies Limited,TSX
EGIF,Exemplar Growth and Income Fund,TSX
EXRO,Exro Technologies Inc.,TSX
EXE,Extendicare Inc.,TSX
FFH,Fairfax Financial Holdings Limited,TSX
FIH.U,Fairfax India Holdings Corporation,TSX
FANS,FansUnite Entertainment Inc.,TSX
FDY,Faraday Copper Corp.,TSX
FRX,Fennec Pharmaceuticals Inc.,TSX
FGAA.U,FG Acquisition Corp.,TSX
FBTC,Fidelity Advantage Bitcoin ETF,TSX
FETH,Fidelity Advantage Ether ETF,TSX
FCCD,Fidelity Canadian High Dividend ETF,TSX
FCCQ,Fidelity Canadian High Quality ETF,TSX
FCMI,Fidelity Canadian Monthly High Income ETF,TSX
FCCV,Fidelity Canadian Value ETF,TSX
FCEM,Fidelity Emerging Markets Fund,TSX
FGEB,Fidelity Global Equity+ Balanced Fund,TSX
FGEP,Fidelity Global Equity+ Fund,TSX
FCGI,Fidelity Global Monthly High Income ETF,TSX
FCID,Fidelity International High Dividend ETF,TSX
FCIQ,Fidelity International High Quality ETF,TSX
FCIV,Fidelity International Value ETF,TSX
FTHI,Fidelity Tactical High Income Fund,TSX
FCRH,Fidelity U.S. Dividend for Rising Rates Currency Neutral ETF,TSX
FCRR,Fidelity U.S. Dividend for Rising Rates ETF,TSX
FCUH,Fidelity U.S. High Dividend Currency Neutral ETF,TSX
FCUD,Fidelity U.S. High Dividend ETF,TSX
FCQH,Fidelity U.S. High Quality Currency Neutral ETF,TSX
FCUQ,Fidelity U.S. High Quality ETF,TSX
FCVH,Fidelity U.S. Value Currency Neutral ETF,TSX
FCUV,Fidelity U.S. Value ETF,TSX
FSZ,Fiera Capital Corporation,TSX
FIL,Filo Corp,TSX
FTN,Financial 15 Split Corp.,TSX
FTT,Finning International Inc.,TSX
FTG,Firan Technology Group Corporation,TSX
FC,Firm Capital Mortgage Investment Corporation,TSX
FCD.UN,Firm Capital Property Trust,TSX
FCR.UN,First Capital Real Estate Investment Trust,TSX
FR,First Majestic Silver Corp.,TSX
FF,First Mining Gold Corp.,TSX
FN,First National Financial Corporation,TSX
FM,First Quantum Minerals Ltd.,TSX
FHH,First Trust AlphaDEX U.S. Health Care Sector Index ETF,TSX
FHG,First Trust AlphaDEX U.S. Industrials Sector Index ETF,TSX
FHQ,First Trust AlphaDEX U.S. Technology Sector Index ETF,TSX
FST,First Trust Canadian Capital Strength ETF,TSX
SKYY,First Trust Cloud Computing ETF,TSX
FDN,First Trust Dow Jones Internet ETF,TSX
ETP,First Trust Global Risk Managed Income Index ETF,TSX
BLCK,First Trust Indxx Innovative Transaction and Process ETF,TSX
NXTG,First Trust Indxx NextG ETF,TSX
FINT,First Trust International Capital Strength ETF,TSX
FDL,First Trust Morningstar Dividend Leaders ETF (CAD-Hedged),TSX
QCLN,First Trust NASDAQ Clean Edge Green Energy ETF,TSX
CIBR,First Trust Nasdaq Cybersecurity ETF,TSX
FBT,First Trust NYSE Arca Biotechnology ETF,TSX
FSL,First Trust Senior Loan ETF (CAD-Hedged),TSX
FUD,First Trust Value Line Dividend Index ETF (CAD-Hedged),TSX
AUGB.F,First Trust Vest U.S. Equity Buffer ETF - August,TSX
FEBB.F,First Trust Vest U.S. Equity Buffer ETF - February,TSX
MAYB.F,First Trust Vest U.S. Equity Buffer ETF - May,TSX
NOVB.F,First Trust Vest U.S. Equity Buffer ETF - November,TSX
FSV,FirstService Corporation,TSX
FCU,Fission Uranium Corp.,TSX
MHC.U,Flagship Communities Real Estate Investment Trust,TSX
FLNT,FLINT Corp.,TSX
FLOW,Flow Beverage Corp.,TSX
FAR,Foraco International SA,TSX
FOM,Foran Mining Corporation,TSX
FEME,Forstrong Emerging Markets Equity ETF,TSX
FINE,Forstrong Global Ex-North America Equity ETF,TSX
FGRW,Forstrong Global Growth ETF,TSX
FINC,Forstrong Global Income ETF,TSX
FSY,Forsys Metals Corp.,TSX
FTS,Fortis Inc.,TSX
FVI,Fortuna Silver Mines Inc.,TSX
FT,Fortune Minerals Limited,TSX
FNV,Franco-Nevada Corporation,TSX
FBGO,Franklin Brandywine Global Sustainable Income Optimiser Fund,TSX
FLCP,Franklin Canadian Core Plus Bond Fund,TSX
FLCI,Franklin Canadian Corporate Bond Fund,TSX
FGOV,Franklin Canadian Government Bond Fund,TSX
FLSD,Franklin Canadian Short Term Bond Fund,TSX
FHIS,Franklin Canadian Ultra Short Term Bond Fund,TSX
FCII,Franklin ClearBridge Sustainable Global Infrastructure Income Fund,TSX
FCSI,Franklin ClearBridge Sustainable International Growth Fund,TSX
FLGA,Franklin Global Core Bond Fund,TSX
FLGD,Franklin Global Dividend Quality Index ETF,TSX
FGGE,Franklin Global Growth Fund,TSX
FINO,Franklin Innovation Fund,TSX
FLDM,Franklin International Multifactor Index ETF,TSX
FLUS,Franklin U.S. Large Cap Multifactor Index ETF,TSX
FWCP,Franklin Western Asset Core Plus Bond Fund,TSX
FVL,Freegold Ventures Limited,TSX
FRU,Freehold Royalties Ltd.,TSX
FEC,Frontera Energy Corporation,TSX
FURY,Fury Gold Mines Limited,TSX
GMIN,G Mining Ventures Corp.,TSX
GTWO,G2 Goldfields Inc.,TSX
GLXY,Galaxy Digital Holdings Ltd.,TSX
GAU,Galiano Gold Inc.,TSX
GH,Gamehost Inc.,TSX
VRTS,Gamelancer Media Corp.,TSX
GATO,\"Gatos Silver, Inc.\",TSX
GDI,GDI Integrated Facility Services Inc.,TSX
GXE,Gear Energy Ltd.,TSX
GENM,Generation Mining Limited,TSX
GDC,Genesis Land Development Corp.,TSX
GEO,Geodrill Limited,TSX
WN,George Weston Limited,TSX
GFL,GFL Environmental Inc.,TSX
GEI,Gibson Energy Inc.,TSX
GIL,Gildan Activewear Inc.,TSX
GIVX,Givex Corp.,TSX
GVC,Glacier Media Inc.,TSX
GLG,GLG Life Tech Corporation,TSX
GLO,Global Atomic Corporation,TSX
GDV,Global Dividend Growth Split Corp.,TSX
GEC,Global Education Communities Corp.,TSX
CBIL,Global X 0-3 Month T-Bill ETF,TSX
UBIL.U,Global X 0-3 Month U.S. T-Bill ETF,TSX
HAD,Global X Active Canadian Bond ETF,TSX
HAL,Global X Active Canadian Dividend ETF,TSX
HMP,Global X Active Canadian Municipal Bond ETF,TSX
HAB,Global X Active Corporate Bond ETF,TSX
HAZ,Global X Active Global Dividend ETF,TSX
HAF,Global X Active Global Fixed Income ETF,TSX
HYBR,Global X Active Hybrid Bond and Preferred Share ETF,TSX
HPR,Global X Active Preferred Share ETF,TSX
HFR,Global X Active Ultra-Short Term Investment Grade Bond ETF,TSX
HEQT,Global X All-Equity Asset Allocation ETF,TSX
AIGO,Global X Artificial Intelligence & Technology Index ETF,TSX
HBAL,Global X Balanced Asset Allocation ETF,TSX
HBGD,Global X Big Data & Hardware Index ETF,TSX
HXH,Global X Canadian High Dividend Index Corporate Class ETF,TSX
ENCC,Global X Canadian Oil and Gas Equity Covered Call ETF,TSX
HBB,Global X Canadian Select Universe Bond Index Corporate Class ETF,TSX
UTIL,Global X Canadian Utility Services High Dividend Index ETF,TSX
CARB,Global X Carbon Credits ETF,TSX
HSAV,Global X Cash Maximizer Corporate Class ETF,TSX
HCON,Global X Conservative Asset Allocation ETF,TSX
COPP,Global X Copper Producers Index ETF,TSX
HUC,Global X Crude Oil ETF,TSX
HBUG,Global X Cybersecurity Index ETF,TSX
HXEM,Global X Emerging Markets Equity Index Corporate Class ETF,TSX
EQCL,Global X Enhanced All-Equity Asset Allocation Covered Call ETF,TSX
HEQL,Global X Enhanced All-Equity Asset Allocation ETF,TSX
ENCL,Global X Enhanced Canadian Oil and Gas Equity Covered Call ETF,TSX
BNKL,Global X Enhanced Equal Weight Banks Index ETF,TSX
BKCL,Global X Enhanced Equal Weight Canadian Banks Covered Call ETF,TSX
QQCL,Global X Enhanced Nasdaq-100 Covered Call ETF,TSX
QQQL,Global X Enhanced Nasdaq-100 Index ETF,TSX
USCL,Global X Enhanced S&P 500 Covered Call ETF,TSX
USSL,Global X Enhanced S&P 500 Index ETF,TSX
CNCL,Global X Enhanced S&P/TSX 60 Covered Call ETF,TSX
CANL,Global X Enhanced S&P/TSX 60 Index ETF,TSX
BKCC,Global X Equal Weight Canadian Bank Covered Call ETF,TSX
HEWB,Global X Equal Weight Canadian Banks Index Corporate Class ETF,TSX
HBNK,Global X Equal Weight Canadian Banks Index ETF,TSX
HCRE,Global X Equal Weight Canadian REITs Index Corporate Class ETF,TSX
HXX,Global X Europe 50 Index Corporate Class ETF,TSX
ETHI,Global X Global Sustainability Leaders Index ETF,TSX
HUG,Global X Gold ETF,TSX
GLCC,Global X Gold Producer Equity Covered Call ETF,TSX
HGY,Global X Gold Yield ETF,TSX
GRCC,Global X Growth Asset Allocation Covered Call ETF,TSX
HGRW,Global X Growth Asset Allocation ETF,TSX
CASH,Global X High Interest Savings ETF,TSX
FOUR,Global X Industry 4.0 Index ETF,TSX
TTTX,Global X Innovative Bluechip Top 10 Index ETF,TSX
INOC,Global X Inovestor Canadian Equity Index ETF,TSX
HXDM,Global X Intl Developed Markets Equity Index Corporate Class ETF,TSX
HLPR,Global X Laddered Canadian Preferred Share Index Corporate Class ETF,TSX
HLIT,Global X Lithium Producers Index ETF,TSX
LPAY,Global X Long-Term U.S. Treasury Premium Yield ETF,TSX
HMMJ,Global X Marijuana Life Sciences Index ETF,TSX
MTAV,Global X Metaverse Index ETF,TSX
MPAY,Global X Mid-Term U.S. Treasury Premium Yield ETF,TSX
QQCC,Global X Nasdaq-100 Covered Call ETF,TSX
HXQ,Global X Nasdaq-100 Index Corporate Class ETF,TSX
QQQX,Global X Nasdaq-100 Index ETF,TSX
HUN,Global X Natural Gas ETF,TSX
PPLN,Global X Pipelines & Energy Services Index ETF,TSX
HRAA,Global X ReSolve Adaptive Asset Allocation Corporate Class ETF,TSX
RBOT,Global X Robotics & AI Index ETF,TSX
HSH,Global X S&P 500 CAD Hedged Index Corporate Class ETF,TSX
USCC,Global X S&P 500 Covered Call ETF,TSX
HXS,Global X S&P 500 Index Corporate Class ETF,TSX
USSX,Global X S&P 500 Index ETF,TSX
HGGB,Global X S&P Green Bond Index ETF,TSX
CNCC,Global X S&P/TSX  60 Covered Call ETF,TSX
HXT,Global X S&P/TSX 60 Index Corporate Class ETF,TSX
CNDX,Global X S&P/TSX 60 Index ETF,TSX
HXCN,Global X S&P/TSX Capped Composite Index Corporate Class ETF,TSX
HXE,Global X S&P/TSX Capped Energy Index Corporate Class ETF,TSX
HXF,Global X S&P/TSX Capped Financials Index Corporate Class ETF,TSX
HAC,Global X Seasonal Rotation ETF,TSX
CHPS,Global X Semiconductor Index ETF,TSX
PAYS,Global X Short-Term Government Bond Premium Yield ETF,TSX
SPAY,Global X Short-Term U.S. Treasury Premium Yield ETF,TSX
HUZ,Global X Silver ETF,TSX
HURA,Global X Uranium Index ETF,TSX
HTB,Global X US 7-10 Year Treasury Bond Index Corporate Class ETF,TSX
DLR,Global X US Dollar Currency ETF,TSX
HULC,Global X US Large Cap Index Corporate Class ETF,TSX
HSUV.U,Global X USD Cash Maximizer Corporate Class ETF,TSX
UCSH.U,Global X USD High Interest Savings ETF,TSX
GMX,Globex Mining Enterprises Inc.,TSX
GSY,goeasy Ltd.,TSX
GGD,GoGold Resources Inc.,TSX
GMTN,Gold Mountain Mining Corp.,TSX
GRC,Gold Springs Resource Corp.,TSX
AUMN,Golden Minerals Company,TSX
GOLD,GoldMining Inc.,TSX
XAU,GoldMoney Inc.,TSX
GDL,Goodfellow Inc.,TSX
FOOD,Goodfood Market Corp.,TSX
GTE,Gran Tierra Energy Inc.,TSX
GRT.UN,Granite Real Estate Investment Trust,TSX
GWO,Great-West Lifeco Inc.,TSX
GFR,Greenfire Resources Ltd.,TSX
GFP,GreenFirst Forest Products Inc.,TSX
GRN,Greenlane Renewables Inc.,TSX
GCBD,Guardian Canadian Bond Fund,TSX
GCFE,Guardian Canadian Focused Equity Fund,TSX
GCSC,Guardian Canadian Sector Controlled Equity Fund,TSX
GCG,Guardian Capital Group Limited,TSX
GDEP,Guardian Directed Equity Path Portfolio,TSX
GDPY,Guardian Directed Premium Yield Portfolio,TSX
GIQG,Guardian i3 Global Quality Growth ETF,TSX
GIQU,Guardian i3 US Quality Growth ETF,TSX
GIES,Guardian International Equity Select Fund,TSX
GIGC,Guardian Investment Grade Corporate Bond Fund,TSX
GCTB,Guardian Ultra-Short Canadian T-Bill Fund,TSX
GUTB.U,Guardian Ultra-Short U.S. T-Bill Fund,TSX
GPMD,GuardPath TM Managed Decumulation 2042 Fund,TSX
GURU,GURU Organic Energy Corp.,TSX
HR.UN,H&R Real Estate Investment Trust,TSX
HAI,Haivision Systems Inc.,TSX
HBA,Hamilton Australian Bank Equal-Weight Index ETF,TSX
HEB,Hamilton Canadian Bank Equal-Weight Index ETF,TSX
HCA,Hamilton Canadian Bank Mean Reversion Index ETF,TSX
HMAX,Hamilton Canadian Financials Yield Maximizer ETF,TSX
EMAX,Hamilton Energy Yield Maximizer ETF,TSX
HCAL,Hamilton Enhanced Canadian Bank ETF,TSX
HFIN,Hamilton Enhanced Canadian Financials ETF,TSX
HDIV,Hamilton Enhanced Multi-Sector Covered Call ETF,TSX
HYLD,Hamilton Enhanced U.S. Covered Call ETF,TSX
HUTS,Hamilton Enhanced Utilities ETF,TSX
HFG,Hamilton Global Financials ETF,TSX
AMAX,Hamilton Gold Producer Yield Maximizer ETF,TSX
LMAX,Hamilton Healthcare Yield Maximizer ETF,TSX
QMAX,Hamilton Technology Yield Maximizer ETF,TSX
HTL,Hamilton Thorne Ltd.,TSX
HBND,Hamilton U.S. Bond Yield Maximizer ETF,TSX
SMAX,Hamilton U.S. Equity Yield Maximizer ETF,TSX
FMAX,Hamilton U.S. Financials Yield Maximizer ETF,TSX
HUM,Hamilton U.S. Mid-Cap Financials ETF,TSX
UMAX,Hamilton Utilities Yield Maximizer ETF,TSX
HMM.A,Hammond Manufacturing Company Limited,TSX
HPS.A,Hammond Power Solutions Inc.,TSX
HBIE,Harvest Balanced Income & Growth Enhanced ETF,TSX
HBIG,Harvest Balanced Income & Growth ETF,TSX
HBFE,Harvest Brand Leaders Enhanced Income ETF,TSX
HBF,Harvest Brand Leaders Plus Income ETF,TSX
HLFE,Harvest Canadian Equity Enhanced Income Leaders ETF,TSX
HLIF,Harvest Canadian Equity Income Leaders ETF,TSX
TBIL,Harvest Canadian T-Bill ETF,TSX
HCLN,Harvest Clean Energy ETF,TSX
HRIF,Harvest Diversified Equity Income ETF,TSX
HDIF,Harvest Diversified Monthly Income ETF,TSX
HPF,Harvest Energy Leaders Plus Income ETF,TSX
HUTE,Harvest Equal Weight Global Utilities Enhanced Income ETF,TSX
HUTL,Harvest Equal Weight Global Utilities Income ETF,TSX
HGGG,Harvest Global Gold Giants Index ETF,TSX
HGR,Harvest Global REIT Leaders Income ETF,TSX
HHLE,Harvest Healthcare Leaders Enhanced Income ETF,TSX
HHL,Harvest Healthcare Leaders Income ETF,TSX
HIND,Harvest Industrial Leaders Income ETF,TSX
HPYM,Harvest Premium Yield 7-10 Year Treasury ETF,TSX
HPYT,Harvest Premium Yield Treasury ETF,TSX
HTAE,Harvest Tech Achievers Enhanced Income ETF,TSX
HTA,Harvest Tech Achievers Growth & Income ETF,TSX
TRVI,Harvest Travel & Leisure Income ETF,TSX
TRVL,Harvest Travel & Leisure Index ETF,TSX
HUBL,Harvest US Bank Leaders Income ETF,TSX
HWX,Headwater Exploration Inc.,TSX
MDS.UN,Healthcare Special Opportunities Fund,TSX
AIDX,HealWELL AI Inc.,TSX
HFPC.U,Helios Fairfax Partners Corporation,TSX
HBP,Helix BioPharma Corp.,TSX
HRX,Heroux-Devtek Inc.,TSX
HWO,High Arctic Energy Services Inc.,TSX
HLF,High Liner Foods Incorporated,TSX
HLS,HLS Therapeutics Inc.,TSX
HBM,Hudbay Minerals Inc.,TSX
HUT,Hut 8 Corp.,TSX
H,Hydro One Limited,TSX
IAU,i-80 Gold Corp.,TSX
ITE,i3 Energy plc,TSX
ICPB,IA Clarington Core Plus Bond Fund,TSX
IFRF,IA Clarington Floating Rate Income Fund,TSX
IGAF,IA Clarington Loomis Global Allocation Fund,TSX
IGEO,IA Clarington Loomis Global Equity Opportunities Fund,TSX
ILGB,IA Clarington Loomis Global Multisector Bond Fund,TSX
ISCB,IA Clarington Strategic Corporate Bond Fund,TSX
ISIF,IA Clarington Strategic Income Fund,TSX
IAG,iA Financial Corporation Inc.,TSX
IWEB,IA Wealth Enhanced Bond Pool,TSX
IMG,IAMGOLD Corporation,TSX
IBG.DB.E,IBI Group Inc.,TSX
IFA,iFabric Corp.,TSX
IGM,IGM Financial Inc.,TSX
ILLM,illumin Holdings Inc.,TSX
III,Imperial Metals Corporation,TSX
IMO,Imperial Oil Limited,TSX
INC.UN,Income Financial Trust,TSX
IDG,Indigo Books & Music Inc.,TSX
IAF.PR.B,Industrial Alliance Insurance and Financial Services Inc.,TSX
ISV,Information Services Corporation,TSX
IS,Infrastructure Dividend Split Corp.,TSX
INE,Innergex Renewable Energy Inc.,TSX
INO.UN,Inovalis Real Estate Investment Trust,TSX
IPO,InPlay Oil Corp.,TSX
IFC,Intact Financial Corporation,TSX
IFP,Interfor Corporation,TSX
IMP,Intermap Technologies Corporation,TSX
IPCO,International Petroleum Corporation,TSX
ITH,International Tower Hill Mines Ltd.,TSX
IIP.UN,InterRent Real Estate Investment Trust,TSX
PFL,Invesco 1-3 Year Laddered Floating Rate Note Index ETF,TSX
PSB,Invesco 1-5 Year Laddered Investment Grade Corporate Bond Index ETF,TSX
PDC,Invesco Canadian Dividend Index ETF,TSX
BESG,Invesco ESG Canadian Core Plus Bond ETF,TSX
IWBE,Invesco ESG Global Bond ETF,TSX
QQCE,Invesco ESG NASDAQ 100 Index ETF,TSX
QQJE,Invesco ESG NASDAQ Next Gen 100 Index ETF,TSX
PXC,Invesco FTSE RAFI Canadian Index ETF,TSX
PZW,Invesco FTSE RAFI Global Small-Mid ETF,TSX
PXU.F,Invesco FTSE RAFI U.S. Index ETF,TSX
PXS,Invesco FTSE RAFI U.S. Index ETF II,TSX
PFH.F,Invesco Fundamental High Yield Corporate Bond Index ETF,TSX
IIMF,Invesco International Developed Dynamic-Multifactor Index ETF,TSX
PLV,Invesco Low Volatility Portfolio ETF,TSX
IGET,Invesco Morningstar Global Energy Transition Index ETF,TSX
INAI,Invesco Morningstar Global Next Gen AI Index ETF,TSX
QQEQ,Invesco NASDAQ 100 Equal Weight Index ETF,TSX
QQC,Invesco NASDAQ 100 Index ETF,TSX
QQJR,Invesco NASDAQ Next Gen 100 Index ETF,TSX
IUMF,Invesco Russell 1000 Dynamic-Multifactor Index ETF,TSX
EQL,Invesco S&P 500 Equal Weight Index ETF,TSX
ESG,Invesco S&P 500 ESG Index ETF,TSX
ISTE,Invesco S&P 500 ESG Tilt Index ETF,TSX
ULV.C,Invesco S&P 500 Low Volatility Index ETF,TSX
IIAE,Invesco S&P International Developed Dividend Aristocrats ESG Index ETF,TSX
IICE,Invesco S&P International Developed ESG Index ETF,TSX
IITE,Invesco S&P International Developed ESG Tilt Index ETF,TSX
IUAE,Invesco S&P US Dividend Aristocrats ESG Index ETF,TSX
IUCE,Invesco S&P US Total Market ESG Index ETF,TSX
IUTE,Invesco S&P US Total Market ESG Tilt Index ETF,TSX
IXTE,Invesco S&P/TSX 60 ESG Tilt Index ETF,TSX
ICAE,Invesco S&P/TSX Canadian Dividend Aristocrats ESG Index ETF,TSX
ESGC,Invesco S&P/TSX Composite ESG Index ETF,TSX
ICTE,Invesco S&P/TSX Composite ESG Tilt Index ETF,TSX
TLV,Invesco S&P/TSX Composite Low Volatility Index ETF,TSX
IUFR.U,Invesco US Treasury Floating Rate Note Index ETF (USD),TSX
IVQ,Invesque Inc.,TSX
XSTP,iShares 0-5 Year TIPS Bond Index ETF,TSX
XSTH,iShares 0-5 Year TIPS Bond Index ETF (CAD-Hedged),TSX
CBH,iShares 1-10 Year Laddered Corporate Bond Index ETF,TSX
CLG,iShares 1-10 Year Laddered Government Bond Index ETF,TSX
CBO,iShares 1-5 Year Laddered Corporate Bond Index ETF,TSX
CLF,iShares 1-5 Year Laddered Government Bond Index ETF,TSX
XSHU,iShares 1-5 Year U.S. IG Corporate Bond Index ETF,TSX
XIGS,iShares 1-5 Year U.S. IG Corporate Bond Index ETF (CAD-Hedged),TSX
XTLT,iShares 20+ Year U.S. Treasury Bond Index ETF,TSX
XTLH,iShares 20+ Year U.S. Treasury Bond Index ETF (CAD-Hedged),TSX
FIE,iShares Canadian Financial Monthly Income ETF,TSX
XCG,iShares Canadian Growth Index ETF,TSX
XHB,iShares Canadian HYBrid Corporate Bond Index ETF,TSX
XRB,iShares Canadian Real Return Bond Index ETF,TSX
XDV,iShares Canadian Select Dividend Index ETF,TSX
XCV,iShares Canadian Value Index ETF,TSX
XCH,iShares China Index ETF,TSX
XSC,iShares Conservative Short Term Strategic Fixed Income ETF,TSX
XSE,iShares Conservative Strategic Fixed Income ETF,TSX
CVD,iShares Convertible Bond Index ETF,TSX
XBAL,iShares Core Balanced ETF Portfolio,TSX
XFLB,iShares Core Canadian 15+ Year Federal Bond Index ETF,TSX
XCB,iShares Core Canadian Corporate Bond Index ETF,TSX
XGB,iShares Core Canadian Government Bond Index ETF,TSX
XLB,iShares Core Canadian Long Term Bond Index ETF,TSX
XSB,iShares Core Canadian Short Term Bond Index ETF,TSX
XSH,iShares Core Canadian Short Term Corporate Bond Index ETF,TSX
XBB,iShares Core Canadian Universe Bond Index ETF,TSX
XCNS,iShares Core Conservative Balanced ETF Portfolio,TSX
XEQT,iShares Core Equity ETF Portfolio,TSX
XGRO,iShares Core Growth ETF Portfolio,TSX
XINC,iShares Core Income Balanced ETF Portfolio,TSX
XAW,iShares Core MSCI All Country World ex Canada Index ETF,TSX
XDIV,iShares Core MSCI Canadian Quality Dividend Index ETF,TSX
XEF,iShares Core MSCI EAFE IMI Index ETF,TSX
XFH,iShares Core MSCI EAFE IMI Index ETF (CAD-Hedged),TSX
XEC,iShares Core MSCI Emerging Markets IMI Index ETF,TSX
XDG,iShares Core MSCI Global Quality Dividend Index ETF,TSX
XDGH,iShares Core MSCI Global Quality Dividend Index ETF (CAD-Hedged),TSX
XDU,iShares Core MSCI US Quality Dividend Index ETF,TSX
XDUH,iShares Core MSCI US Quality Dividend Index ETF (CAD-Hedged),TSX
XUS,iShares Core S&P 500 Index ETF,TSX
XSP,iShares Core S&P 500 Index ETF (CAD-Hedged),TSX
XUU,iShares Core S&P U.S. Total Market Index ETF,TSX
XUH,iShares Core S&P U.S. Total Market Index ETF (CAD-Hedged),TSX
XIC,iShares Core S&P/TSX Capped Composite Index ETF,TSX
XHAK,iShares Cybersecurity and Tech Index ETF,TSX
XTR,iShares Diversified Monthly Income ETF,TSX
CEW,iShares Equal Weight Banc & Lifeco ETF,TSX
XSHG,iShares ESG Advanced 1-5 Year Canadian Corporate Bond Index ETF,TSX
XCBG,iShares ESG Advanced Canadian Corporate Bond Index ETF,TSX
XCSR,iShares ESG Advanced MSCI Canada Index ETF,TSX
XDSR,iShares ESG Advanced MSCI EAFE Index ETF,TSX
XUSR,iShares ESG Advanced MSCI USA Index ETF,TSX
XSAB,iShares ESG Aware Canadian Aggregate Bond Index ETF,TSX
XSTB,iShares ESG Aware Canadian Short Term Bond Index ETF,TSX
XESG,iShares ESG Aware MSCI Canada Index ETF,TSX
XSEA,iShares ESG Aware MSCI EAFE Index ETF,TSX
XSEM,iShares ESG Aware MSCI Emerging Markets Index ETF,TSX
XSUS,iShares ESG Aware MSCI USA Index ETF,TSX
GBAL,iShares ESG Balanced ETF Portfolio,TSX
GCNS,iShares ESG Conservative Balanced ETF Portfolio,TSX
GEQT,iShares ESG Equity ETF Portfolio,TSX
GGRO,iShares ESG Growth ETF Portfolio,TSX
XEXP,iShares Exponential Technologies Index ETF,TSX
XFR,iShares Floating Rate Index ETF,TSX
XDNA,iShares Genomics Immunology and Healthcare Index ETF,TSX
COW,iShares Global Agriculture Index ETF,TSX
XCLN,iShares Global Clean Energy Index ETF,TSX
XDRV,iShares Global Electric and Autonomous Vehicles Index ETF,TSX
XHC,iShares Global Healthcare Index ETF (CAD-Hedged),TSX
CIF,iShares Global Infrastructure Index ETF,TSX
CYH,iShares Global Monthly Dividend Index ETF (CAD-Hedged),TSX
CGR,iShares Global Real Estate Index ETF,TSX
CWW,iShares Global Water Index ETF,TSX
CGL,iShares Gold Bullion ETF,TSX
XQB,iShares High Quality Canadian Bond Index ETF,TSX
XID,iShares India Index ETF,TSX
XEB,iShares J.P. Morgan USD Emerging Markets Bond Index ETF (CAD-Hedged),TSX
XEN,iShares Jantzi Social Index ETF,TSX
XIN,iShares MSCI EAFE Index ETF (CAD-Hedged),TSX
XEMC,iShares MSCI Emerging Markets ex China Index ETF,TSX
XEM,iShares MSCI Emerging Markets Index ETF,TSX
XEU,iShares MSCI Europe IMI Index ETF,TSX
XEH,iShares MSCI Europe IMI Index ETF (CAD-Hedged),TSX
XMV,iShares MSCI Min Vol Canada Index ETF,TSX
XMI,iShares MSCI Min Vol EAFE Index ETF,TSX
XML,iShares MSCI Min Vol EAFE Index ETF (CAD-Hedged),TSX
XMM,iShares MSCI Min Vol Emerging Markets Index ETF,TSX
XMW,iShares MSCI Min Vol Global Index ETF,TSX
XMY,iShares MSCI Min Vol Global Index ETF (CAD-Hedged),TSX
XMU,iShares MSCI Min Vol USA Index ETF,TSX
XMS,iShares MSCI Min Vol USA Index ETF (CAD-Hedged),TSX
XMTM,iShares MSCI USA Momentum Factor Index ETF,TSX
XQLT,iShares MSCI USA Quality Factor Index ETF,TSX
XVLU,iShares MSCI USA Value Factor Index ETF,TSX
XWD,iShares MSCI World Index ETF,TSX
XQQU,iShares NASDAQ 100 Index ETF,TSX
XQQ,iShares NASDAQ 100 Index ETF (CAD-Hedged),TSX
CMR,iShares Premium Money Market ETF,TSX
XCD,iShares S&P Global Consumer Discretionary Index ETF (CAD-Hedged),TSX
XGI,iShares S&P Global Industrials Index ETF(CAD-Hedged),TSX
XUSF,iShares S&P U.S. Financials Index ETF,TSX
XMC,iShares S&P U.S. Mid-Cap Index ETF,TSX
XMH,iShares S&P U.S. Mid-Cap Index ETF (CAD-Hedged),TSX
XSMC,iShares S&P U.S. Small-Cap Index ETF,TSX
XSMH,iShares S&P U.S. Small-Cap Index ETF (CAD-Hedged),TSX
XIU,iShares S&P/TSX 60 Index ETF,TSX
CDZ,iShares S&P/TSX Canadian Dividend Aristocrats Index ETF,TSX
CPD,iShares S&P/TSX Canadian Preferred Share Index ETF,TSX
XST,iShares S&P/TSX Capped Consumer Staples Index ETF,TSX
XEG,iShares S&P/TSX Capped Energy Index ETF,TSX
XFN,iShares S&P/TSX Capped Financials Index ETF,TSX
XIT,iShares S&P/TSX Capped Information Technology Index ETF,TSX
XMA,iShares S&P/TSX Capped Materials Index ETF,TSX
XRE,iShares S&P/TSX Capped REIT Index ETF,TSX
XUT,iShares S&P/TSX Capped Utilities Index ETF,TSX
XMD,iShares S&P/TSX Completion Index ETF,TSX
XEI,iShares S&P/TSX Composite High Dividend Index ETF,TSX
XETM,iShares S&P/TSX Energy Transition Materials Index ETF,TSX
XBM,iShares S&P/TSX Global Base Metals Index ETF,TSX
XGD,iShares S&P/TSX Global Gold Index ETF,TSX
XPF,iShares S&P/TSX North American Preferred Stock Index ETF (CAD-Hedged),TSX
XCS,iShares S&P/TSX SmallCap Index ETF,TSX
XCHP,iShares Semiconductor Index ETF,TSX
XSI,iShares Short Term Strategic Fixed Income ETF,TSX
SVR,iShares Silver Bullion ETF,TSX
XAD,iShares U.S. Aerospace & Defense Index ETF,TSX
XAGG,iShares U.S. Aggregate Bond Index ETF,TSX
XAGH,iShares U.S. Aggregate Bond Index ETF (CAD-Hedged),TSX
XHU,iShares U.S. High Dividend Equity Index ETF,TSX
XHD,iShares U.S. High Dividend Equity Index ETF (CAD-Hedged),TSX
XHY,iShares U.S. High Yield Bond Index ETF (CAD-Hedged),TSX
XCBU,iShares U.S. IG Corporate Bond Index ETF,TSX
XIG,iShares U.S. IG Corporate Bond Index ETF (CAD-Hedged),TSX
XSU,iShares U.S. Small Cap Index ETF (CAD-Hedged),TSX
CUD,iShares US Dividend Growers Index ETF (CAD-Hedged),TSX
IE,Ivanhoe Electric Inc.,TSX
IVN,Ivanhoe Mines Ltd.,TSX
JAG,Jaguar Mining Inc.,TSX
JWEL,Jamieson Wellness Inc.,TSX
JFS.UN,JFT Strategies Fund,TSX
JOY,Journey Energy Inc.,TSX
KBL,K-Bro Linen Inc.,TSX
KNT,K92 Mining Inc.,TSX
KRN,Karnalyte Resources Inc.,TSX
KRR,Karora Resources Inc.,TSX
KEG.UN,Keg Royalties Income Fund (The),TSX
KLS,Kelso Technologies Inc.,TSX
KEL,Kelt Exploration Ltd.,TSX
KEY,Keyera Corp.,TSX
KMP.UN,Killam Apartment Real Estate Investment Trust,TSX
KXS,Kinaxis Inc.,TSX
K,Kinross Gold Corporation,TSX
KITS,Kits Eyecare Ltd.,TSX
KEC,Kiwetinohk Energy Corp.,TSX
KSI,\"kneat.com, inc.\",TSX
GUD,Knight Therapeutics Inc.,TSX
KEI,Kolibri Global Energy Inc.,TSX
KPT,KP Tissue Inc.,TSX
LIF,Labrador Iron Ore Royalty Corporation,TSX
LAM,Laramide Resources Ltd.,TSX
LGO,Largo Inc.,TSX
LAS.A,Lassonde Industries Inc.,TSX
LB,Laurentian Bank of Canada,TSX
LNF,Leon's Furniture Limited,TSX
LGD,Liberty Gold Corp.,TSX
LBS,Life & Banc Split Corp.,TSX
LSPK,LifeSpeak Inc,TSX
LSPD,Lightspeed Commerce Inc.,TSX
LNR,Linamar Corporation,TSX
LEV,Lion Electric Company (The),TSX
LAAC,Lithium Americas (Argentina) Corp.,TSX
LAC,Lithium Americas Corp.,TSX
LIRC,Lithium Royalty Corp.,TSX
L,Loblaw Companies Limited,TSX
LN,Loncor Gold Inc.,TSX
LPEN,Loop Energy Inc.,TSX
LUC,Lucara Diamond Corp.,TSX
LUG,Lundin Gold Inc.,TSX
LUN,Lundin Mining Corporation,TSX
LYCT,Lysander-Canso Corporate Treasury ActivETF,TSX
LYFR,Lysander-Canso Floating Rate ActivETF,TSX
PR,Lysander-Slater Preferred Share ActivETF,TSX
XMF.A,M Split Corp.,TSX
MEQT,Mackenzie All-Equity Allocation ETF,TSX
MBAL,Mackenzie Balanced Allocation ETF,TSX
QBB,Mackenzie Canadian Aggregate Bond Index ETF,TSX
QCN,Mackenzie Canadian Equity Index ETF,TSX
QLB,Mackenzie Canadian Government Long Bond Index ETF,TSX
QCE,Mackenzie Canadian Large Cap Equity Index ETF,TSX
MCSB,Mackenzie Canadian Short Term Fixed Income ETF,TSX
QSB,Mackenzie Canadian Short-Term Bond Index ETF,TSX
QASH,Mackenzie Canadian Ultra Short Bond Index ETF,TSX
MCON,Mackenzie Conservative Allocation ETF,TSX
MKB,Mackenzie Core Plus Canadian Fixed Income ETF,TSX
MGB,Mackenzie Core Plus Global Fixed Income ETF,TSX
QDXB,Mackenzie Developed ex-North America Aggregate Bond Index ETF (CAD-Hedged),TSX
QRET,Mackenzie Developed Markets Real Estate Index ETF,TSX
QEBH,Mackenzie Emerging Markets Bond Index ETF (CAD-Hedged),TSX
QEE,Mackenzie Emerging Markets Equity Index ETF,TSX
QEBL,Mackenzie Emerging Markets Local Currency Bond Index ETF,TSX
MFT,Mackenzie Floating Rate Income ETF,TSX
MGAB,Mackenzie Global Fixed Income Allocation ETF,TSX
QINF,Mackenzie Global Infrastructure Index ETF,TSX
MDVD,Mackenzie Global Sustainable Dividend Index ETF,TSX
MGRW,Mackenzie Growth Allocation ETF,TSX
QDX,Mackenzie International Equity Index ETF,TSX
QDXH,Mackenzie International Equity Index ETF (CAD-Hedged),TSX
MIVG,Mackenzie Ivy Global Equity ETF,TSX
MKZ.UN,Mackenzie Master Limited Partnership,TSX
MXU,Mackenzie Maximum Diversification All World Developed ex North America Index ETF,TSX
MWD,Mackenzie Maximum Diversification All World Developed Index ETF,TSX
MKC,Mackenzie Maximum Diversification Canada Index ETF,TSX
MEU,Mackenzie Maximum Diversification Developed Europe Index ETF,TSX
MEE,Mackenzie Maximum Diversification Emerging Markets Index ETF,TSX
MUS,Mackenzie Maximum Diversification US Index ETF,TSX
QUB,Mackenzie U.S. Aggregate Bond Index ETF (CAD-Hedged),TSX
MUB,Mackenzie Unconstrained Bond ETF,TSX
QTLT,Mackenzie US Government Long Bond Index ETF,TSX
QHY,Mackenzie US High Yield Bond Index ETF (CAD-Hedged),TSX
QUIG,Mackenzie US Investment Grade Corporate Bond Index ETF (CAD-Hedged),TSX
QUU,Mackenzie US Large Cap Equity Index ETF,TSX
QAH,Mackenzie US Large Cap Equity Index ETF (CAD-Hedged),TSX
MWLV,Mackenzie World Low Volatility ETF,TSX
MPC,Madison Pacific Properties Inc.,TSX
MAG,MAG Silver Corp.,TSX
MAL,Magellan Aerospace Corporation,TSX
MG,Magna International Inc.,TSX
MEQ,Mainstreet Equity Corp.,TSX
MDI,Major Drilling Group International Inc.,TSX
MND,Mandalay Resources Corporation,TSX
MFC,Manulife Financial Corporation,TSX
MCLC,Manulife Multifactor Canadian Large Cap Index ETF,TSX
MCSM,Manulife Multifactor Canadian SMID Cap Index ETF,TSX
MINT,Manulife Multifactor Developed International Index ETF,TSX
MEME.B,Manulife Multifactor Emerging Markets Index ETF,TSX
MULC,Manulife Multifactor U.S. Large Cap Index ETF,TSX
MUMC,Manulife Multifactor U.S. Mid Cap Index ETF,TSX
MUSC,Manulife Multifactor U.S. Small Cap Index ETF,TSX
BSKT,Manulife Smart Core Bond ETF,TSX
CBND,Manulife Smart Corporate Bond ETF,TSX
CDEF,Manulife Smart Defensive Equity ETF,TSX
CDIV,Manulife Smart Dividend ETF,TSX
IDEF.B,Manulife Smart International Defensive Equity ETF,TSX
IDIV.B,Manulife Smart International Dividend ETF,TSX
TERM,Manulife Smart Short-Term Bond ETF,TSX
UDEF,Manulife Smart U.S. Defensive Equity ETF,TSX
UDIV,Manulife Smart U.S. Dividend ETF,TSX
MFI,Maple Leaf Foods Inc.,TSX
MARI,Marimaca Copper Corp.,TSX
MRE,Martinrea International Inc.,TSX
MATR,Mattr Corp.,TSX
MXG,Maxim Power Corp.,TSX
MKP,MCAN Mortgage Corporation,TSX
MCB,McCoy Global Inc.,TSX
MUX,McEwen Mining Inc.,TSX
MDA,MDA Space Ltd.,TSX
MDF,mdf commerce inc.,TSX
MDP,Medexus Pharmaceuticals Inc.,TSX
DR,Medical Facilities Corporation,TSX
MDNA,Medicenna Therapeutics Corp.,TSX
LABS,MediPharm Labs Corp.,TSX
MEG,MEG Energy Corp.,TSX
MGA,Mega Uranium Ltd.,TSX
MRD,Melcor Developments Ltd.,TSX
MR.DB.B,Melcor Real Estate Investment Trust,TSX
MNO,Meridian Mining UK Societas,TSX
MX,Methanex Corporation,TSX
MRU,Metro Inc.,TSX
MBX,Microbix Biosystems Inc.,TSX
RA.UN,Middlefield Global Real Asset Fund,TSX
MHCD,Middlefield Healthcare Dividend ETF,TSX
MINN,Middlefield Innovation Dividend ETF,TSX
MREL,Middlefield Real Estate Dividend ETF,TSX
MDIV,Middlefield Sustainable Global Dividend ETF,TSX
MINF,Middlefield Sustainable Infrastructure Dividend ETF,TSX
MUSA,Middlefield U.S. Equity Dividend ETF,TSX
MSV,Minco Silver Corporation,TSX
MSA,Mineros S.A.,TSX
MID.UN,MINT Income Fund,TSX
MI.UN,Minto Apartment Real Estate Investment Trust,TSX
MOGO,Mogo Inc.,TSX
TPX.A,Molson Coors Canada Inc.,TSX
MRC,Morguard Corporation,TSX
MRG.DB.B,Morguard North American Residential Real Estate Investment Trust,TSX
MRT.DB.A,Morguard Real Estate Investment Trust,TSX
MPVD,Mountain Province Diamonds Inc.,TSX
MTY,MTY Food Group Inc.,TSX
MTL,Mullen Group Ltd.,TSX
CBNK,Mulvihill Canadian Bank Enhanced Yield ETF,TSX
MPY,Mulvihill Premium Yield Fund,TSX
XLVE,Mulvihill U.S. Health Care Enhanced Yield ETF,TSX
NANO,Nano One Materials Corp.,TSX
GRA,NanoXplore Inc.,TSX
NA,National Bank of Canada,TSX
NPRF,NBI Active Canadian Preferred Shares ETF,TSX
NINT,NBI Active International Equity ETF,TSX
NUSA,NBI Active U.S. Equity ETF,TSX
NDIV,NBI Canadian Dividend Income ETF,TSX
NFAM,NBI Canadian Family Business ETF,TSX
NGPE,NBI Global Private Equity ETF,TSX
NREA,NBI Global Real Assets Income ETF,TSX
NHYB,NBI High Yield Bond ETF,TSX
NALT,NBI Liquid Alternatives ETF,TSX
NSCB,NBI Sustainable Canadian Bond ETF,TSX
NSCC,NBI Sustainable Canadian Corporate Bond ETF,TSX
NSCE,NBI Sustainable Canadian Equity ETF,TSX
NSSB,NBI Sustainable Canadian Short Term Bond ETF,TSX
NSGE,NBI Sustainable Global Equity ETF,TSX
NUBF,NBI Unconstrained Fixed Income ETF,TSX
NEO,Neo Performance Materials Inc.,TSX
NCU,Nevada Copper Corp.,TSX
NGD,New Gold Inc.,TSX
NUAG,New Pacific Metals Corp.,TSX
NGT,Newmont Corporation,TSX
NXE,NexGen Energy Ltd.,TSX
NEXT,NextSource Materials Inc.,TSX
NXR.UN,Nexus Industrial REIT,TSX
NFI,NFI Group Inc.,TSX
NGEX,NGEx Minerals Ltd.,TSX
NCP,Nickel Creek Platinum Corp.,TSX
TKN,Ninepoint Web3 Innovators Fund,TSX
NOA,North American Construction Group Ltd.,TSX
FFN,North American Financial 15 Split Corp.,TSX
NWC,North West Company Inc. (The),TSX
NCF,Northcliff Resources Ltd.,TSX
NDM,Northern Dynasty Minerals Ltd.,TSX
NPI,Northland Power Inc.,TSX
NRR.UN,Northview Residential REIT,TSX
NWH.DB.G,NorthWest Healthcare Properties Real Estate Investment Trust,TSX
NOVC,Nova Cannabis Inc.,TSX
NG,NovaGold Resources Inc.,TSX
NVO,Novo Resources Corp.,TSX
NUMI,Numinus Wellness Inc.,TSX
NTR,Nutrien Ltd.,TSX
NVEI,Nuvei Corporation,TSX
NVA,NuVista Energy Ltd.,TSX
SFD,NXT Energy Solutions Inc.,TSX
OBE,Obsidian Energy Ltd.,TSX
OGC,OceanaGold Corporation,TSX
OLY,Olympia Financial Group Inc.,TSX
ONC,Oncolytics Biotech Inc.,TSX
ONEX,ONEX Corporation,TSX
OTEX,Open Text Corporation,TSX
OPT,Optiva Inc.,TSX
OGD,Orbit Garant Drilling Inc.,TSX
ORE,Orezone Gold Corporation,TSX
OGI,Organigram Holdings Inc.,TSX
OLA,Orla Mining Ltd,TSX
ORV,Orvana Minerals Corp.,TSX
OR,Osisko Gold Royalties Ltd,TSX
OSK,Osisko Mining Inc.,TSX
OVV,Ovintiv Inc.,TSX
PAAS,Pan American Silver Corp.,TSX
POU,Paramount Resources Ltd.,TSX
PXT,Parex Resources Inc.,TSX
PLC,Park Lawn Corporation,TSX
PKI,Parkland Corporation,TSX
PVS.PR.F,Partners Value Split Corp.,TSX
PSI,Pason Systems Inc.,TSX
PMET,Patriot Battery Metals Inc.,TSX
PAY,Payfare Inc.,TSX
PPL,Pembina Pipeline Corporation,TSX
PPTA,Perpetua Resources Corp.,TSX
PMT,Perpetual Energy Inc.,TSX
PRU,Perseus Mining Limited,TSX
PET,Pet Valu Holdings Ltd.,TSX
TAL,PetroTal Corp.,TSX
PRQ,Petrus Resources Ltd.,TSX
PEY,Peyto Exploration & Development Corp.,TSX
PHX,PHX Energy Services Corp.,TSX
PFAE,Picton Mahoney Fortified Active Extension Alternative Fund,TSX
PFAA,Picton Mahoney Fortified Alpha Alternative Fund,TSX
PFCB,Picton Mahoney Fortified Core Bond Fund,TSX
PFIA,Picton Mahoney Fortified Income Alternative Fund,TSX
PFLS,Picton Mahoney Fortified Long Short Alternative Fund,TSX
PFMN,Picton Mahoney Fortified Market Neutral Alternative Fund,TSX
PFMS,Picton Mahoney Fortified Multi-Strategy Alternative Fund,TSX
PFSS,Picton Mahoney Fortified Special Situations Alternative Fund,TSX
PMB.UN,Picton Mahoney Tactical Income Fund,TSX
PEA,Pieridae Energy Limited,TSX
PGI.UN,PIMCO Global Income Opportunities Fund,TSX
PMNT,PIMCO Global Short Maturity Fund (Canada),TSX
IGCF,PIMCO Investment Grade Credit Fund (Canada),TSX
PLDI,PIMCO Low Duration Monthly Income Fund (Canada),TSX
PCON,PIMCO Managed Conservative Bond Pool,TSX
PCOR,PIMCO Managed Core Bond Pool,TSX
PMIF,PIMCO Monthly Income Fund (Canada),TSX
PIX.UN,PIMCO Multi-Sector Income Fund,TSX
PTI.UN,PIMCO Tactical Income Fund,TSX
PTO.UN,PIMCO Tactical Income Opportunities Fund,TSX
PNE,Pine Cliff Energy Ltd.,TSX
PNP,Pinetree Capital Ltd.,TSX
PZA,Pizza Pizza Royalty Corp.,TSX
PTM,Platinum Group Metals Ltd.,TSX
PLZ.UN,Plaza Retail REIT,TSX
PIF,Polaris Renewable Energy Inc.,TSX
PBL,Pollard Banknote Limited,TSX
PNC.A,Postmedia Network Canada Corp.,TSX
POW,Power Corporation of Canada,TSX
PWF.PF.A,Power Financial Corporation,TSX
PPR,Prairie Provident Resources Inc.,TSX
PSK,PrairieSky Royalty Ltd.,TSX
MMP.UN,Precious Metals and Mining Trust,TSX
PD,Precision Drilling Corporation,TSX
PBH,Premium Brands Holdings Corporation,TSX
MCAD,Premium Cash Management Fund,TSX
PIC.A,Premium Income Corporation,TSX
PMZ.UN,Primaris Real Estate Investment Trust,TSX
PDV,Prime Dividend Corp.,TSX
PRYM,Prime Mining Corp.,TSX
PRMW,Primo Water Corporation,TSX
PRV.DB,PRO Real Estate Investment Trust,TSX
PRB,Probe Gold Inc.,TSX
PRN,Profound Medical Corp.,TSX
PRL,Propel Holdings Inc.,TSX
PSD,Pulse Seismic Inc.,TSX
PABF,Purpose Active Balanced Fund,TSX
PACF,Purpose Active Conservative Fund,TSX
PAGF,Purpose Active Growth Fund,TSX
PBI,Purpose Best Ideas Fund,TSX
BTCC,Purpose Bitcoin ETF,TSX
BTCY,Purpose Bitcoin Yield ETF,TSX
BNC,Purpose Canadian Financial Income Fund,TSX
MNY,Purpose Cash Management Fund,TSX
PRP,Purpose Conservative Income Fund,TSX
PDF,Purpose Core Dividend Fund,TSX
CROP,Purpose Credit Opportunities Fund,TSX
PRA,Purpose Diversified Real Asset Fund,TSX
PDIV,Purpose Enhanced Dividend Fund,TSX
PAYF,Purpose Enhanced Premium Yield Fund,TSX
ETHH,Purpose Ether ETF,TSX
ETHY,Purpose Ether Yield ETF,TSX
IGB,Purpose Global Bond Class,TSX
BND,Purpose Global Bond Fund,TSX
FLX,Purpose Global Flexible Credit Fund,TSX
PINV,Purpose Global Innovators Fund,TSX
KILO,Purpose Gold Bullion Fund,TSX
PSA,Purpose High Interest Savings Fund,TSX
PID,Purpose International Dividend Fund,TSX
PHW,Purpose International Tactical Hedged Equity Fund,TSX
PIN,Purpose Monthly Income Fund,TSX
PINC,Purpose Multi-Asset Income Fund,TSX
PMM,Purpose Multi-Strategy Market Neutral Fund,TSX
PYF,Purpose Premium Yield Fund,TSX
PHR,Purpose Real Estate Income Fund,TSX
SBT,Purpose Silver Bullion Fund,TSX
SYLD,Purpose Strategic Yield Fund,TSX
PHE,Purpose Tactical Hedged Equity Fund,TSX
PBD,Purpose Total Return Bond Fund,TSX
PSU.U,Purpose US Cash Fund,TSX
MNU.U,Purpose USD Cash Management Fund,TSX
PYR,PyroGenesis Canada Inc.,TSX
QTRH,Quarterhill Inc.,TSX
QBR.A,Quebecor Inc.,TSX
QRC,Queen's Road Capital Investment Ltd.,TSX
QEC,Questerre Energy Corporation,TSX
QIPT,Quipt Home Medical Corp.,TSX
RAV.UN,Ravensource Fund,TSX
RBA,\"RB Global, Inc.\",TSX
RLB,RBC 1-5 Year Laddered Canadian Bond ETF,TSX
RBO,RBC 1-5 Year Laddered Canadian Corporate Bond ETF,TSX
RBNK,RBC Canadian Bank Yield Index ETF,TSX
RCDC,RBC Canadian Dividend Covered Call ETF,TSX
RPF,RBC Canadian Preferred Share ETF,TSX
RPSB,RBC PH&N Short Term Canadian Bond ETF,TSX
RCD,RBC Quant Canadian Dividend Leaders ETF,TSX
RIDH,RBC Quant EAFE Dividend Leaders (CAD Hedged) ETF,TSX
RID,RBC Quant EAFE Dividend Leaders ETF,TSX
RXD,RBC Quant Emerging Markets Dividend Leaders ETF,TSX
RPDH,RBC Quant European Dividend Leaders (CAD Hedged) ETF,TSX
RPD,RBC Quant European Dividend Leaders ETF,TSX
RUDH,RBC Quant U.S. Dividend Leaders (CAD Hedged) ETF,TSX
RUD,RBC Quant U.S. Dividend Leaders ETF,TSX
RUSB,RBC Short Term U.S. Corporate Bond ETF,TSX
RQL,RBC Target 2024 Canadian Corporate Bond Index ETF,TSX
RGQL,RBC Target 2024 Canadian Government Bond ETF,TSX
RQN,RBC Target 2025 Canadian Corporate Bond Index ETF,TSX
RGQN,RBC Target 2025 Canadian Government Bond ETF,TSX
RUQN,RBC Target 2025 U.S. Corporate Bond ETF,TSX
RQO,RBC Target 2026 Canadian Corporate Bond Index ETF,TSX
RGQO,RBC Target 2026 Canadian Government Bond ETF,TSX
RUQO,RBC Target 2026 U.S. Corporate Bond ETF,TSX
RQP,RBC Target 2027 Canadian Corporate Bond Index ETF,TSX
RGQP,RBC Target 2027 Canadian Government Bond ETF,TSX
RUQP,RBC Target 2027 U.S. Corporate Bond ETF,TSX
RQQ,RBC Target 2028 Canadian Corporate Bond Index ETF,TSX
RGQQ,RBC Target 2028 Canadian Government Bond ETF,TSX
RUQQ,RBC Target 2028 U.S. Corporate Bond ETF,TSX
RQR,RBC Target 2029 Canadian Corporate Bond Index ETF,TSX
RGQR,RBC Target 2029 Canadian Government Bond ETF,TSX
RUQR,RBC Target 2029 U.S. Corporate Bond ETF,TSX
RQS,RBC Target 2030 Canadian Corporate Bond Index ETF,TSX
RGQS,RBC Target 2030 Canadian Government Bond ETF,TSX
RUQS,RBC Target 2030 U.S. Corporate Bond ETF,TSX
RUBH,RBC U.S. Banks Yield (CAD Hedged) Index ETF,TSX
RUBY,RBC U.S. Banks Yield Index ETF,TSX
RDBH,RBC U.S. Discount Bond (CAD Hedged) ETF,TSX
RUDB,RBC U.S. Discount Bond ETF,TSX
RUDC,RBC U.S. Dividend Covered Call ETF,TSX
RS,Real Estate Split Corp.,TSX
REAL,Real Matters Inc.,TSX
QSR,Restaurant Brands International Inc.,TSX
QSP.UN,Restaurant Brands International Limited Partnership,TSX
RVX,Resverlogix Corp.,TSX
RCG,RF Capital Group Inc.,TSX
RPI.UN,Richards Packaging Income Fund,TSX
RCH,Richelieu Hardware Ltd.,TSX
REI.UN,RioCan Real Estate Investment Trust,TSX
RCI.A,Rogers Communications Inc.,TSX
RSI,Rogers Sugar Inc.,TSX
ROOT,Roots Corporation,TSX
RY,Royal Bank of Canada,TSX
MNT,Royal Canadian Mint - Canadian Gold Reserves,TSX
MNS,Royal Canadian Mint - Canadian Silver Reserves,TSX
RTG,RTG Mining Inc.,TSX
RBY,Rubellite Energy Inc.,TSX
RUP,Rupert Resources Ltd.,TSX
RUS,Russel Metals Inc.,TSX
RIFI,Russell Investments Fixed Income Pool,TSX
RIIN,Russell Investments Global Infrastructure Pool,TSX
RIRA,Russell Investments Real Assets,TSX
SBN,S Split Corp.,TSX
SGLD,Sabre Gold Mines Corp.,TSX
MIC.PR.A,Sagen MI Canada Inc.,TSX
SFC,Sagicor Financial Company Ltd.,TSX
SSL,Sandstorm Gold Ltd.,TSX
STC,Sangoma Technologies Corporation,TSX
SAP,Saputo Inc.,TSX
MSCL,Satellos Bioscience Inc.,TSX
SOIL,Saturn Oil & Gas Inc.,TSX
SIS,Savaria Corporation,TSX
SCY,Scandium International Mining Corp.,TSX
SEA,Seabridge Gold Inc.,TSX
SES,Secure Energy Services Inc.,TSX
PME,Sentry Select Primary Metals Corp.,TSX
SEC,Senvest Capital Inc.,TSX
SBI,Serabi Gold plc,TSX
SVA,Sernova Corp.,TSX
S,Sherritt International Corporation,TSX
SHOP,Shopify Inc.,TSX
SIA,Sienna Senior Living Inc.,TSX
SMT,Sierra Metals Inc.,TSX
SGNL,Signal Gold Inc.,TSX
SBR,Silver Bear Resources plc,TSX
SVB,\"Silver Bull Resources, Inc.\",TSX
ELEF,Silver Elephant Mining Corp.,TSX
SVM,Silvercorp Metals Inc.,TSX
SIL,SilverCrest Metals Inc.,TSX
SRV.UN,SIR Royalty Income Fund,TSX
SKE,Skeena Resources Limited,TSX
SGR.U,Slate Grocery REIT,TSX
SOT.DB,Slate Office REIT,TSX
ZZZ,Sleep Country Canada Holdings Inc.,TSX
SRU.UN,SmartCentres Real Estate Investment Trust,TSX
ATRL,SNC-Lavalin Group Inc.,TSX
SFTC,Softchoice Corporation,TSX
SLS,Solaris Resources Inc.,TSX
SOLG,SolGold plc,TSX
SLR,Solitario Resources Corp.,TSX
SFI,Solution Financial Inc.,TSX
SHLE,Source Energy Services Ltd.,TSX
SDE,Spartan Delta Corp.,TSX
EDT,Spectral Medical Inc.,TSX
TOY,Spin Master Corp.,TSX
SII,Sprott Inc.,TSX
CEF,Sprott Physical Gold and Silver Trust,TSX
PHYS,Sprott Physical Gold Trust,TSX
SPPP,Sprott Physical Platinum and Palladium Trust,TSX
PSLV,Sprott Physical Silver Trust,TSX
U.U,Sprott Physical Uranium Trust,TSX
SSRM,SSR Mining Inc.,TSX
SAU,St. Augustine Gold and Copper Limited,TSX
STCK,Stack Capital Group Inc.,TSX
SZLS,StageZero Life Sciences Ltd.,TSX
STN,Stantec Inc.,TSX
DIAM,Star Diamond Corporation,TSX
SAM,Starcore International Mines Ltd.,TSX
STLC,Stelco Holdings Inc.,TSX
SJ,Stella-Jones Inc.,TSX
STEP,STEP Energy Services Ltd.,TSX
STGO,Steppe Gold Ltd.,TSX
RAY.A,Stingray Group Inc.,TSX
STLR,STLLR Gold Inc.,TSX
SVI,StorageVault Canada Inc.,TSX
SCR,Strathcona Resources Ltd.,TSX
SMC,Sulliden Mining Capital Inc.,TSX
SLF,Sun Life Financial Inc.,TSX
SU,Suncor Energy Inc.,TSX
SOY,\"SunOpta, Inc.\",TSX
SPB,Superior Plus Corp.,TSX
SXP,Supremex Inc.,TSX
SGY,Surge Energy Inc.,TSX
SIH.UN,Sustainable Innovation & Health Dividend Fund,TSX
PWI,Sustainable Power & Infrastructure Split Corp.,TSX
SWP,Swiss Water Decaffeinated Coffee Inc.,TSX
SYZ,Sylogist Ltd.,TSX
SSF.UN,Symphony Floating Rate Senior Loan Fund,TSX
SXI,Synex Renewable Energy Corporation,TSX
TBL,Taiga Building Products Ltd.,TSX
TAIG,Taiga Motors Corporation,TSX
TSK,Talisker Resources Ltd.,TSX
TLO,Talon Metals Corp.,TSX
TVE,Tamarack Valley Energy Ltd.,TSX
GRID,Tantalus Systems Holding Inc.,TSX
TKO,Taseko Mines Limited,TSX
TRP,TC Energy Corporation,TSX
TGED,TD Active Global Enhanced Dividend ETF,TSX
TGGR,TD Active Global Equity Growth ETF,TSX
TGFI,TD Active Global Income ETF,TSX
TINF,TD Active Global Infrastructure Equity ETF,TSX
TGRE,TD Active Global Real Estate Equity ETF,TSX
TPRF,TD Active Preferred Share ETF,TSX
TUEX,TD Active U.S. Enhanced Dividend CAD Hedged ETF,TSX
TUED,TD Active U.S. Enhanced Dividend ETF,TSX
TUHY,TD Active U.S. High Yield Bond ETF,TSX
TBAL,TD Balanced ETF Portfolio,TSX
TDB,TD Canadian Aggregate Bond Index ETF,TSX
TBNK,TD Canadian Bank Dividend Index ETF,TSX
TTP,TD Canadian Equity Index ETF,TSX
TCLB,TD Canadian Long Term Federal Bond ETF,TSX
TCSH,TD Cash Management ETF,TSX
TCON,TD Conservative ETF Portfolio,TSX
TCBN,TD Global Carbon Credit Index ETF,TSX
TDOC,TD Global Healthcare Leaders Index ETF,TSX
TECI,TD Global Technology Innovators Index ETF,TSX
TECX,TD Global Technology Leaders CAD Hedged Index ETF,TSX
TEC,TD Global Technology Leaders Index ETF,TSX
TGRO,TD Growth ETF Portfolio,TSX
TPAY,TD Income Builder ETF,TSX
THE,TD International Equity CAD Hedged Index ETF,TSX
TPE,TD International Equity Index ETF,TSX
TMCC,TD Morningstar ESG Canada Corporate Bond Index ETF,TSX
TMEC,TD Morningstar ESG Canada Equity Index ETF,TSX
TMEI,TD Morningstar ESG International Equity Index ETF,TSX
TMUC,TD Morningstar ESG U.S. Corporate Bond Index ETF,TSX
TMEU,TD Morningstar ESG U.S. Equity Index ETF,TSX
TQCD,TD Q Canadian Dividend ETF,TSX
TCLV,TD Q Canadian Low Volatility ETF,TSX
TQGD,TD Q Global Dividend ETF,TSX
TQGM,TD Q Global Multifactor ETF,TSX
TILV,TD Q International Low Volatility ETF,TSX
TULV,TD Q U.S. Low Volatility ETF,TSX
TQSM,TD Q U.S. Small-Mid-Cap Equity ETF,TSX
TCSB,TD Select Short Term Corporate Bond Ladder ETF,TSX
TUSB,TD Select U.S. Short Term Corporate Bond Ladder ETF,TSX
TBCE,TD Target 2025 Investment Grade Bond ETF,TSX
TBUE.U,TD Target 2025 U.S. Investment Grade Bond ETF,TSX
TBCF,TD Target 2026 Investment Grade Bond ETF,TSX
TBUF.U,TD Target 2026 U.S. Investment Grade Bond ETF,TSX
TBCG,TD Target 2027 Investment Grade Bond ETF,TSX
TBUG.U,TD Target 2027 U.S. Investment Grade Bond ETF,TSX
THU,TD U.S. Equity CAD Hedged Index ETF,TSX
TPU,TD U.S. Equity Index ETF,TSX
TULB,TD U.S. Long Term Treasury Bond ETF,TSX
XTD,TDb Split Corp.,TSX
TECK.A,Teck Resources Limited,TSX
TCS,TECSYS Inc.,TSX
TSAT,Telesat Corporation,TSX
T,TELUS Corporation,TSX
TIXT,TELUS International (Cda) Inc.,TSX
TNZ,Tenaz Energy Corp.,TSX
TGO,TeraGo Inc.,TSX
TSND,TerrAscend Corp.,TSX
TVK,TerraVest Industries Inc.,TSX
TFII,TFI International Inc.,TSX
QETH.U,The Ether Fund,TSX
TH,Theratechnologies Inc.,TSX
THNC,Thinkific Labs Inc.,TSX
TRI,Thomson Reuters Corporation,TSX
TWM,Tidewater Midstream and Infrastructure Ltd.,TSX
LCFS,Tidewater Renewables Ltd.,TSX
TLRY,\"Tilray Brands, Inc.\",TSX
TF,Timbercreek Financial Corp.,TSX
TMD,Titan Medical Inc.,TSX
TI,Titan Mining Corporation,TSX
TTNM,Titanium Transportation Group Inc.,TSX
X,TMX Group Limited,TSX
TXT.PR.A,Top 10 Split Trust,TSX
TPZ,Topaz Energy Corp.,TSX
TXG,Torex Gold Resources Inc.,TSX
TIH,Toromont Industries Ltd.,TSX
TD,Toronto-Dominion Bank (The),TSX
TOT,Total Energy Services Inc.,TSX
TXP,Touchstone Exploration Inc.,TSX
TOU,Tourmaline Oil Corp.,TSX
TGAF,Tralucent Global Alt (Long/Short) Equity Fund,TSX
TA,TransAlta Corporation,TSX
TRZ,Transat A.T. Inc.,TSX
TCL.A,Transcontinental Inc.,TSX
TML,Treasury Metals Inc.,TSX
TSL,Tree Island Steel Ltd.,TSX
TCW,Trican Well Service Ltd.,TSX
TMQ,Trilogy Metals Inc.,TSX
TFPM,Triple Flag Precious Metals Corp.,TSX
TSU,Trisura Group Ltd.,TSX
TLG,Troilus Gold Corp.,TSX
TNT.UN,True North Commercial Real Estate Investment Trust,TSX
TRX,TRX Gold Corporation,TSX
TC,Tucows Inc.,TSX
TVA.B,TVA Group Inc.,TSX
TWC,TWC Enterprises Limited,TSX
UNI,Unisync Corp.,TSX
UNC,United Corporations Limited,TSX
URE,Ur-Energy Inc.,TSX
URC,Uranium Royalty Corp.,TSX
URB,Urbana Corporation,TSX
FTU,US Financial 15 Split Corp.,TSX
HISU.U,US High Interest Savings Account Fund,TSX
MUSD.U,US Premium Cash Management Fund,TSX
VPH,Valeo Pharma Inc.,TSX
VLE,Valeura Energy Inc.,TSX
VEQT,Vanguard All-Equity ETF Portfolio,TSX
VBAL,Vanguard Balanced ETF Portfolio,TSX
VAB,Vanguard Canadian Aggregate Bond Index ETF,TSX
VCB,Vanguard Canadian Corporate Bond Index ETF,TSX
VGV,Vanguard Canadian Government Bond Index ETF,TSX
VLB,Vanguard Canadian Long-Term Bond Index ETF,TSX
VSB,Vanguard Canadian Short-Term Bond Index ETF,TSX
VSC,Vanguard Canadian Short-Term Corporate Bond Index ETF,TSX
VCNS,Vanguard Conservative ETF Portfolio,TSX
VCIP,Vanguard Conservative Income ETF Portfolio,TSX
VCN,Vanguard FTSE Canada All Cap Index ETF,TSX
VCE,Vanguard FTSE Canada Index ETF,TSX
VRE,Vanguard FTSE Canadian Capped REIT Index ETF,TSX
VDY,Vanguard FTSE Canadian High Dividend Yield Index ETF,TSX
VIU,Vanguard FTSE Developed All Cap ex North America Index ETF,TSX
VI,Vanguard FTSE Developed All Cap ex North America Index ETF (CAD-Hedged),TSX
VDU,Vanguard FTSE Developed All Cap Ex U.S. Index ETF,TSX
VEF,Vanguard FTSE Developed All Cap Ex U.S. Index ETF (CAD-hedged),TSX
VA,Vanguard FTSE Developed Asia Pacific All Cap Index ETF,TSX
VE,Vanguard FTSE Developed Europe All Cap Index ETF,TSX
VIDY,Vanguard FTSE Developed ex North America High Dividend Yield Index ETF,TSX
VEE,Vanguard FTSE Emerging Markets All Cap Index ETF,TSX
VXC,Vanguard FTSE Global All Cap ex Canada Index ETF,TSX
VVO,Vanguard Global Minimum Volatility ETF,TSX
VMO,Vanguard Global Momentum Factor ETF,TSX
VVL,Vanguard Global Value Factor ETF,TSX
VGRO,Vanguard Growth ETF Portfolio,TSX
VRIF,Vanguard Retirement Income ETF Portfolio,TSX
VFV,Vanguard S&P 500 Index ETF,TSX
VSP,Vanguard S&P 500 Index ETF (CAD-hedged),TSX
VGG,Vanguard U.S. Dividend Appreciation Index ETF,TSX
VGH,Vanguard U.S. Dividend Appreciation Index ETF (CAD-hedged),TSX
VUN,Vanguard U.S. Total Market Index ETF,TSX
VUS,Vanguard U.S. Total Market Index ETF (CAD-hedged),TSX
VCM,Vecima Networks Inc.,TSX
VLN,Velan Inc.,TSX
NPK,Verde Agritech Ltd.,TSX
VRN,Veren Inc.,TSX
VET,Vermilion Energy Inc.,TSX
VBNK,VersaBank,TSX
FORA,VerticalScope Holdings Inc.,TSX
VGCX,Victoria Gold Corp.,TSX
VQS,VIQ Solutions Inc.,TSX
VGZ,Vista Gold Corp.,TSX
VHI,Vitalhub Corp.,TSX
VOXR,Vox Royalty Corp.,TSX
WJX,Wajax Corporation,TSX
WFC,Wall Financial Corporation,TSX
WM,Wallbridge Mining Company Limited,TSX
WCN,\"Waste Connections, Inc.\",TSX
WSRD,Wealthsimple Developed Markets ex North America Socially Responsible Index ETF,TSX
WSRI,Wealthsimple North America Socially Responsible Index ETF,TSX
WELL,WELL Health Technologies Corp.,TSX
WDO,Wesdome Gold Mines Ltd.,TSX
WFG,West Fraser Timber Co. Ltd.,TSX
WRN,Western Copper and Gold Corporation,TSX
WRG,Western Energy Services Corp.,TSX
WEF,Western Forest Products Inc.,TSX
WRX,Western Resources Corp.,TSX
WPRT,Westport Fuel Systems Inc.,TSX
WTE,Westshore Terminals Investment Corporation,TSX
WPM,Wheaton Precious Metals Corp.,TSX
WCP,Whitecap Resources Inc.,TSX
WILD,WildBrain Ltd.,TSX
WLLW,Willow Biosciences Inc.,TSX
WCM.A,Wilmington Capital Management Inc.,TSX
WPK,Winpak Ltd.,TSX
WNDR,WonderFi Technologies Inc.,TSX
WFS,World Financial Split Corp.,TSX
WSP,WSP Global Inc.,TSX
XAM,Xanadu Mines Ltd.,TSX
XTG,Xtra-Gold Resources Corp.,TSX
XTRA,Xtract One Technologies Inc.,TSX
YGR,Yangarra Resources Ltd.,TSX
Y,Yellow Pages Limited,TSX
YRB,Yorbeau Resources Inc.,TSX
AACG,\"ATA Creativity Global - American Depositary Shares, each representing two common shares\",NASDAQ Global MarketSM
AACI,Armada Acquisition Corp. I - Common Stock,NASDAQ Global MarketSM
AACIU,Armada Acquisition Corp. I - Unit,NASDAQ Global MarketSM
AACIW,Armada Acquisition Corp. I - Warrant,NASDAQ Global MarketSM
AADI,\"Aadi Bioscience, Inc. - Common Stock\",NASDAQ Capital Market
AADR,AdvisorShares Dorsey Wright ADR ETF,NASDAQ Global MarketSM
AAGR,African Agriculture Holdings Inc. - Common Stock,NASDAQ Global MarketSM
AAGRW,African Agriculture Holdings Inc. - Warrant,NASDAQ Global MarketSM
AAL,\"American Airlines Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AAME,Atlantic American Corporation - Common Stock,NASDAQ Global MarketSM
AAOI,\"Applied Optoelectronics, Inc. - Common Stock\",NASDAQ Global MarketSM
AAON,\"AAON, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AAPB,GraniteShares 2x Long AAPL Daily ETF,NASDAQ Global MarketSM
AAPD,Direxion Daily AAPL Bear 1X Shares,NASDAQ Global MarketSM
AAPL,Apple Inc. - Common Stock,NASDAQ Global Select MarketSM
AAPU,Direxion Daily AAPL Bull 2X Shares,NASDAQ Global MarketSM
AAXJ,iShares MSCI All Country Asia ex Japan ETF,NASDAQ Global MarketSM
ABAT,American Battery Technology Company - Common Stock,NASDAQ Capital Market
ABCB,Ameris Bancorp - Common Stock,NASDAQ Global Select MarketSM
ABCL,AbCellera Biologics Inc. - Common Shares,NASDAQ Global Select MarketSM
ABCS,Alpha Blue Capital US Small-Mid Cap Dynamic ETF,NASDAQ Global MarketSM
ABEO,Abeona Therapeutics Inc. - Common Stock,NASDAQ Capital Market
ABIO,\"ARCA biopharma, Inc. - Common Stock\",NASDAQ Capital Market
ABL,\"Abacus Life, Inc. - Class A Common Stock\",NASDAQ Capital Market
ABLLL,\"Abacus Life, Inc. - 9.875% Fixed Rate Senior Notes due 2028\",NASDAQ Global MarketSM
ABLLW,\"Abacus Life, Inc. - Warrant\",NASDAQ Capital Market
ABLV,Able View Global Inc. - Class B Ordinary Shares,NASDAQ Capital Market
ABLVW,Able View Global Inc. - Warrant,NASDAQ Capital Market
ABNB,\"Airbnb, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
ABOS,\"Acumen Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ABSI,Absci Corporation - Common Stock,NASDAQ Global Select MarketSM
ABTS,Abits Group Inc - Ordinary Shares,NASDAQ Capital Market
ABUS,Arbutus Biopharma Corporation - Common Stock,NASDAQ Global Select MarketSM
ABVC,\"ABVC BioPharma, Inc. - Common Stock\",NASDAQ Capital Market
ABVX,Abivax SA - American Depositary Shares,NASDAQ Global MarketSM
ACAB,Atlantic Coastal Acquisition Corp. II - Class A Common Stock,NASDAQ Global MarketSM
ACABU,Atlantic Coastal Acquisition Corp. II - Unit,NASDAQ Global MarketSM
ACABW,Atlantic Coastal Acquisition Corp. II - Warrant,NASDAQ Global MarketSM
ACAC,Acri Capital Acquisition Corporation - Class A Common Stock,NASDAQ Capital Market
ACACU,Acri Capital Acquisition Corporation - Unit,NASDAQ Capital Market
ACACW,Acri Capital Acquisition Corporation - Warrant,NASDAQ Capital Market
ACAD,ACADIA Pharmaceuticals Inc. - Common Stock,NASDAQ Global Select MarketSM
ACB,Aurora Cannabis Inc. - Common Shares,NASDAQ Capital Market
ACBA,Ace Global Business Acquisition Limited - Ordinary Shares,NASDAQ Capital Market
ACBAU,Ace Global Business Acquisition Limited - Unit,NASDAQ Capital Market
ACBAW,Ace Global Business Acquisition Limited - Warrant,NASDAQ Capital Market
ACCD,\"Accolade, Inc. - common stock\",NASDAQ Global Select MarketSM
ACDC,ProFrac Holding Corp. - Class A Common Stock,NASDAQ Global Select MarketSM
ACET,\"Adicet Bio, Inc. - Common Stock\",NASDAQ Global MarketSM
ACGL,Arch Capital Group Ltd. - Common Stock,NASDAQ Global Select MarketSM
ACGLN,\"Arch Capital Group Ltd. - Depositary Shares, each Representing a 1/1,000th Interest in a 4.550% Non-Cumulative Preferred Share, Series G\",NASDAQ Global Select MarketSM
ACGLO,\"Arch Capital Group Ltd. - Depositary Shares Each Representing 1/1,000th Interest in a Share of5.45% Non-Cumulative Preferred Shares, Series F\",NASDAQ Global Select MarketSM
ACHC,\"Acadia Healthcare Company, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ACHL,Achilles Therapeutics plc - American Depositary Shares,NASDAQ Global Select MarketSM
ACHV,\"Achieve Life Sciences, Inc.  - Common Shares\",NASDAQ Capital Market
ACIC,American Coastal Insurance Corporation - Common Stock,NASDAQ Capital Market
ACIU,AC Immune SA - Common Stock,NASDAQ Global MarketSM
ACIW,\"ACI Worldwide, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ACLS,\"Axcelis Technologies, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ACLX,\"Arcellx, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ACMR,\"ACM Research, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
ACNB,ACNB Corporation - Common Stock,NASDAQ Capital Market
ACNT,Ascent Industries Co. - Common Stock,NASDAQ Global MarketSM
ACON,\"Aclarion, Inc. - Common Stock\",NASDAQ Capital Market
ACONW,\"Aclarion, Inc. - Warrant\",NASDAQ Capital Market
ACRS,\"Aclaris Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ACRV,\"Acrivon Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
ACST,\"Acasti Pharma, Inc.  - Class A Common Stock\",NASDAQ Capital Market
ACT,\"Enact Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ACTG,Acacia Research Corporation - Common Stock,NASDAQ Global Select MarketSM
ACVA,ACV Auctions Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
ACWI,iShares MSCI ACWI ETF,NASDAQ Global MarketSM
ACWX,iShares MSCI ACWI ex U.S. ETF,NASDAQ Global MarketSM
ACXP,\"Acurx Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
ADAG,\"Adagene Inc. - ADS, each representing 1.25 ordinary shares\",NASDAQ Global MarketSM
ADAP,Adaptimmune Therapeutics plc - American Depositary Shares,NASDAQ Global Select MarketSM
ADBE,Adobe Inc. - Common Stock,NASDAQ Global Select MarketSM
ADD,\"Color Star Technology Co., Ltd. - Class A Ordinary Shares\",NASDAQ Capital Market
ADEA,Adeia Inc.  - Common Stock,NASDAQ Global Select MarketSM
ADI,\"Analog Devices, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ADIL,\"Adial Pharmaceuticals, Inc - Common Stock\",NASDAQ Capital Market
ADMA,ADMA Biologics Inc - Common Stock,NASDAQ Global MarketSM
ADN,\"Advent Technologies Holdings, Inc. - Class A Common Stock\",NASDAQ Capital Market
ADNWW,\"Advent Technologies Holdings, Inc. - Warrant\",NASDAQ Capital Market
ADP,\"Automatic Data Processing, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ADPT,Adaptive Biotechnologies Corporation - Common Stock,NASDAQ Global Select MarketSM
ADSE,ADS-TEC ENERGY PLC - Ordinary Shares,NASDAQ Capital Market
ADSEW,ADS-TEC ENERGY PLC - Warrant,NASDAQ Capital Market
ADSK,\"Autodesk, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ADTH,\"AdTheorent Holding Company, Inc. - Common Stock\",NASDAQ Capital Market
ADTHW,\"AdTheorent Holding Company, Inc. - Warrants\",NASDAQ Capital Market
ADTN,\"ADTRAN Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ADTX,\"Aditxt, Inc. - Common Stock\",NASDAQ Capital Market
ADUS,Addus HomeCare Corporation - Common Stock,NASDAQ Global Select MarketSM
ADV,Advantage Solutions Inc.  - Class A Common Stock,NASDAQ Global Select MarketSM
ADVM,\"Adverum Biotechnologies, Inc. - Common Stock\",NASDAQ Capital Market
ADVWW,Advantage Solutions Inc.  - Warrant,NASDAQ Global Select MarketSM
ADXN,Addex Therapeutics Ltd - American Depositary Shares,NASDAQ Capital Market
AEAE,AltEnergy Acquisition Corp. - Class A Common Stock,NASDAQ Global MarketSM
AEAEU,AltEnergy Acquisition Corp. - Unit,NASDAQ Global MarketSM
AEAEW,AltEnergy Acquisition Corp. - Warrant,NASDAQ Global MarketSM
AEHL,Antelope Enterprise Holdings Limited - Class A Ordinary Shares,NASDAQ Capital Market
AEHR,Aehr Test Systems - Common Stock,NASDAQ Capital Market
AEI,Alset Inc. - Common Stock,NASDAQ Capital Market
AEIS,\"Advanced Energy Industries, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AEMD,\"Aethlon Medical, Inc. - Common Stock\",NASDAQ Capital Market
AENT,Alliance Entertainment Holding Corporation - common stock,NASDAQ Capital Market
AENTW,Alliance Entertainment Holding Corporation - Warrants,NASDAQ Capital Market
AEP,\"American Electric Power Company, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AERT,\"Aeries Technology, Inc. - Class A Ordinary Share\",NASDAQ Capital Market
AERTW,\"Aeries Technology, Inc. - Warrant\",NASDAQ Capital Market
AEYE,\"AudioEye, Inc. - Common Stock\",NASDAQ Capital Market
AEZS,Aeterna Zentaris Inc. - Common Stock,NASDAQ Capital Market
AFAR,Aura FAT Projects Acquisition Corp - Class A Ordinary Shares,NASDAQ Global MarketSM
AFARU,Aura FAT Projects Acquisition Corp - Unit,NASDAQ Global MarketSM
AFARW,Aura FAT Projects Acquisition Corp - Warrant,NASDAQ Global MarketSM
AFBI,\"Affinity Bancshares, Inc. - Common Stock\",NASDAQ Capital Market
AFCG,\"AFC Gamma, Inc. - Common Stock\",NASDAQ Global MarketSM
AFJK,\"Aimei Health Technology Co., Ltd - Ordinary Share\",NASDAQ Global MarketSM
AFJKR,\"Aimei Health Technology Co., Ltd - Right\",NASDAQ Global MarketSM
AFJKU,\"Aimei Health Technology Co., Ltd - Unit\",NASDAQ Global MarketSM
AFMD,Affimed N.V. - Common Stock,NASDAQ Global MarketSM
AFRI,Forafric Global PLC - Ordinary Shares,NASDAQ Capital Market
AFRIW,Forafric Global PLC - Warrants,NASDAQ Capital Market
AFRM,\"Affirm Holdings, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
AFYA,Afya Limited - Class A Common Shares,NASDAQ Global Select MarketSM
AGAE,Allied Gaming & Entertainment Inc. - Common Stock,NASDAQ Capital Market
AGBA,AGBA Group Holding Limited - Ordinary Share,NASDAQ Capital Market
AGBAW,AGBA Group Holding Limited - Warrant,NASDAQ Capital Market
AGEN,Agenus Inc. - Common Stock,NASDAQ Capital Market
AGFY,Agrify Corporation - Common Stock,NASDAQ Capital Market
AGIO,\"Agios Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AGMH,AGM Group Holdings Inc. - Class A Ordinary Shares,NASDAQ Capital Market
AGMI,Themes Silver Miners ETF,NASDAQ Global MarketSM
AGNC,AGNC Investment Corp. - Common Stock,NASDAQ Global Select MarketSM
AGNCL,\"AGNC Investment Corp. - Depositary Shares Each Representing a 1/1,000th Interest in a Share of 7.75% Series G Fixed-Rate Reset Cumulative Redeemable Preferred Stock\",NASDAQ Global Select MarketSM
AGNCM,AGNC Investment Corp. - Depositary Shares rep 6.875% Series D Fixed-to-Floating Cumulative Redeemable Preferred Stock,NASDAQ Global Select MarketSM
AGNCN,\"AGNC Investment Corp. - Depositary Shares Each Representing a 1/1,000th Interest in a Share of 7.00% Series C Fixed-To-Floating Rate Cumulative Redeemable Preferred Stock\",NASDAQ Global Select MarketSM
AGNCO,\"AGNC Investment Corp. - Depositary Shares, each representing a 1/1,000th interest in a share of Series E Fixed-to-Floating Cumulative Redeemable Preferred Stock\",NASDAQ Global Select MarketSM
AGNCP,\"AGNC Investment Corp. - Depositary Shares Each Representing a 1/1,000th Interest in a Share of 6.125% Series F Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock\",NASDAQ Global Select MarketSM
AGNG,Global X Aging Population ETF,NASDAQ Global MarketSM
AGRI,AgriFORCE  Growing Systems Ltd. - Common Shares,NASDAQ Capital Market
AGRIW,AgriFORCE  Growing Systems Ltd. - Warrant,NASDAQ Capital Market
AGYS,\"Agilysys, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AGZD,WisdomTree Interest Rate Hedged U.S. Aggregate Bond Fund,NASDAQ Global MarketSM
AHCO,AdaptHealth Corp.  - Common Stock,NASDAQ Capital Market
AHG,Akso Health Group - American Depositary Shares,NASDAQ Capital Market
AHI,Advanced Health Intelligence Ltd. - American Depositary Shares,NASDAQ Capital Market
AIA,iShares Asia 50 ETF,NASDAQ Global MarketSM
AIHS,Senmiao Technology Limited - Common Stock,NASDAQ Capital Market
AILE,\"iLearningEngines, Inc. - Common Stock\",NASDAQ Capital Market
AILEW,\"iLearningEngines, Inc. - Warrant\",NASDAQ Capital Market
AIMAU,Aimfinity Investment Corp. I - Unit,NASDAQ Global MarketSM
AIMAW,Aimfinity Investment Corp. I - Warrant,NASDAQ Global MarketSM
AIMBU,Aimfinity Investment Corp. I - Subunit,NASDAQ Global MarketSM
AIMD,\"Ainos, Inc. - Common Stock\",NASDAQ Capital Market
AIMDW,\"Ainos, Inc. - warrants\",NASDAQ Capital Market
AIP,\"Arteris, Inc.  - Common Stock\",NASDAQ Global MarketSM
AIQ,Global X Artificial Intelligence & Technology ETF,NASDAQ Global MarketSM
AIRE,reAlpha Tech Corp. - Common Stock,NASDAQ Capital Market
AIRG,\"Airgain, Inc. - Common Stock\",NASDAQ Capital Market
AIRJ,Montana Technologies Corp - Class A Common Stock,NASDAQ Capital Market
AIRJW,Montana Technologies Corp - Warrant,NASDAQ Capital Market
AIRL,Themes Airlines ETF,NASDAQ Global MarketSM
AIRR,First Trust RBA American Industrial Renaissance ETF,NASDAQ Global MarketSM
AIRS,\"AirSculpt Technologies, Inc. - Common Stock\",NASDAQ Global MarketSM
AIRT,\"Air T, Inc. - Common Stock\",NASDAQ Capital Market
AIRTP,\"Air T, Inc. - Trust Preferred Securities\",NASDAQ Global MarketSM
AISP,\"Airship AI Holdings, Inc - Class A Common Stock\",NASDAQ Global MarketSM
AISPW,\"Airship AI Holdings, Inc - Warrants\",NASDAQ Capital Market
AITR,AI TRANSPORTATION ACQUISITION CORP - Ordinary shares,NASDAQ Capital Market
AITRR,AI TRANSPORTATION ACQUISITION CORP - Right,NASDAQ Capital Market
AITRU,AI TRANSPORTATION ACQUISITION CORP - Unit,NASDAQ Capital Market
AIXI,XIAO-I Corporation - American Depositary Shares,NASDAQ Global MarketSM
AKAM,\"Akamai Technologies, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AKAN,Akanda Corp. - Common Shares,NASDAQ Capital Market
AKBA,\"Akebia Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
AKLI,\"Akili, Inc. - Common Stock\",NASDAQ Capital Market
AKRO,\"Akero Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AKTS,\"Akoustis Technologies, Inc. - Common Stock\",NASDAQ Capital Market
AKTX,Akari Therapeutics Plc - American Depositary Shares,NASDAQ Capital Market
AKYA,\"Akoya BioSciences, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ALAB,\"Astera Labs, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ALAR,Alarum Technologies Ltd. - American Depositary Shares,NASDAQ Capital Market
ALBT,Avalon GloboCare Corp. - Common Stock,NASDAQ Capital Market
ALCE,\"Alternus Clean Energy, Inc. - Class A Common Stock\",NASDAQ Capital Market
ALCO,\"Alico, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ALCY,Alchemy Investments Acquisition Corp 1 - Class A Ordinary Shares,NASDAQ Global MarketSM
ALCYU,Alchemy Investments Acquisition Corp 1 - Units,NASDAQ Global MarketSM
ALCYW,Alchemy Investments Acquisition Corp 1 - Warrants,NASDAQ Global MarketSM
ALDX,\"Aldeyra Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
ALEC,\"Alector, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ALGM,\"Allegro MicroSystems, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ALGN,\"Align Technology, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ALGS,\"Aligos Therapeutics, Inc. - Common stock\",NASDAQ Capital Market
ALGT,Allegiant Travel Company - Common Stock,NASDAQ Global Select MarketSM
ALHC,\"Alignment Healthcare, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ALIM,\"Alimera Sciences, Inc. - Common Stock\",NASDAQ Global MarketSM
ALKS,Alkermes plc - Ordinary Shares,NASDAQ Global Select MarketSM
ALKT,\"Alkami Technology, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ALLK,Allakos Inc. - Common Stock,NASDAQ Global Select MarketSM
ALLO,\"Allogene Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ALLR,\"Allarity Therapeutics, Inc. - Common stock\",NASDAQ Capital Market
ALLT,Allot Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
ALNT,Allient Inc. - Common Stock,NASDAQ Global MarketSM
ALNY,\"Alnylam Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ALOT,\"AstroNova, Inc. - Common Stock\",NASDAQ Global MarketSM
ALPP,\"Alpine 4 Holdings, Inc. - Class A Common Stock\",NASDAQ Capital Market
ALRM,\"Alarm.com Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ALRN,\"Aileron Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
ALRS,Alerus Financial Corporation - Common Stock,NASDAQ Capital Market
ALSA,Alpha Star Acquisition Corporation - Ordinary Shares,NASDAQ Global MarketSM
ALSAR,Alpha Star Acquisition Corporation - Rights,NASDAQ Global MarketSM
ALSAU,Alpha Star Acquisition Corporation - Units,NASDAQ Global MarketSM
ALSAW,Alpha Star Acquisition Corporation - Warrants,NASDAQ Global MarketSM
ALT,\"Altimmune, Inc. - Common Stock\",NASDAQ Global MarketSM
ALTI,\"AlTi Global, Inc. - Class A Common Stock\",NASDAQ Capital Market
ALTO,\"Alto Ingredients, Inc. - Common Stock\",NASDAQ Capital Market
ALTR,Altair Engineering Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
ALTY,Global X Alternative Income ETF,NASDAQ Global MarketSM
ALVO,Alvotech - Ordinary Shares,NASDAQ Global MarketSM
ALVOW,Alvotech - Warrant,NASDAQ Global MarketSM
ALVR,\"AlloVir, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ALXO,ALX Oncology Holdings Inc. - Common Stock,NASDAQ Global Select MarketSM
ALZN,\"Alzamend Neuro, Inc. - Common Stock\",NASDAQ Capital Market
AMAL,Amalgamated Financial Corp. - Common Stock,NASDAQ Global MarketSM
AMAT,\"Applied Materials, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AMBA,\"Ambarella, Inc. - Ordinary Shares\",NASDAQ Global Select MarketSM
AMCX,AMC Networks Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
AMD,\"Advanced Micro Devices, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AMDL,GraniteShares 2x Long AMD Daily ETF,NASDAQ Global MarketSM
AMDS,GraniteShares 1x Short AMD Daily ETF,NASDAQ Global MarketSM
AMED,Amedisys Inc - Common Stock,NASDAQ Global Select MarketSM
AMGN,Amgen Inc. - Common Stock,NASDAQ Global Select MarketSM
AMID,Argent Mid Cap ETF,NASDAQ Global MarketSM
AMIX,\"Autonomix Medical, Inc. - Common Stock\",NASDAQ Capital Market
AMKR,\"Amkor Technology, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AMLI,American Lithium Corp. - Common Stock,NASDAQ Capital Market
AMLX,\"Amylyx Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AMPG,\"Amplitech Group, Inc. - Common Stock\",NASDAQ Capital Market
AMPGW,\"Amplitech Group, Inc. - Warrants\",NASDAQ Capital Market
AMPH,\"Amphastar Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AMPL,\"Amplitude, Inc. - Class A Common Stock\",NASDAQ Capital Market
AMRK,\"A-Mark Precious Metals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AMRN,\"Amarin Corporation plc - American Depositary Shares, each representing one Ordinary Share\",NASDAQ Global MarketSM
AMRX,\"Amneal Pharmaceuticals, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
AMSC,American Superconductor Corporation - Common Stock,NASDAQ Global Select MarketSM
AMSF,\"AMERISAFE, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AMST,Amesite Inc. - Common Stock,NASDAQ Capital Market
AMSWA,\"American Software, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
AMTX,\"Aemetis, Inc - Common Stock\",NASDAQ Global MarketSM
AMWD,American Woodmark Corporation - Common Stock,NASDAQ Global Select MarketSM
AMZD,Direxion Daily AMZN Bear 1X Shares,NASDAQ Global MarketSM
AMZN,\"Amazon.com, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AMZU,Direxion Daily AMZN Bull 2X Shares,NASDAQ Global MarketSM
AMZZ,GraniteShares 2x Long AMZN Daily ETF,NASDAQ Global MarketSM
ANAB,\"AnaptysBio, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ANDE,\"The Andersons, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ANEB,\"Anebulo Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
ANGH,Anghami Inc. - Ordinary Shares,NASDAQ Global MarketSM
ANGHW,Anghami Inc. - Warrants,NASDAQ Capital Market
ANGI,Angi Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
ANGL,VanEck Fallen Angel High Yield Bond ETF,NASDAQ Global MarketSM
ANGO,\"AngioDynamics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ANIK,Anika Therapeutics Inc. - Common Stock,NASDAQ Global Select MarketSM
ANIP,\"ANI Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global MarketSM
ANIX,\"Anixa Biosciences, Inc. - Common Stock\",NASDAQ Capital Market
ANL,Adlai Nortye Ltd. - American Depositary Shares,NASDAQ Global MarketSM
ANNX,\"Annexon, Inc. - common stock\",NASDAQ Global Select MarketSM
ANSC,Agriculture & Natural Solutions Acquisition Corporation - Class A Ordinary Shares,NASDAQ Global MarketSM
ANSCU,Agriculture & Natural Solutions Acquisition Corporation - Unit,NASDAQ Global MarketSM
ANSCW,Agriculture & Natural Solutions Acquisition Corporation - Warrant,NASDAQ Global MarketSM
ANSS,\"ANSYS, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ANTE,\"AirNet Technology Inc. - American Depositary Shares, each representing one ordinary shares\",NASDAQ Capital Market
ANTX,\"AN2 Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ANY,Sphere 3D Corp. - Common Shares,NASDAQ Capital Market
AOGO,Arogo Capital Acquisition Corp. - Class A Common Stock,NASDAQ Global MarketSM
AOGOU,Arogo Capital Acquisition Corp. - Unit,NASDAQ Global MarketSM
AOGOW,Arogo Capital Acquisition Corp. - Warrant,NASDAQ Global MarketSM
AONC,\"American Oncology Network, Inc. - Class A Common Stock\",NASDAQ Capital Market
AONCW,\"American Oncology Network, Inc. - Warrant\",NASDAQ Capital Market
AOSL,Alpha and Omega Semiconductor Limited - Common Shares,NASDAQ Global Select MarketSM
AOTG,AOT Growth and Innovation ETF,NASDAQ Global MarketSM
AOUT,\"American Outdoor Brands, Inc. - Common Stock\",NASDAQ Global Select MarketSM
APA,APA Corporation - Common Stock,NASDAQ Global Select MarketSM
APCX,AppTech Payments Corp. - Common stock,NASDAQ Capital Market
APCXW,AppTech Payments Corp. - Warrant,NASDAQ Capital Market
APDN,\"Applied DNA Sciences, Inc. - Common Stock\",NASDAQ Capital Market
APEI,\"American Public Education, Inc. - Common Stock\",NASDAQ Global Select MarketSM
APGE,\"Apogee Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
API,\"Agora, Inc. - ADS\",NASDAQ Global Select MarketSM
APLD,Applied Digital Corporation - Common Stock,NASDAQ Global Select MarketSM
APLM,Apollomics Inc. - Class A Ordinary Shares,NASDAQ Capital Market
APLMW,Apollomics Inc. - Warrant,NASDAQ Capital Market
APLS,\"Apellis Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
APLT,\"Applied Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
APM,Aptorum Group Limited - Class A Ordinary Shares,NASDAQ Capital Market
APOG,\"Apogee Enterprises, Inc. - Common Stock\",NASDAQ Global Select MarketSM
APP,Applovin Corporation - Class A Common Stock,NASDAQ Global Select MarketSM
APPF,\"AppFolio, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
APPN,Appian Corporation - Class A Common Stock,NASDAQ Global MarketSM
APPS,\"Digital Turbine, Inc. - Common Stock\",NASDAQ Capital Market
APRE,\"Aprea Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
APTO,\"Aptose Biosciences, Inc. - Common Shares\",NASDAQ Capital Market
APVO,Aptevo Therapeutics Inc. - Common Stock,NASDAQ Capital Market
APWC,\"Asia Pacific Wire & Cable Corporation Limited  - Common shares, Par value .01 per share\",NASDAQ Capital Market
APXI,APx Acquisition Corp. I - Class A Ordinary Share,NASDAQ Global MarketSM
APXIU,APx Acquisition Corp. I - Unit,NASDAQ Global MarketSM
APXIW,APx Acquisition Corp. I - Warrant,NASDAQ Global MarketSM
APYX,Apyx Medical Corporation - Common Stock,NASDAQ Global Select MarketSM
AQB,\"AquaBounty Technologies, Inc. - Common Stock\",NASDAQ Capital Market
AQMS,\"Aqua Metals, Inc. - Common Stock\",NASDAQ Capital Market
AQST,\"Aquestive Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
AQU,Aquaron Acquisition Corp. - Common Stock,NASDAQ Capital Market
AQUNR,Aquaron Acquisition Corp. - Rights,NASDAQ Capital Market
AQUNU,Aquaron Acquisition Corp. - Units,NASDAQ Capital Market
AQWA,Global X Clean Water ETF,NASDAQ Global MarketSM
ARAY,Accuray Incorporated - Common Stock,NASDAQ Global Select MarketSM
ARBB,ARB IOT Group Limited - Ordinary Shares,NASDAQ Capital Market
ARBE,Arbe Robotics Ltd. - Ordinary Shares,NASDAQ Capital Market
ARBEW,Arbe Robotics Ltd. - Warrant,NASDAQ Capital Market
ARBK,Argo Blockchain plc - American Depositary Shares,NASDAQ Global Select MarketSM
ARBKL,Argo Blockchain plc - 8.75% Senior Notes due 2026,NASDAQ Global MarketSM
ARCB,ArcBest Corporation - Common Stock,NASDAQ Global Select MarketSM
ARCC,Ares Capital Corporation - Closed End Fund,NASDAQ Global Select MarketSM
ARCT,Arcturus Therapeutics Holdings Inc. - Common Stock,NASDAQ Global MarketSM
ARDX,\"Ardelyx, Inc. - Common Stock\",NASDAQ Global MarketSM
AREB,\"American Rebel Holdings, Inc. - Common Stock\",NASDAQ Capital Market
AREBW,\"American Rebel Holdings, Inc. - warrants\",NASDAQ Capital Market
AREC,American Resources Corporation - Class A Common Stock,NASDAQ Capital Market
ARGX,argenx SE - American Depositary Shares,NASDAQ Global Select MarketSM
ARHS,\"Arhaus, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
ARKO,ARKO Corp. - Common Stock,NASDAQ Capital Market
ARKOW,ARKO Corp. - Warrant,NASDAQ Capital Market
ARKR,Ark Restaurants Corp. - Common Stock,NASDAQ Global MarketSM
ARLP,\"Alliance Resource Partners, L.P. - Common Units Representing Limited Partnership Interests\",NASDAQ Global Select MarketSM
ARM,Arm Holdings plc - American Depositary Shares,NASDAQ Global Select MarketSM
AROW,Arrow Financial Corporation - Common Stock,NASDAQ Global Select MarketSM
ARQ,\"Arq, Inc. - Common Stock\",NASDAQ Global MarketSM
ARQQ,Arqit Quantum Inc. - Ordinary Shares,NASDAQ Capital Market
ARQQW,Arqit Quantum Inc. - Warrants,NASDAQ Capital Market
ARQT,\"Arcutis Biotherapeutics, Inc. - Common stock\",NASDAQ Global Select MarketSM
ARRY,\"Array Technologies, Inc. - Common Stock\",NASDAQ Global MarketSM
ARTL,\"Artelo Biosciences, Inc. - Common Stock\",NASDAQ Capital Market
ARTLW,\"Artelo Biosciences, Inc. - Warrant\",NASDAQ Capital Market
ARTNA,Artesian Resources Corporation - Class A Non-Voting Common Stock,NASDAQ Global Select MarketSM
ARTW,\"Art's-Way Manufacturing Co., Inc. - Common Stock\",NASDAQ Capital Market
ARVN,\"Arvinas, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ARVR,First Trust Indxx Metaverse ETF,NASDAQ Global MarketSM
ARWR,\"Arrowhead Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ARYD,ARYA Sciences Acquisition Corp IV - Class A Ordinary Shares,NASDAQ Capital Market
ASCB,\"A SPAC II Acquisition Corp. - Ordinary Shares, Class A Common Stock\",NASDAQ Global MarketSM
ASCBR,A SPAC II Acquisition Corp. - Right,NASDAQ Global MarketSM
ASCBU,A SPAC II Acquisition Corp. - Unit,NASDAQ Global MarketSM
ASCBW,A SPAC II Acquisition Corp. - Warrant,NASDAQ Global MarketSM
ASET,FlexShares Real Assets Allocation Index Fund,NASDAQ Global MarketSM
ASLE,AerSale Corporation - Common Stock,NASDAQ Capital Market
ASLN,ASLAN Pharmaceuticals Limited - American Depositary Shares,NASDAQ Capital Market
ASMB,\"Assembly Biosciences, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ASML,ASML Holding N.V. - New York Registry Shares,NASDAQ Global Select MarketSM
ASND,Ascendis Pharma A/S - American Depositary Shares,NASDAQ Global Select MarketSM
ASNS,\"Actelis Networks, Inc. - Common Stock\",NASDAQ Capital Market
ASO,\"Academy Sports and Outdoors, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ASPI,ASP Isotopes Inc. - Common Stock,NASDAQ Capital Market
ASPS,Altisource Portfolio Solutions S.A. - Common Stock,NASDAQ Global Select MarketSM
ASRT,\"Assertio Holdings, Inc. - Common Stock\",NASDAQ Capital Market
ASRV,AmeriServ Financial Inc. - Common Stock,NASDAQ Global MarketSM
ASST,Asset Entities Inc. - Class B Common Stock,NASDAQ Capital Market
ASTC,Astrotech Corporation - Common Stock,NASDAQ Capital Market
ASTE,\"Astec Industries, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ASTH,Astrana Health Inc. - Common Stock,NASDAQ Capital Market
ASTI,\"Ascent Solar Technologies, Inc - Common Stock\",NASDAQ Capital Market
ASTL,Algoma Steel Group Inc. - Common Shares,NASDAQ Global MarketSM
ASTLW,Algoma Steel Group Inc. - Warrant,NASDAQ Global MarketSM
ASTR,\"Astra Space, Inc. - Class A Common Stock\",NASDAQ Capital Market
ASTS,\"AST SpaceMobile, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
ASTSW,\"AST SpaceMobile, Inc. - Warrant\",NASDAQ Global Select MarketSM
ASUR,Asure Software Inc - Common Stock,NASDAQ Capital Market
ASYS,\"Amtech Systems, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ATAI,ATAI Life Sciences N.V. - Common Shares,NASDAQ Global MarketSM
ATAT,Atour Lifestyle Holdings Limited - American Depositary Shares,NASDAQ Global Select MarketSM
ATCOL,Atlas Corp. - 7.125% Notes due 2027,NASDAQ Global MarketSM
ATEC,\"Alphatec Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ATER,\"Aterian, Inc. - Common Stock\",NASDAQ Capital Market
ATEX,Anterix Inc. - Common Stock,NASDAQ Capital Market
ATGL,Alpha Technology Group Limited - Ordinary Shares,NASDAQ Capital Market
ATHA,\"Athira Pharma, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ATHE,Alterity Therapeutics Limited - American Depositary Shares,NASDAQ Capital Market
ATIF,ATIF Holdings Limited - Ordinary Shares,NASDAQ Capital Market
ATLC,Atlanticus Holdings Corporation - Common Stock,NASDAQ Global Select MarketSM
ATLCL,Atlanticus Holdings Corporation - 6.125% Senior Notes due 2026,NASDAQ Global MarketSM
ATLCP,\"Atlanticus Holdings Corporation - 7.625% Series B Cumulative Perpetual Preferred Stock, no par value per share\",NASDAQ Global Select MarketSM
ATLCZ,Atlanticus Holdings Corporation - 9.25% Senior Notes due 2029,NASDAQ Global MarketSM
ATLO,Ames National Corporation - Common Stock,NASDAQ Capital Market
ATLX,Atlas Lithium Corporation - Common Stock,NASDAQ Capital Market
ATMC,AlphaTime Acquisition Corp - Ordinary Shares,NASDAQ Global MarketSM
ATMCR,AlphaTime Acquisition Corp - Right,NASDAQ Global MarketSM
ATMCU,AlphaTime Acquisition Corp - Unit,NASDAQ Global MarketSM
ATMCW,AlphaTime Acquisition Corp - Warrant,NASDAQ Global MarketSM
ATMV,AlphaVest Acquisition Corp - Ordinary Shares,NASDAQ Global MarketSM
ATMVR,AlphaVest Acquisition Corp - Right,NASDAQ Global MarketSM
ATMVU,AlphaVest Acquisition Corp - Unit,NASDAQ Global MarketSM
ATNF,180 Life Sciences Corp. - Common Stock,NASDAQ Capital Market
ATNFW,180 Life Sciences Corp. - Warrant,NASDAQ Capital Market
ATNI,\"ATN International, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ATOM,Atomera Incorporated - Common Stock,NASDAQ Capital Market
ATOS,\"Atossa Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
ATPC,Agape ATP Corporation - Common Stock,NASDAQ Capital Market
ATRA,\"Atara Biotherapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ATRC,\"AtriCure, Inc. - Common Stock\",NASDAQ Global MarketSM
ATRI,Atrion Corporation - Common Stock,NASDAQ Global Select MarketSM
ATRO,Astronics Corporation - Common Stock,NASDAQ Global Select MarketSM
ATSG,\"Air Transport Services Group, Inc - Common Stock\",NASDAQ Global Select MarketSM
ATXG,Addentax Group Corp. - Common Stock,NASDAQ Capital Market
ATXI,\"Avenue Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
ATXS,\"Astria Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
AUBN,\"Auburn National Bancorporation, Inc. - Common Stock\",NASDAQ Global MarketSM
AUDC,AudioCodes Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
AUGX,\"Augmedix, Inc. - Common Stock\",NASDAQ Capital Market
AUID,authID Inc. - Common Stock,NASDAQ Capital Market
AUMI,Themes Gold Miners ETF,NASDAQ Global MarketSM
AUPH,Aurinia Pharmaceuticals Inc - Common Shares,NASDAQ Global MarketSM
AUR,\"Aurora Innovation, Inc.  - Class A Common Stock\",NASDAQ Global Select MarketSM
AURA,\"Aura Biosciences, Inc. - Common Stock\",NASDAQ Global MarketSM
AUROW,\"Aurora Innovation, Inc.  - Warrant\",NASDAQ Global Select MarketSM
AUTL,Autolus Therapeutics plc - American Depositary Shares,NASDAQ Global Select MarketSM
AUUD,Auddia Inc. - Common Stock,NASDAQ Capital Market
AUUDW,Auddia Inc. - Warrants,NASDAQ Capital Market
AUVI,\"Applied UV, Inc. - Common Stock\",NASDAQ Capital Market
AUVIP,\"Applied UV, Inc. - 10.5% Series A Cumulative Perpetual Preferred Stock, $0.0001 par value per share\",NASDAQ Capital Market
AVAH,Aveanna Healthcare Holdings Inc. - Common Stock,NASDAQ Global Select MarketSM
AVAV,\"AeroVironment, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AVBP,\"ArriVent BioPharma, Inc. - Common Stock\",NASDAQ Global MarketSM
AVDL,Avadel Pharmaceuticals plc - Ordinary Share,NASDAQ Global MarketSM
AVDX,\"AvidXchange Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AVGO,Broadcom Inc. - Common Stock,NASDAQ Global Select MarketSM
AVGR,\"Avinger, Inc. - Common Stock\",NASDAQ Capital Market
AVIR,\"Atea Pharmaceuticals, Inc. - common stock\",NASDAQ Global Select MarketSM
AVNW,\"Aviat Networks, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AVO,\"Mission Produce, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AVPT,\"AvePoint, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
AVPTW,\"AvePoint, Inc. - Warrant\",NASDAQ Global Select MarketSM
AVRO,\"AVROBIO, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AVT,\"Avnet, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AVTE,\"Aerovate Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
AVTX,\"Avalo Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
AVXC,Avantis Emerging Markets ex-China Equity ETF,NASDAQ Global MarketSM
AVXL,Anavex Life Sciences Corp. - Common Stock,NASDAQ Global Select MarketSM
AWH,Aspira Women's Health Inc. - Common Stock,NASDAQ Capital Market
AWIN,AERWINS Technologies Inc. - Common Stock,NASDAQ Capital Market
AWINW,AERWINS Technologies Inc. - Warrant,NASDAQ Capital Market
AWRE,\"Aware, Inc. - Common Stock\",NASDAQ Global MarketSM
AXDX,\"Accelerate Diagnostics, Inc. - Common Stock\",NASDAQ Capital Market
AXGN,\"Axogen, Inc. - Common Stock\",NASDAQ Capital Market
AXNX,\"Axonics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AXON,\"Axon Enterprise, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AXSM,\"Axsome Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
AXTI,AXT Inc - Common Stock,NASDAQ Global Select MarketSM
AY,Atlantica Sustainable Infrastructure plc - Ordinary Shares,NASDAQ Global Select MarketSM
AYRO,\"AYRO, Inc. - Common Stock\",NASDAQ Capital Market
AYTU,\"Aytu BioPharma, Inc.  - Common Stock\",NASDAQ Capital Market
AZ,A2Z Smart Technologies Corp. - Common Shares,NASDAQ Capital Market
AZN,AstraZeneca PLC - American Depositary Shares,NASDAQ Global Select MarketSM
AZPN,\"Aspen Technology, Inc. - Common Stock\",NASDAQ Global Select MarketSM
AZTA,\"Azenta, Inc. - Common Stock\",NASDAQ Global Select MarketSM
BABX,GraniteShares 2x Long BABA Daily ETF,NASDAQ Global MarketSM
BACK,\"IMAC Holdings, Inc. - Common Stock\",NASDAQ Capital Market
BAER,\"Bridger Aerospace Group Holdings, Inc. - Common Stock\",NASDAQ Global MarketSM
BAERW,\"Bridger Aerospace Group Holdings, Inc. - Warrant\",NASDAQ Global MarketSM
BAFN,BayFirst Financial Corp. - Common Stock,NASDAQ Capital Market
BAND,Bandwidth Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
BANF,BancFirst Corporation - Common Stock,NASDAQ Global Select MarketSM
BANFP,BancFirst Corporation - 7.2% Cumulative Trust Preferred Securities,NASDAQ Global Select MarketSM
BANL,CBL International Limited - Ordinary Shares,NASDAQ Capital Market
BANR,Banner Corporation - Common Stock,NASDAQ Global Select MarketSM
BANX,ArrowMark Financial Corp. - Closed End Fund,NASDAQ Global Select MarketSM
BAOS,Baosheng Media Group Holdings Limited - Ordinary shares,NASDAQ Capital Market
BASE,\"Couchbase, Inc. - Common Stock\",NASDAQ Global Select MarketSM
BATRA,\"Atlanta Braves Holdings, Inc. - Series A Common Stock\",NASDAQ Global Select MarketSM
BATRK,\"Atlanta Braves Holdings, Inc. - Series C Common Stock\",NASDAQ Global Select MarketSM
BAYA,Bayview Acquisition Corp - Ordinary Share,NASDAQ Global MarketSM
BAYAR,Bayview Acquisition Corp - Right,NASDAQ Global MarketSM
BAYAU,Bayview Acquisition Corp - Unit,NASDAQ Global MarketSM
BBCP,\"Concrete Pumping Holdings, Inc.  - Common Stock\",NASDAQ Capital Market
BBGI,\"Beasley Broadcast Group, Inc. - Class A Common Stock\",NASDAQ Capital Market
BBH,VanEck Biotech ETF,NASDAQ Global MarketSM
BBIO,\"BridgeBio Pharma, Inc. - Common Stock\",NASDAQ Global Select MarketSM
BBLG,Bone Biologics Corp - Common Stock,NASDAQ Capital Market
BBLGW,Bone Biologics Corp - warrants,NASDAQ Capital Market
BBSI,\"Barrett Business Services, Inc. - Common Stock\",NASDAQ Global Select MarketSM
BCAB,\"BioAtla, Inc. - Common Stock\",NASDAQ Global MarketSM
BCAL,Southern California Bancorp - Common Stock,NASDAQ Capital Market
BCAN,BYND Cannasoft Enterprises Inc. - Common Stock,NASDAQ Capital Market
BCBP,\"BCB Bancorp, Inc. (NJ) - Common Stock\",NASDAQ Global MarketSM
BCDA,\"BioCardia, Inc. - Common Stock\",NASDAQ Capital Market
BCDAW,\"BioCardia, Inc. - Warrant\",NASDAQ Capital Market
BCG,\"Binah Capital Group, Inc. - Common Stock\",NASDAQ Global MarketSM
BCGWW,\"Binah Capital Group, Inc. - Warrants\",NASDAQ Capital Market
BCLI,Brainstorm Cell Therapeutics Inc. - Common Stock,NASDAQ Capital Market
BCML,BayCom Corp - Common Stock,NASDAQ Global Select MarketSM
BCOV,Brightcove Inc. - Common Stock,NASDAQ Global Select MarketSM
BCOW,\"1895 Bancorp of Wisconsin, Inc. - Common Stock\",NASDAQ Capital Market
BCPC,Balchem Corporation - Common Stock,NASDAQ Global Select MarketSM
BCRX,\"BioCryst Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
BCSA,Blockchain Coinvestors Acquisition Corp. I - Class A Ordinary Shares,NASDAQ Global MarketSM
BCSAU,Blockchain Coinvestors Acquisition Corp. I - Unit,NASDAQ Global MarketSM
BCSAW,Blockchain Coinvestors Acquisition Corp. I - Warrant,NASDAQ Global MarketSM
BCTX,BriaCell Therapeutics Corp. - Common Shares,NASDAQ Capital Market
BCTXW,BriaCell Therapeutics Corp. - Warrant,NASDAQ Capital Market
BCYC,Bicycle Therapeutics plc - American Depositary Shares,NASDAQ Global Select MarketSM
BDGS,Bridges Capital Tactical ETF,NASDAQ Global MarketSM
BDRX,Biodexa Pharmaceuticals plc - American Depositary Shares,NASDAQ Capital Market
BDSX,\"Biodesix, Inc. - Common Stock\",NASDAQ Global MarketSM
BDTX,\"Black Diamond Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
BEAM,Beam Therapeutics Inc. - Common Stock,NASDAQ Global Select MarketSM
BEAT,\"Heartbeam, Inc. - Common Stock\",NASDAQ Capital Market
BEATW,\"Heartbeam, Inc. - Warrant\",NASDAQ Capital Market
BECN,\"Beacon Roofing Supply, Inc. - Common Stock\",NASDAQ Global Select MarketSM
BEEM,Beam Global - Common Stock,NASDAQ Capital Market
BEEZ,Honeytree U.S. Equity ETF,NASDAQ Global MarketSM
BELFA,Bel Fuse Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
BELFB,Bel Fuse Inc. - Class B Common Stock,NASDAQ Global Select MarketSM
BENF,Beneficient - Class A Common Stock,NASDAQ Capital Market
BENFW,Beneficient - Warrant,NASDAQ Capital Market
BETR,Better Home & Finance Holding Company - Class A Common Stock,NASDAQ Capital Market
BETRW,Better Home & Finance Holding Company - Warrant,NASDAQ Capital Market
BFC,Bank First Corporation - Common Stock,NASDAQ Capital Market
BFI,BurgerFi International Inc - Common Stock,NASDAQ Global MarketSM
BFIIW,BurgerFi International Inc - Warrant,NASDAQ Global MarketSM
BFIN,BankFinancial Corporation - Common Stock,NASDAQ Global Select MarketSM
BFRG,\"Bullfrog AI Holdings, Inc. - Common Stock\",NASDAQ Capital Market
BFRGW,\"Bullfrog AI Holdings, Inc. - Warrants\",NASDAQ Capital Market
BFRI,Biofrontera Inc. - Common Stock,NASDAQ Capital Market
BFRIW,Biofrontera Inc. - Warrants,NASDAQ Capital Market
BFST,\"Business First Bancshares, Inc. - Common Stock\",NASDAQ Global Select MarketSM
BGC,\"BGC Group, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
BGFV,Big 5 Sporting Goods Corporation - Common Stock,NASDAQ Global Select MarketSM
BGLC,BioNexus Gene Lab Corp - Common stock,NASDAQ Capital Market
BGNE,\"BeiGene, Ltd. - American Depositary Shares\",NASDAQ Global Select MarketSM
BGRN,iShares USD Green Bond ETF,NASDAQ Global MarketSM
BGXX,Bright Green Corporation - Common Stock,NASDAQ Capital Market
BHAC,Focus Impact BH3 Acquisition Company - Class A Common Stock,NASDAQ Global MarketSM
BHACU,Focus Impact BH3 Acquisition Company - Units,NASDAQ Global MarketSM
BHACW,Focus Impact BH3 Acquisition Company - Warrants,NASDAQ Global MarketSM
BHAT,Blue Hat Interactive Entertainment Technology - Ordinary Shares,NASDAQ Capital Market
BHF,\"Brighthouse Financial, Inc. - Common Stock\",NASDAQ Global Select MarketSM
BHFAL,\"Brighthouse Financial, Inc. - Junior Subordinated Debentures due 2058\",NASDAQ Global Select MarketSM
BHFAM,\"Brighthouse Financial, Inc. - Depositary shares each representing a 1/1,000th Interest in a Share of 4.625% Non-Cumulative Preferred Stock, Series D\",NASDAQ Global Select MarketSM
BHFAN,\"Brighthouse Financial, Inc. - depositary shares, each representing a 1/1,000th interest in a share of 5.375% Non-Cumulative Preferred Stock, Series C\",NASDAQ Global Select MarketSM
BHFAO,\"Brighthouse Financial, Inc. - Depositary Shares, each representing a 1/1,000th interest in a share of 6.750% Non-Cumulative Preferred Stock, Series B\",NASDAQ Global Select MarketSM
BHFAP,\"Brighthouse Financial, Inc. - Depositary Shares 6.6% Non-Cumulative Preferred Stock, Series A\",NASDAQ Global Select MarketSM
BHRB,Burke & Herbert Financial Services Corp. - Common Stock,NASDAQ Capital Market
BIAF,\"bioAffinity Technologies, Inc. - Common Stock\",NASDAQ Capital Market
BIAFW,\"bioAffinity Technologies, Inc. - Warrant\",NASDAQ Capital Market
BIB,ProShares Ultra Nasdaq Biotechnology,NASDAQ Global MarketSM
BIDU,\"Baidu, Inc. - American Depositary Shares, each representing 8 ordinary share\",NASDAQ Global Select MarketSM
BIGC,\"BigCommerce Holdings, Inc. - Series 1 Common Stock\",NASDAQ Global MarketSM
BIIB,Biogen Inc. - Common Stock,NASDAQ Global Select MarketSM
BILI,Bilibili Inc. - American Depositary Shares,NASDAQ Global Select MarketSM
BIMI,BIMI International Medical Inc. - Common Stock,NASDAQ Capital Market
BIOL,\"Biolase, Inc. - Common Stock\",NASDAQ Capital Market
BIOR,\"Biora Therapeutics, Inc.  - Common Stock\",NASDAQ Global MarketSM
BIOX,Bioceres Crop Solutions Corp. - Ordinary Shares,NASDAQ Global Select MarketSM
BIRD,\"Allbirds, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
BIS,ProShares UltraShort Nasdaq Biotechnology,NASDAQ Global MarketSM
BITF,Bitfarms Ltd. - Common Stock,NASDAQ Global MarketSM
BITS,Global X Blockchain & Bitcoin Strategy ETF,NASDAQ Global MarketSM
BIVI,BioVie Inc. - Common stock,NASDAQ Capital Market
BJDX,\"Bluejay Diagnostics, Inc. - Common Stock\",NASDAQ Capital Market
BJK,VanEck Gaming ETF,NASDAQ Global MarketSM
BJRI,\"BJ's Restaurants, Inc. - Common Stock\",NASDAQ Global Select MarketSM
BKCH,Global X Blockchain ETF,NASDAQ Global MarketSM
BKHA,Black Hawk Acquisition Corporation - Class A Ordinary Shares,NASDAQ Global MarketSM
BKHAR,Black Hawk Acquisition Corporation - Rights,NASDAQ Global MarketSM
BKHAU,Black Hawk Acquisition Corporation - Units,NASDAQ Global MarketSM
BKIV,BNY Mellon Innovators ETF,NASDAQ Global MarketSM
BKNG,Booking Holdings Inc. - Common Stock,NASDAQ Global Select MarketSM
BKR,Baker Hughes Company - Common Stock,NASDAQ Global Select MarketSM
BKWO,BNY Mellon Women's Opportunities ETF,NASDAQ Global MarketSM
BKYI,\"BIO-key International, Inc. - Common Stock\",NASDAQ Capital Market
BL,\"BlackLine, Inc. - Common Stock\",NASDAQ Global Select MarketSM
BLAC,Bellevue Life Sciences Acquisition Corp. - Common Stock,NASDAQ Capital Market
BLACR,Bellevue Life Sciences Acquisition Corp. - Rights,NASDAQ Capital Market
BLACU,Bellevue Life Sciences Acquisition Corp. - Unit,NASDAQ Capital Market
BLACW,Bellevue Life Sciences Acquisition Corp. - Warrant,NASDAQ Capital Market
BLBD,Blue Bird Corporation - Common Stock,NASDAQ Global MarketSM
BLBX,Blackboxstocks Inc. - Common Stock,NASDAQ Capital Market
BLCN,Siren Nasdaq NexGen Economy ETF,NASDAQ Global MarketSM
BLCR,BlackRock Large Cap Core ETF,NASDAQ Global MarketSM
BLDE,\"Blade Air Mobility, Inc. - Class A Common Stock\",NASDAQ Capital Market
BLDEW,\"Blade Air Mobility, Inc. - Warrants\",NASDAQ Capital Market
BLDP,\"Ballard Power Systems, Inc. - Common Shares\",NASDAQ Global MarketSM
BLEU,bleuacacia ltd - Class A Ordinary Shares,NASDAQ Capital Market
BLEUR,bleuacacia ltd - Rights,NASDAQ Capital Market
BLEUU,bleuacacia ltd - Units,NASDAQ Capital Market
BLEUW,bleuacacia ltd - Warrants,NASDAQ Capital Market
BLFS,\"BioLife Solutions, Inc. - Common Stock\",NASDAQ Capital Market
BLFY,Blue Foundry Bancorp - Common Stock,NASDAQ Global Select MarketSM
BLIN,\"Bridgeline Digital, Inc. - Common Stock\",NASDAQ Capital Market
BLKB,\"Blackbaud, Inc. - Common Stock\",NASDAQ Global Select MarketSM
BLLD,JPMorgan Sustainable Infrastructure ETF,NASDAQ Global MarketSM
BLMN,\"Bloomin' Brands, Inc. - Common Stock\",NASDAQ Global Select MarketSM
BLNK,Blink Charging Co. - Common Stock,NASDAQ Capital Market
BLRX,BioLineRx Ltd. - American Depositary Shares,NASDAQ Capital Market
BLTE,\"Belite Bio, Inc - American Depositary Shares\",NASDAQ Capital Market
BLUE,\"bluebird bio, Inc. - Common Stock\",NASDAQ Global Select MarketSM
BLZE,\"Backblaze, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
BMBL,Bumble Inc. - common stock,NASDAQ Global Select MarketSM
BMEA,\"Biomea Fusion, Inc. - Common Stock\",NASDAQ Global Select MarketSM
BMR,Beamr Imaging Ltd. - Ordinary Share,NASDAQ Capital Market
BMRA,\"Biomerica, Inc. - Common Stock\",NASDAQ Capital Market
BMRC,Bank of Marin Bancorp - Common Stock,NASDAQ Capital Market
BMRN,BioMarin Pharmaceutical Inc. - Common Stock,NASDAQ Global Select MarketSM
BNAI,Brand Engagement Network Inc. - Common Stock,NASDAQ Capital Market
BNAIW,Brand Engagement Network Inc. - Warrant,NASDAQ Capital Market
BND,Vanguard Total Bond Market ETF,NASDAQ Global MarketSM
BNDW,Vanguard Total World Bond ETF,NASDAQ Global MarketSM
BNDX,Vanguard Total International Bond ETF,NASDAQ Global MarketSM
BNGO,\"Bionano Genomics, Inc. - Common Stock\",NASDAQ Capital Market
BNIX,Bannix Acquisition Corp. - Common Stock,NASDAQ Capital Market
BNIXR,Bannix Acquisition Corp. - Right,NASDAQ Capital Market
BNIXW,Bannix Acquisition Corp. - Warrant,NASDAQ Capital Market
BNOX,Bionomics Limited - American Depository Shares,NASDAQ Global MarketSM
BNR,Burning Rock Biotech Limited - American Depositary Shares,NASDAQ Global MarketSM
BNRG,Brenmiller Energy Ltd - Ordinary Shares,NASDAQ Capital Market
BNTC,Benitec Biopharma Inc. - Common Stock,NASDAQ Capital Market
BNTX,BioNTech SE - American Depositary Shares,NASDAQ Global Select MarketSM
BNZI,\"Banzai International, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
BNZIW,\"Banzai International, Inc. - Warrant\",NASDAQ Capital Market
BOCN,Blue Ocean Acquisition Corp - Class A Ordinary Shares,NASDAQ Global MarketSM
BOCNU,Blue Ocean Acquisition Corp - Units,NASDAQ Global MarketSM
BOCNW,Blue Ocean Acquisition Corp - Warrants,NASDAQ Global MarketSM
BOF,BranchOut Food Inc. - Common Stock,NASDAQ Capital Market
BOKF,BOK Financial Corporation - Common Stock,NASDAQ Global Select MarketSM
BOLD,\"Boundless Bio, Inc. - Common Stock\",NASDAQ Global Select MarketSM
BOLT,\"Bolt Biotherapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
BON,Bon Natural Life Limited - Ordinary Shares,NASDAQ Capital Market
BOOM,DMC Global Inc. - Common Stock,NASDAQ Global Select MarketSM
BOSC,B.O.S. Better Online Solutions - Ordinary Shares,NASDAQ Capital Market
BOTJ,\"Bank of the James Financial Group, Inc. - Common Stock\",NASDAQ Capital Market
BOTT,Themes Robotics & Automation ETF,NASDAQ Global MarketSM
BOTZ,Global X Robotics & Artificial Intelligence ETF,NASDAQ Global MarketSM
BOWN,Bowen Acquisition Corp - Ordinary Shares,NASDAQ Global MarketSM
BOWNR,Bowen Acquisition Corp - Rights,NASDAQ Global MarketSM
BOWNU,Bowen Acquisition Corp - Unit,NASDAQ Global MarketSM
BOXL,Boxlight Corporation - Class A Common Stock,NASDAQ Capital Market
BPMC,Blueprint Medicines Corporation - Common Stock,NASDAQ Global Select MarketSM
BPOP,\"Popular, Inc. - Common Stock\",NASDAQ Global Select MarketSM
BPOPM,\"Popular, Inc. - Popular Capital Trust II - 6.125% Cumulative Monthly Income Trust Preferred Securities\",NASDAQ Global Select MarketSM
BPRN,\"Princeton Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
BPTH,\"Bio-Path Holdings, Inc. - Common Stock\",NASDAQ Capital Market
BPYPM,\"Brookfield Property Partners L.P. - 6.25% Class A Cumulative Redeemable Preferred Units, Series 1\",NASDAQ Global Select MarketSM
BPYPN,\"Brookfield Property Partners L.P. - 5.750% Class A Cumulative Redeemable Perpetual Preferred Units, Series 3\",NASDAQ Global Select MarketSM
BPYPO,\"Brookfield Property Partners L.P. - 6.375% Class A Cumulative Redeemable Perpetual Preferred Units, Series 2\",NASDAQ Global Select MarketSM
BPYPP,Brookfield Property Partners L.P. - 6.50% Class A Cumulative Redeemable Perpetual Preferred Units,NASDAQ Global Select MarketSM
BRAC,Broad Capital Acquisition Corp - Common Stock,NASDAQ Global MarketSM
BRACR,Broad Capital Acquisition Corp - Rights,NASDAQ Global MarketSM
BRACU,Broad Capital Acquisition Corp - Units,NASDAQ Global MarketSM
BRAG,Bragg Gaming Group Inc. - Common Shares,NASDAQ Global Select MarketSM
BREA,Brera Holdings PLC - Class B Ordinary Shares,NASDAQ Capital Market
BREZ,Breeze Holdings Acquisition Corp. - Common Stock,NASDAQ Capital Market
BREZR,Breeze Holdings Acquisition Corp. - Right,NASDAQ Capital Market
BREZW,Breeze Holdings Acquisition Corp. - Warrant,NASDAQ Capital Market
BRFH,Barfresh Food Group Inc. - Common Stock,NASDAQ Capital Market
BRID,Bridgford Foods Corporation - Common Stock,NASDAQ Global MarketSM
BRKH,BurTech Acquisition Corp. - Class A Common Stock,NASDAQ Global MarketSM
BRKHU,BurTech Acquisition Corp. - Unit,NASDAQ Global MarketSM
BRKHW,BurTech Acquisition Corp. - Warrants,NASDAQ Global MarketSM
BRKL,\"Brookline Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
BRKR,Bruker Corporation - Common Stock,NASDAQ Global Select MarketSM
BRLS,Borealis Foods Inc. - Class A Common Shares,NASDAQ Capital Market
BRLSW,Borealis Foods Inc. - Warrant,NASDAQ Capital Market
BRLT,\"Brilliant Earth Group, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
BRNS,Barinthus Biotherapeutics plc - American Depositary Shares,NASDAQ Global MarketSM
BRNY,Burney U.S. Factor Rotation ETF,NASDAQ Global MarketSM
BROG,Brooge Energy Limited  - Ordinary Shares,NASDAQ Capital Market
BROGW,Brooge Energy Limited  - Warrant,NASDAQ Capital Market
BRRR,Valkyrie Bitcoin Fund,NASDAQ Global MarketSM
BRSH,Bruush Oral Care Inc. - Common Stock,NASDAQ Capital Market
BRSHW,Bruush Oral Care Inc. - Warrant,NASDAQ Capital Market
BRTR,BlackRock Total Return ETF,NASDAQ Global MarketSM
BRTX,\"BioRestorative Therapies, Inc. - Common Stock\",NASDAQ Capital Market
BRY,Berry Corporation (bry) - Common Stock,NASDAQ Global Select MarketSM
BRZE,\"Braze, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
BSBK,Bogota Financial Corp. - Common Stock,NASDAQ Capital Market
BSCO,Invesco BulletShares 2024 Corporate Bond ETF,NASDAQ Global MarketSM
BSCP,Invesco BulletShares 2025 Corporate Bond ETF,NASDAQ Global MarketSM
BSCQ,Invesco BulletShares 2026 Corporate Bond ETF,NASDAQ Global MarketSM
BSCR,Invesco BulletShares 2027 Corporate Bond ETF,NASDAQ Global MarketSM
BSCS,Invesco BulletShares 2028 Corporate Bond ETF,NASDAQ Global MarketSM
BSCT,Invesco BulletShares 2029 Corporate Bond ETF,NASDAQ Global MarketSM
BSCU,Invesco BulletShares 2030 Corporate Bond ETF,NASDAQ Global MarketSM
BSCV,Invesco BulletShares 2031 Corporate Bond ETF,NASDAQ Global MarketSM
BSCW,Invesco BulletShares 2032 Corporate Bond ETF,NASDAQ Global MarketSM
BSCX,Invesco BulletShares 2033 Corporate Bond ETF,NASDAQ Global MarketSM
BSET,\"Bassett Furniture Industries, Incorporated - Common Stock\",NASDAQ Global Select MarketSM
BSFC,Blue Star Foods Corp. - Common stock,NASDAQ Capital Market
BSGM,\"BioSig Technologies, Inc. - Common Stock\",NASDAQ Capital Market
BSJO,Invesco BulletShares 2024 High Yield Corporate Bond ETF,NASDAQ Global MarketSM
BSJP,Invesco BulletShares 2025 High Yield Corporate Bond ETF,NASDAQ Global MarketSM
BSJQ,Invesco BulletShares 2026 High Yield Corporate Bond ETF,NASDAQ Global MarketSM
BSJR,Invesco BulletShares 2027 High Yield Corporate Bond ETF,NASDAQ Global MarketSM
BSJS,Invesco BulletShares 2028 High Yield Corporate Bond ETF,NASDAQ Global MarketSM
BSJT,Invesco BulletShares 2029 High Yield Corporate Bond ETF,NASDAQ Global MarketSM
BSJU,Invesco BulletShares 2030 High Yield Corporate Bond ETF,NASDAQ Global MarketSM
BSJV,Invesco BulletShares 2031 High Yield Corporate Bond ETF,NASDAQ Global MarketSM
BSMO,Invesco BulletShares 2024 Municipal Bond ETF,NASDAQ Global MarketSM
BSMP,Invesco BulletShares 2025 Municipal Bond ETF,NASDAQ Global MarketSM
BSMQ,Invesco BulletShares 2026 Municipal Bond ETF,NASDAQ Global MarketSM
BSMR,Invesco BulletShares 2027 Municipal Bond ETF,NASDAQ Global MarketSM
BSMS,Invesco BulletShares 2028 Municipal Bond ETF,NASDAQ Global MarketSM
BSMT,Invesco BulletShares 2029 Municipal Bond ETF,NASDAQ Global MarketSM
BSMU,Invesco BulletShares 2030 Municipal Bond ETF,NASDAQ Global MarketSM
BSMV,Invesco BulletShares 2031 Municipal Bond ETF,NASDAQ Global MarketSM
BSMW,Invesco BulletShares 2032 Municipal Bond ETF,NASDAQ Global MarketSM
BSRR,Sierra Bancorp - Common Stock,NASDAQ Global Select MarketSM
BSSX,Invesco BulletShares 2033 Municipal Bond ETF,NASDAQ Global MarketSM
BSVN,Bank7 Corp. - Common stock,NASDAQ Global Select MarketSM
BSVO,EA Bridgeway Omni Small-Cap Value ETF,NASDAQ Global MarketSM
BSY,\"Bentley Systems, Incorporated - Class B Common Stock\",NASDAQ Global Select MarketSM
BTAI,\"BioXcel Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
BTBD,\"BT Brands, Inc. - Common Stock\",NASDAQ Capital Market
BTBDW,\"BT Brands, Inc. - Warrant\",NASDAQ Capital Market
BTBT,\"Bit Digital, Inc. - Ordinary Share\",NASDAQ Capital Market
BTCS,BTCS Inc. - Common Stock,NASDAQ Capital Market
BTCT,BTC Digital Ltd. - Ordinary Shares,NASDAQ Capital Market
BTCTW,BTC Digital Ltd. - Warrant,NASDAQ Capital Market
BTCY,\"Biotricity, Inc. - Common Stock\",NASDAQ Capital Market
BTDR,Bitdeer Technologies Group - Ordinary Shares,NASDAQ Capital Market
BTEC,Principal Healthcare Innovators ETF,NASDAQ Global MarketSM
BTF,Valkyrie Bitcoin and Ether Strategy ETF,NASDAQ Global MarketSM
BTFX,Valkyrie Bitcoin Futures Leveraged Strategy ETF,NASDAQ Global MarketSM
BTM,Bitcoin Depot Inc. - Class A Common Stock,NASDAQ Capital Market
BTMD,Biote Corp. - Class A common stock,NASDAQ Global MarketSM
BTMWW,Bitcoin Depot Inc. - Warrant,NASDAQ Capital Market
BTOC,Armlogi Holding Corp. - common stock,NASDAQ Global MarketSM
BTOG,Bit Origin Limited - Ordinary Shares,NASDAQ Capital Market
BTSG,\"BrightSpring Health Services, Inc. - Common Stock\",NASDAQ Global Select MarketSM
BTSGU,\"BrightSpring Health Services, Inc. - Tangible Equity Unit\",NASDAQ Global Select MarketSM
BUFC,AB Conservative Buffer ETF,NASDAQ Global MarketSM
BUG,Global X Cybersecurity ETF,NASDAQ Global MarketSM
BUJA,Bukit Jalil Global Acquisition 1 Ltd. - Ordinary Shares,NASDAQ Capital Market
BUJAR,Bukit Jalil Global Acquisition 1 Ltd. - Rights,NASDAQ Capital Market
BUJAU,Bukit Jalil Global Acquisition 1 Ltd. - Unit,NASDAQ Capital Market
BUJAW,Bukit Jalil Global Acquisition 1 Ltd. - Warrants,NASDAQ Capital Market
BULD,Pacer BlueStar Engineering the Future ETF,NASDAQ Global MarketSM
BUSE,First Busey Corporation - Common Stock,NASDAQ Global Select MarketSM
BVFL,\"BV Financial, Inc. - Common Stock\",NASDAQ Capital Market
BVS,Bioventus Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
BWAQ,Blue World Acquisition Corporation - Class A ordinary shares,NASDAQ Global MarketSM
BWAQR,Blue World Acquisition Corporation - Right,NASDAQ Global MarketSM
BWAQU,Blue World Acquisition Corporation - Unit,NASDAQ Global MarketSM
BWAQW,Blue World Acquisition Corporation - Warrant,NASDAQ Global MarketSM
BWAY,BrainsWay Ltd. - American Depositary Shares,NASDAQ Global MarketSM
BWB,\"Bridgewater Bancshares, Inc. - Common Stock\",NASDAQ Capital Market
BWBBP,\"Bridgewater Bancshares, Inc. - Depositary Shares, Each Representing a 1/100th Interest in a Share of 5.875% Non-Cumulative Perpetual Preferred Stock, Series A\",NASDAQ Capital Market
BWEN,\"Broadwind, Inc. - Common Stock\",NASDAQ Capital Market
BWFG,\"Bankwell Financial Group, Inc. - Common Stock\",NASDAQ Global MarketSM
BWIN,\"The Baldwin Insurance Group, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
BWMN,Bowman Consulting Group Ltd. - Common Stock,NASDAQ Global MarketSM
BWMX,\"Betterware de Mexico, S.A.P.I. de C.V. - Ordinary Shares\",NASDAQ Capital Market
BYFC,Broadway Financial Corporation - Class A Common Stock,NASDAQ Capital Market
BYND,\"Beyond Meat, Inc. - Common stock\",NASDAQ Global Select MarketSM
BYNO,byNordic Acquisition Corporation - Class A Common Stock,NASDAQ Global MarketSM
BYNOU,byNordic Acquisition Corporation - Units,NASDAQ Global MarketSM
BYNOW,byNordic Acquisition Corporation - Warrant,NASDAQ Global MarketSM
BYRN,\"Byrna Technologies, Inc. - Common Stock\",NASDAQ Capital Market
BYSI,\"BeyondSpring, Inc. - Ordinary Shares\",NASDAQ Capital Market
BYU,\"BAIYU Holdings, Inc. - Common Stock\",NASDAQ Capital Market
BZ,KANZHUN LIMITED - American Depository Shares,NASDAQ Global Select MarketSM
BZFD,\"BuzzFeed, Inc. - Class A Common Stock\",NASDAQ Capital Market
BZFDW,\"BuzzFeed, Inc. - Warrant\",NASDAQ Capital Market
BZUN,Baozun Inc. - American Depositary Shares,NASDAQ Global Select MarketSM
CA,Xtrackers California Municipal Bonds ETF,NASDAQ Global MarketSM
CAAS,\"China Automotive Systems, Inc. - Common Stock\",NASDAQ Capital Market
CABA,\"Cabaletta Bio, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CAC,Camden National Corporation - Common Stock,NASDAQ Global Select MarketSM
CACC,Credit Acceptance Corporation - Common Stock,NASDAQ Global Select MarketSM
CACG,ClearBridge All Cap Growth ESG ETF,NASDAQ Global MarketSM
CACO,Caravelle International Group - Ordinary Shares,NASDAQ Capital Market
CADL,\"Candel Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
CAFG,Pacer US Small Cap Cash Cows Growth Leaders ETF,NASDAQ Global MarketSM
CAKE,The Cheesecake Factory Incorporated - Common Stock,NASDAQ Global Select MarketSM
CALB,California BanCorp - Common Stock,NASDAQ Global Select MarketSM
CALC,\"CalciMedica, Inc. - Common Stock\",NASDAQ Capital Market
CALM,\"Cal-Maine Foods, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CALT,Calliditas Therapeutics AB - American Depositary Shares,NASDAQ Global Select MarketSM
CALY,BlackRock Short-Term California Muni Bond ETF,NASDAQ Global MarketSM
CAMP,CalAmp Corp. - Common Stock,NASDAQ Global Select MarketSM
CAMT,Camtek Ltd. - Ordinary Shares,NASDAQ Global MarketSM
CAN,Canaan Inc. - American Depositary Shares,NASDAQ Global MarketSM
CANC,Tema Oncology ETF,NASDAQ Global MarketSM
CANQ,Calamos Alternative Nasdaq & Bond ETF,NASDAQ Global MarketSM
CAPR,\"Capricor Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
CAPT,Captivision Inc. - Ordinary Shares,NASDAQ Global MarketSM
CAPTW,Captivision Inc. - Warrant,NASDAQ Capital Market
CAR,\"Avis Budget Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CARA,\"Cara Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
CARE,\"Carter Bankshares, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CARG,\"CarGurus, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
CARM,\"Carisma Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
CART,Maplebear Inc. - Common Stock,NASDAQ Global Select MarketSM
CARV,\"Carver Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
CARZ,First Trust S-Network Electric & Future Vehicle Ecosystem ETF,NASDAQ Global MarketSM
CASH,\"Pathward Financial, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CASI,\"CASI Pharmaceuticals, Inc. - Ordinary Shares\",NASDAQ Capital Market
CASS,\"Cass Information Systems, Inc - Common Stock\",NASDAQ Global Select MarketSM
CASY,\"Caseys General Stores, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CATC,Cambridge Bancorp - Common Stock,NASDAQ Capital Market
CATH,Global X S&P 500 Catholic Values ETF,NASDAQ Global MarketSM
CATY,Cathay General Bancorp - Common Stock,NASDAQ Global Select MarketSM
CAUD,\"Collective Audience, Inc. - Common Stock\",NASDAQ Global MarketSM
CBAN,\"Colony Bankcorp, Inc. - Common Stock\",NASDAQ Global MarketSM
CBAT,\"CBAK Energy Technology, Inc. - Common Stock\",NASDAQ Capital Market
CBFV,\"CB Financial Services, Inc. - Common Stock\",NASDAQ Global MarketSM
CBNK,\"Capital Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CBRG,Chain Bridge I - Class A Ordinary Shares,NASDAQ Capital Market
CBRGU,Chain Bridge I - Units,NASDAQ Capital Market
CBRL,\"Cracker Barrel Old Country Store, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CBSH,\"Commerce Bancshares, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CBUS,\"Cibus, Inc. - Class A Common Stock\",NASDAQ Capital Market
CCAP,\"Crescent Capital BDC, Inc. - Common Stock\",NASDAQ Global MarketSM
CCB,Coastal Financial Corporation - Common Stock,NASDAQ Global Select MarketSM
CCBG,Capital City Bank Group - Common Stock,NASDAQ Global Select MarketSM
CCCC,\"C4 Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CCCS,CCC Intelligent Solutions Holdings Inc. - Common Stock,NASDAQ Global Select MarketSM
CCD,Calamos Dynamic Convertible & Income Fund - Closed End Fund,NASDAQ Global Select MarketSM
CCEP,Coca-Cola Europacific Partners plc - Ordinary Shares,NASDAQ Global Select MarketSM
CCG,Cheche Group Inc. - Class A Ordinary Shares,NASDAQ Capital Market
CCGWW,Cheche Group Inc. - Warrant,NASDAQ Capital Market
CCIXU,Churchill Capital Corp IX - Unit,NASDAQ Global MarketSM
CCLD,\"CareCloud, Inc. - Common Stock\",NASDAQ Global MarketSM
CCLDO,\"CareCloud, Inc. - 8.75% Series B Cumulative Redeemable Perpetual Preferred Stock\",NASDAQ Global MarketSM
CCLDP,\"CareCloud, Inc. - 11% Series A Cumulative Redeemable Perpetual Preferred Stock\",NASDAQ Global MarketSM
CCNE,CNB Financial Corporation - Common Stock,NASDAQ Global Select MarketSM
CCNEP,\"CNB Financial Corporation - Depositary shares, each representing a 1/40th ownership interest in a share of 7.125% Series A Fixed- Rate Non-Cumulative Perpetual Preferred Stock\",NASDAQ Global Select MarketSM
CCOI,\"Cogent Communications Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CCRN,\"Cross Country Healthcare, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CCSB,Carbon Collective Short Duration Green Bond ETF,NASDAQ Global MarketSM
CCSI,\"Consensus Cloud Solutions, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CCSO,Carbon Collective Climate Solutions U.S. Equity ETF,NASDAQ Global MarketSM
CCTG,CCSC Technology International Holdings Limited - Ordinary Shares,NASDAQ Capital Market
CCTS,Cactus Acquisition Corp. 1 Limited - Class A Ordinary Share,NASDAQ Global MarketSM
CCTSU,Cactus Acquisition Corp. 1 Limited - Unit,NASDAQ Global MarketSM
CCTSW,Cactus Acquisition Corp. 1 Limited - Warrant,NASDAQ Global MarketSM
CDAQ,Compass Digital Acquisition Corp. - Class A Ordinary Shares,NASDAQ Global MarketSM
CDAQU,Compass Digital Acquisition Corp. - Unit,NASDAQ Global MarketSM
CDAQW,Compass Digital Acquisition Corp. - Warrant,NASDAQ Global MarketSM
CDC,VictoryShares US EQ Income Enhanced Volatility Wtd ETF,NASDAQ Global MarketSM
CDIO,Cardio Diagnostics Holdings Inc. - Common stock,NASDAQ Capital Market
CDIOW,Cardio Diagnostics Holdings Inc. - Warrant,NASDAQ Capital Market
CDL,VictoryShares US Large Cap High Div Volatility Wtd ETF,NASDAQ Global MarketSM
CDLX,\"Cardlytics, Inc. - Common Stock\",NASDAQ Global MarketSM
CDMO,\"Avid Bioservices, Inc. - Common Stock\",NASDAQ Capital Market
CDNA,\"CareDx, Inc. - Common Stock\",NASDAQ Global MarketSM
CDNS,\"Cadence Design Systems, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CDRO,\"Codere Online Luxembourg, S.A. - Ordinary Shares\",NASDAQ Capital Market
CDROW,\"Codere Online Luxembourg, S.A. - Warrants\",NASDAQ Capital Market
CDT,Conduit Pharmaceuticals Inc.  - Common Stock,NASDAQ Global MarketSM
CDTG,CDT Environmental Technology Investment Holdings Limited - ordinary shares,NASDAQ Capital Market
CDTTW,Conduit Pharmaceuticals Inc.  - Warrant,NASDAQ Capital Market
CDTX,\"Cidara Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
CDW,CDW Corporation - Common Stock,NASDAQ Global Select MarketSM
CDXC,ChromaDex Corporation - Common Stock,NASDAQ Capital Market
CDXS,\"Codexis, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CDZI,\"Cadiz, Inc. - Common Stock\",NASDAQ Global MarketSM
CDZIP,\"Cadiz, Inc. - Depositary Shares\",NASDAQ Global MarketSM
CEAD,CEA Industries Inc. - Common Stock,NASDAQ Capital Market
CEADW,CEA Industries Inc. - Warrant,NASDAQ Capital Market
CECO,CECO Environmental Corp. - Common Stock,NASDAQ Global Select MarketSM
CEFA,Global X S&P Catholic Values Developed ex-U.S. ETF,NASDAQ Global MarketSM
CEG,Constellation Energy Corporation - Common Stock,NASDAQ Global Select MarketSM
CELC,Celcuity Inc. - Common Stock,NASDAQ Capital Market
CELH,\"Celsius Holdings, Inc. - Common Stock\",NASDAQ Capital Market
CELU,Celularity Inc. - Class A Common Stock,NASDAQ Capital Market
CELUW,Celularity Inc. - Warrant,NASDAQ Capital Market
CELZ,\"Creative Medical Technology Holdings, Inc. - Common Stock\",NASDAQ Capital Market
CENN,Cenntro Inc. - Common Stock,NASDAQ Capital Market
CENT,Central Garden & Pet Company - Common Stock,NASDAQ Global Select MarketSM
CENTA,Central Garden & Pet Company - Class A Common Stock Nonvoting,NASDAQ Global Select MarketSM
CENX,Century Aluminum Company - Common Stock,NASDAQ Global Select MarketSM
CERE,\"Cerevel Therapeutics Holdings, Inc. - Common Stock\",NASDAQ Capital Market
CERO,\"CERo Therapeutics Holdings, Inc. - Common Stock\",NASDAQ Global MarketSM
CEROW,\"CERo Therapeutics Holdings, Inc. - Warrants\",NASDAQ Capital Market
CERS,Cerus Corporation - Common Stock,NASDAQ Global MarketSM
CERT,\"Certara, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CETU,Cetus Capital Acquisition Corp. - Class A Common Stock,NASDAQ Capital Market
CETUR,Cetus Capital Acquisition Corp. - Right to receive 1/6 of one share of Class A Common Stock,NASDAQ Capital Market
CETUU,Cetus Capital Acquisition Corp. - Unit,NASDAQ Capital Market
CETUW,Cetus Capital Acquisition Corp. - Warrant,NASDAQ Capital Market
CETX,Cemtrex Inc. - Common Stock,NASDAQ Capital Market
CETY,\"Clean Energy Technologies, Inc. - Common Stock\",NASDAQ Capital Market
CEVA,\"CEVA, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CFA,VictoryShares US 500 Volatility Wtd ETF,NASDAQ Global MarketSM
CFB,\"CrossFirst Bankshares, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CFBK,CF Bankshares Inc. - Common Stock,NASDAQ Capital Market
CFFI,C&F Financial Corporation - Common Stock,NASDAQ Global Select MarketSM
CFFN,\"Capitol Federal Financial, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CFFS,CF Acquisition Corp. VII - Class A Common Stock,NASDAQ Capital Market
CFFSU,CF Acquisition Corp. VII - Unit,NASDAQ Global MarketSM
CFFSW,CF Acquisition Corp. VII - Warrant,NASDAQ Capital Market
CFLT,\"Confluent, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
CFO,VictoryShares US 500 Enhanced Volatility Wtd ETF,NASDAQ Global MarketSM
CFSB,\"CFSB Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
CG,The Carlyle Group Inc. - Common Stock,NASDAQ Global Select MarketSM
CGABL,The Carlyle Group Inc. - 4.625% Subordinated Notes due 2061,NASDAQ Global Select MarketSM
CGBD,\"Carlyle Secured Lending, Inc. - Closed End Fund\",NASDAQ Global Select MarketSM
CGBDL,\"Carlyle Secured Lending, Inc. - 8.20% Notes due 2028\",NASDAQ Global MarketSM
CGC,Canopy Growth Corporation - Common Shares,NASDAQ Global Select MarketSM
CGEM,\"Cullinan Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CGEN,Compugen Ltd. - Ordinary Shares,NASDAQ Capital Market
CGNT,Cognyte Software Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
CGNX,Cognex Corporation - Common Stock,NASDAQ Global Select MarketSM
CGO,Calamos Global Total Return Fund - Closed End Fund,NASDAQ Global Select MarketSM
CGON,\"CG Oncology, Inc. - Common stock\",NASDAQ Global Select MarketSM
CGTX,\"Cognition Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
CHCI,\"Comstock Holding Companies, Inc. - Class A Common Stock\",NASDAQ Capital Market
CHCO,City Holding Company - Common Stock,NASDAQ Global Select MarketSM
CHDN,\"Churchill Downs, Incorporated - Common Stock\",NASDAQ Global Select MarketSM
CHEF,\"The Chefs' Warehouse, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CHEK,Check-Cap Ltd. - Ordinary Share,NASDAQ Capital Market
CHI,Calamos Convertible Opportunities and Income Fund - Closed End Fund,NASDAQ Global Select MarketSM
CHK,Chesapeake Energy Corporation - Common Stock,NASDAQ Global Select MarketSM
CHKEL,Chesapeake Energy Corporation - Class C Warrants,NASDAQ Global Select MarketSM
CHKEW,Chesapeake Energy Corporation - Class A Warrants,NASDAQ Capital Market
CHKEZ,Chesapeake Energy Corporation - Class B Warrants,NASDAQ Capital Market
CHKP,Check Point Software Technologies Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
CHMG,Chemung Financial Corp  - Common Stock,NASDAQ Global Select MarketSM
CHNR,\"China Natural Resources, Inc. - Common Shares\",NASDAQ Capital Market
CHPS,Xtrackers Semiconductor Select Equity ETF,NASDAQ Global MarketSM
CHR,\"Cheer Holding, Inc.  - Ordinary Share\",NASDAQ Capital Market
CHRD,Chord Energy Corporation - Common Stock,NASDAQ Global Select MarketSM
CHRS,\"Coherus BioSciences, Inc. - Common Stock\",NASDAQ Global MarketSM
CHRW,\"C.H. Robinson Worldwide, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CHSCL,\"CHS Inc - Class B Cumulative Redeemable Preferred Stock, Series 4\",NASDAQ Global Select MarketSM
CHSCM,\"CHS Inc - Class B Reset Rate Cumulative Redeemable Preferred Stock, Series 3\",NASDAQ Global Select MarketSM
CHSCN,CHS Inc - Preferred Class B Series 2 Reset Rate,NASDAQ Global Select MarketSM
CHSCO,CHS Inc - Class B Cumulative Redeemable Preferred Stock,NASDAQ Global Select MarketSM
CHSCP,CHS Inc - 8%  Cumulative Redeemable Preferred Stock,NASDAQ Global Select MarketSM
CHSN,Chanson International Holding - Class A Ordinary Shares,NASDAQ Capital Market
CHTR,\"Charter Communications, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
CHUY,\"Chuy's Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CHW,Calamos Global Dynamic Income Fund - Closed End Fund,NASDAQ Global Select MarketSM
CHX,ChampionX Corporation - Common Stock,NASDAQ Global Select MarketSM
CHY,Calamos Convertible and High Income Fund - Closed End Fund,NASDAQ Global Select MarketSM
CIBR,First Trust NASDAQ Cybersecurity ETF,NASDAQ Global MarketSM
CID,VictoryShares International High Div Volatility Wtd ETF,NASDAQ Global MarketSM
CIFR,Cipher Mining Inc. - Common Stock,NASDAQ Global Select MarketSM
CIFRW,Cipher Mining Inc. - Warrant,NASDAQ Global Select MarketSM
CIGI,Colliers International Group Inc.  - Subordinate Voting Shares,NASDAQ Global Select MarketSM
CIL,VictoryShares International Volatility Wtd ETF,NASDAQ Global MarketSM
CINF,Cincinnati Financial Corporation - Common Stock,NASDAQ Global Select MarketSM
CING,Cingulate Inc. - Common Stock,NASDAQ Capital Market
CINGW,Cingulate Inc. - Warrants,NASDAQ Capital Market
CISO,\"CISO Global, Inc. - Common Stock\",NASDAQ Capital Market
CISS,C3is Inc. - Common Stock,NASDAQ Capital Market
CITE,Cartica Acquisition Corp - Class A Ordinary Shares,NASDAQ Global MarketSM
CITEU,Cartica Acquisition Corp - Unit,NASDAQ Global MarketSM
CITEW,Cartica Acquisition Corp - Warrant,NASDAQ Global MarketSM
CIVB,\"Civista Bancshares, Inc.  - Common Stock\",NASDAQ Capital Market
CIZ,VictoryShares Developed Enhanced Volatility Wtd ETF,NASDAQ Global MarketSM
CJET,\"Chijet Motor Company, Inc. - Ordinary Shares\",NASDAQ Global MarketSM
CJJD,\"China Jo-Jo Drugstores, Inc. - Ordinary Shares\",NASDAQ Capital Market
CKPT,\"Checkpoint Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
CLAR,Clarus Corporation - Common Stock,NASDAQ Global Select MarketSM
CLBK,\"Columbia Financial, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CLBT,Cellebrite DI Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
CLBTW,Cellebrite DI Ltd. - Warrants,NASDAQ Global Select MarketSM
CLDX,\"Celldex Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
CLEU,China Liberal Education Holdings Limited - Ordinary Shares,NASDAQ Capital Market
CLFD,\"Clearfield, Inc. - Common Stock\",NASDAQ Global MarketSM
CLGN,CollPlant Biotechnologies Ltd. - Ordinary Shares,NASDAQ Global MarketSM
CLIR,ClearSign Technologies Corporation - Common Stock,NASDAQ Capital Market
CLLS,Cellectis S.A. - American Depositary Shares,NASDAQ Global MarketSM
CLMB,\"Climb Global Solutions, Inc. - Common Stock\",NASDAQ Global MarketSM
CLMT,\"Calumet Specialty Products Partners, L.P. - Common units representing limited partner interests\",NASDAQ Global Select MarketSM
CLNE,Clean Energy Fuels Corp. - Common Stock,NASDAQ Global Select MarketSM
CLNN,Clene Inc. - Common Stock,NASDAQ Capital Market
CLNNW,Clene Inc. - Warrant,NASDAQ Capital Market
CLOA,BlackRock AAA CLO ETF,NASDAQ Global MarketSM
CLOD,Themes Cloud Computing ETF,NASDAQ Global MarketSM
CLOE,Clover Leaf Capital Corp. - Class A Common Stock,NASDAQ Capital Market
CLOER,Clover Leaf Capital Corp. - Rights,NASDAQ Capital Market
CLOEU,Clover Leaf Capital Corp. - Unit,NASDAQ Capital Market
CLOU,Global X Cloud Computing ETF,NASDAQ Global MarketSM
CLOV,\"Clover Health Investments, Corp.  - Class A Common stock\",NASDAQ Global Select MarketSM
CLPS,CLPS Incorporation - Common Stock,NASDAQ Global MarketSM
CLPT,ClearPoint Neuro Inc. - Common Stock,NASDAQ Capital Market
CLRB,\"Cellectar Biosciences, Inc. - Common Stock\",NASDAQ Capital Market
CLRC,ClimateRock - Class A Ordinary Shares,NASDAQ Global MarketSM
CLRCR,ClimateRock - Right,NASDAQ Global MarketSM
CLRCU,ClimateRock - Unit,NASDAQ Global MarketSM
CLRCW,ClimateRock - Warrant,NASDAQ Global MarketSM
CLRO,\"ClearOne, Inc. - Common Stock\",NASDAQ Capital Market
CLSD,\"Clearside Biomedical, Inc. - Common Stock\",NASDAQ Global MarketSM
CLSK,\"CleanSpark, Inc. - Common Stock\",NASDAQ Capital Market
CLSM,Cabana Target Leading Sector Moderate ETF,NASDAQ Global MarketSM
CLST,\"Catalyst Bancorp, Inc. - common stock\",NASDAQ Capital Market
CLWT,Euro Tech Holdings Company Limited - Ordinary Shares,NASDAQ Capital Market
CMAX,\"CareMax, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
CMAXW,\"CareMax, Inc. - Warrant\",NASDAQ Global Select MarketSM
CMBM,Cambium Networks Corporation - Ordinary Shares,NASDAQ Global MarketSM
CMCA,Capitalworks Emerging Markets Acquisition Corp - Class A Ordinary Shares,NASDAQ Global MarketSM
CMCAU,Capitalworks Emerging Markets Acquisition Corp - Unit,NASDAQ Global MarketSM
CMCAW,Capitalworks Emerging Markets Acquisition Corp - Warrant,NASDAQ Global MarketSM
CMCO,Columbus McKinnon Corporation - Common Stock,NASDAQ Global Select MarketSM
CMCSA,Comcast Corporation - Class A Common Stock,NASDAQ Global Select MarketSM
CMCT,Creative Media & Community Trust Corporation - Common Stock,NASDAQ Global MarketSM
CME,CME Group Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
CMLS,Cumulus Media Inc. - Class A Common Stock,NASDAQ Global MarketSM
CMMB,Chemomab Therapeutics Ltd.  - American Depositary Shares,NASDAQ Capital Market
CMND,Clearmind Medicine Inc. - Common Shares,NASDAQ Capital Market
CMPO,\"CompoSecure, Inc.  - Class A Common Stock\",NASDAQ Global MarketSM
CMPOW,\"CompoSecure, Inc.  - Warrant\",NASDAQ Global MarketSM
CMPR,Cimpress plc - Ordinary Shares,NASDAQ Global Select MarketSM
CMPS,COMPASS Pathways Plc - American Depository Shares,NASDAQ Global Select MarketSM
CMPX,\"Compass Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
CMRX,\"Chimerix, Inc. - Common Stock\",NASDAQ Global MarketSM
CMTL,Comtech Telecommunications Corp. - Common Stock,NASDAQ Global Select MarketSM
CNCR,Range Cancer Therapeutics ETF,NASDAQ Global MarketSM
CNDT,Conduent Incorporated - Common Stock,NASDAQ Global Select MarketSM
CNET,ZW Data Action Technologies Inc. - Common Stock,NASDAQ Capital Market
CNEY,CN Energy Group Inc. - Class A Ordinary Shares,NASDAQ Capital Market
CNFR,\"Conifer Holdings, Inc. - Common Stock\",NASDAQ Capital Market
CNFRZ,\"Conifer Holdings, Inc. - 9.75% Senior Unsecured Notes due 2028\",NASDAQ Global MarketSM
CNGL,Canna-Global Acquisition Corp - Class A Common Stock,NASDAQ Capital Market
CNGLU,Canna-Global Acquisition Corp - Unit,NASDAQ Capital Market
CNGLW,Canna-Global Acquisition Corp - Warrant,NASDAQ Capital Market
CNOB,\"ConnectOne Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CNOBP,\"ConnectOne Bancorp, Inc. - Depositary Shares (each representing a 1/40th interest in a share of 5.25% Fixed-Rate Reset Non-Cumulative Perpetual Preferred Stock, Series A, par value $0.00 per share)\",NASDAQ Global Select MarketSM
CNSL,\"Consolidated Communications Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CNSP,\"CNS Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
CNTA,Centessa Pharmaceuticals plc - American Depositary Shares,NASDAQ Global Select MarketSM
CNTB,Connect Biopharma Holdings Limited - American Depositary Shares,NASDAQ Global MarketSM
CNTG,Centogene N.V. - Common Shares,NASDAQ Global MarketSM
CNTX,Context Therapeutics Inc. - Common Stock,NASDAQ Capital Market
CNTY,\"Century Casinos, Inc. - Common Stock\",NASDAQ Capital Market
CNVS,Cineverse Corp. - Class A Common Stock,NASDAQ Capital Market
CNXC,Concentrix Corporation - Common Stock,NASDAQ Global Select MarketSM
CNXN,\"PC Connection, Inc. - Common Stock\",NASDAQ Global Select MarketSM
COCH,\"Envoy Medical, Inc. - Class A Common Stock\",NASDAQ Capital Market
COCHW,\"Envoy Medical, Inc. - Warrant\",NASDAQ Capital Market
COCO,\"The Vita Coco Company, Inc. - Common Stock\",NASDAQ Global Select MarketSM
COCP,\"Cocrystal Pharma, Inc. - Common Stock\",NASDAQ Capital Market
CODA,\"Coda Octopus Group, Inc. - Common stock\",NASDAQ Capital Market
CODX,\"Co-Diagnostics, Inc. - Common Stock\",NASDAQ Capital Market
COEP,\"Coeptis Therapeutics Holdings, Inc. - Common Stock\",NASDAQ Capital Market
COEPW,\"Coeptis Therapeutics Holdings, Inc. - Warrants\",NASDAQ Capital Market
COFS,\"ChoiceOne Financial Services, Inc. - Common Stock\",NASDAQ Capital Market
COGT,\"Cogent Biosciences, Inc. - Common Stock\",NASDAQ Global Select MarketSM
COHU,\"Cohu, Inc. - Common Stock\",NASDAQ Global Select MarketSM
COIN,\"Coinbase Global, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
COKE,\"Coca-Cola Consolidated, Inc. - Common Stock\",NASDAQ Global Select MarketSM
COLB,\"Columbia Banking System, Inc. - Common Stock\",NASDAQ Global Select MarketSM
COLL,\"Collegium Pharmaceutical, Inc. - Common Stock\",NASDAQ Global Select MarketSM
COLM,Columbia Sportswear Company - Common Stock,NASDAQ Global Select MarketSM
COMM,\"CommScope Holding Company, Inc. - Common Stock\",NASDAQ Global Select MarketSM
COMT,iShares GSCI Commodity Dynamic Roll Strategy ETF,NASDAQ Global MarketSM
CONL,GraniteShares 2x Long COIN Daily ETF,NASDAQ Global MarketSM
CONN,\"Conn's, Inc. - Common Stock\",NASDAQ Global Select MarketSM
COO,\"The Cooper Companies, Inc.  - Common Stock\",NASDAQ Global Select MarketSM
COOL,Corner Growth Acquisition Corp. - Class A Ordinary Shares,NASDAQ Capital Market
COOLU,Corner Growth Acquisition Corp. - Unit,NASDAQ Capital Market
COOLW,Corner Growth Acquisition Corp. - Warrant,NASDAQ Capital Market
COOP,Mr. Cooper Group Inc. - Common Stock,NASDAQ Capital Market
COOT,Australian Oilseeds Holdings Limited - Ordinary Shares,NASDAQ Global MarketSM
COOTW,Australian Oilseeds Holdings Limited - Warrant,NASDAQ Global MarketSM
COPJ,Sprott Junior Copper Miners ETF,NASDAQ Global MarketSM
COPP,Sprott Copper Miners ETF,NASDAQ Global MarketSM
CORT,Corcept Therapeutics Incorporated - Common Stock,NASDAQ Capital Market
CORZ,\"Core Scientific, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CORZW,\"Core Scientific, Inc. - Tranche 1 Warrants\",NASDAQ Global Select MarketSM
CORZZ,\"Core Scientific, Inc. - Tranche 2 Warrants\",NASDAQ Global Select MarketSM
COSM,Cosmos Health Inc. - Common Stock,NASDAQ Capital Market
COST,Costco Wholesale Corporation - Common Stock,NASDAQ Global Select MarketSM
COWG,Pacer US Large Cap Cash Cows Growth Leaders ETF,NASDAQ Global MarketSM
COWS,Amplify Cash Flow Dividend Leaders ETF,NASDAQ Global MarketSM
COYA,\"Coya Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
CPBI,\"Central Plains Bancshares, Inc. - Common Stock\",NASDAQ Capital Market
CPHC,Canterbury Park Holding Corporation - Common Stock,NASDAQ Global MarketSM
CPIX,Cumberland Pharmaceuticals Inc. - Common Stock,NASDAQ Global Select MarketSM
CPLP,Capital Product Partners L.P. - Common Units representing limited partner interests,NASDAQ Global Select MarketSM
CPLS,AB Core Plus Bond ETF,NASDAQ Global MarketSM
CPOP,\"Pop Culture Group Co., Ltd - Class A Ordinary Shares\",NASDAQ Capital Market
CPRT,\"Copart, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CPRX,\"Catalyst Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
CPSH,CPS Technologies Corp. - Common Stock,NASDAQ Capital Market
CPSS,\"Consumer Portfolio Services, Inc. - Common Stock\",NASDAQ Global MarketSM
CPTN,\"Cepton, Inc. - Common Stock\",NASDAQ Capital Market
CPTNW,\"Cepton, Inc. - Warrant\",NASDAQ Capital Market
CPZ,Calamos Long/Short Equity & Dynamic Income Trust - Closed End Fund,NASDAQ Global Select MarketSM
CRAI,\"CRA International,Inc. - Common Stock\",NASDAQ Global Select MarketSM
CRBP,\"Corbus Pharmaceuticals Holdings, Inc. - Common Stock\",NASDAQ Capital Market
CRBU,\"Caribou Biosciences, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CRCT,\"Cricut, Inc. - Class A common stock\",NASDAQ Global Select MarketSM
CRDF,\"Cardiff Oncology, Inc. - Common Stock\",NASDAQ Capital Market
CRDL,Cardiol Therapeutics Inc. - Class A Common Shares,NASDAQ Capital Market
CRDO,Credo Technology Group Holding Ltd - Ordinary Shares,NASDAQ Global Select MarketSM
CREG,Smart Powerr Corp. - Common Stock,NASDAQ Capital Market
CRESW,Cresud S.A.C.I.F. y A. - Warrant,NASDAQ Capital Market
CRESY,\"Cresud S.A.C.I.F. y A. - American Depositary Shares, each representing ten shares of Common Stock\",NASDAQ Global Select MarketSM
CREV,Carbon Revolution Public Limited Company - Ordinary Shares,NASDAQ Global MarketSM
CREVW,Carbon Revolution Public Limited Company - Warrant,NASDAQ Capital Market
CREX,\"Creative Realities, Inc. - Common Stock\",NASDAQ Capital Market
CRGO,Freightos Limited - Ordinary shares,NASDAQ Capital Market
CRGOW,Freightos Limited - Warrants,NASDAQ Capital Market
CRGX,\"CARGO Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CRIS,\"Curis, Inc. - Common Stock\",NASDAQ Capital Market
CRKN,Crown Electrokinetics Corp. - Common Stock,NASDAQ Capital Market
CRMD,CorMedix Inc. - Common Stock,NASDAQ Global MarketSM
CRML,Critical Metals Corp. - Ordinary Shares,NASDAQ Global MarketSM
CRMLW,Critical Metals Corp. - Warrants,NASDAQ Capital Market
CRMT,\"America's Car-Mart, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CRNC,Cerence Inc. - Common Stock,NASDAQ Global Select MarketSM
CRNT,Ceragon Networks Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
CRNX,\"Crinetics Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CRON,Cronos Group Inc. - Common Share,NASDAQ Global MarketSM
CROX,\"Crocs, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CRSP,CRISPR Therapeutics AG - Common Shares,NASDAQ Global MarketSM
CRSR,\"Corsair Gaming, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CRTO,Criteo S.A. - American Depositary Shares,NASDAQ Global Select MarketSM
CRUS,\"Cirrus Logic, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CRVL,CorVel Corp. - Common Stock,NASDAQ Global Select MarketSM
CRVO,CervoMed Inc. - Common Stock,NASDAQ Capital Market
CRVS,\"Corvus Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global MarketSM
CRWD,\"CrowdStrike Holdings, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
CRWS,\"Crown Crafts, Inc. - Common Stock\",NASDAQ Capital Market
CSA,VictoryShares US Small Cap Volatility Wtd ETF,NASDAQ Global MarketSM
CSB,VictoryShares US Small Cap High Div Volatility Wtd ETF,NASDAQ Global MarketSM
CSBR,\"Champions Oncology, Inc. - Common Stock\",NASDAQ Capital Market
CSCO,\"Cisco Systems, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CSF,VictoryShares US Discovery Enhanced Volatility Wtd ETF,NASDAQ Global MarketSM
CSGP,\"CoStar Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CSGS,\"CSG Systems International, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CSIQ,Canadian Solar Inc. - Common Shares,NASDAQ Global Select MarketSM
CSLM,CSLM Acquisition Corp. - Class A Ordinary Share,NASDAQ Capital Market
CSLMR,CSLM Acquisition Corp. - Right,NASDAQ Capital Market
CSLMU,CSLM Acquisition Corp. - Unit,NASDAQ Capital Market
CSLMW,CSLM Acquisition Corp. - Warrant,NASDAQ Capital Market
CSLR,\"Complete Solaria, Inc. - Common Stock\",NASDAQ Global MarketSM
CSLRW,\"Complete Solaria, Inc. - Warrant\",NASDAQ Capital Market
CSPI,CSP Inc. - Common Stock,NASDAQ Global MarketSM
CSQ,Calamos Strategic Total Return Fund - Closed End Fund,NASDAQ Global Select MarketSM
CSSE,\"Chicken Soup for the Soul Entertainment, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
CSSEL,\"Chicken Soup for the Soul Entertainment, Inc. - Warrant\",NASDAQ Global MarketSM
CSSEN,\"Chicken Soup for the Soul Entertainment, Inc. - 9.50% Notes due 2025\",NASDAQ Global MarketSM
CSSEP,\"Chicken Soup for the Soul Entertainment, Inc. - 9.75% Series A Cumulative Redeemable Perpetual Preferred Stock\",NASDAQ Global MarketSM
CSTE,Caesarstone Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
CSTL,\"Castle Biosciences, Inc. - Common stock\",NASDAQ Global MarketSM
CSWC,Capital Southwest Corporation - Common Stock,NASDAQ Global Select MarketSM
CSWCZ,Capital Southwest Corporation - 7.75% Notes due 2028,NASDAQ Global Select MarketSM
CSWI,\"CSW Industrials, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CSX,CSX Corporation - Common Stock,NASDAQ Global Select MarketSM
CTAS,Cintas Corporation - Common Stock,NASDAQ Global Select MarketSM
CTBI,\"Community Trust Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CTCX,Carmell Corporation - Common Stock,NASDAQ Capital Market
CTCXW,Carmell Corporation - Warrant,NASDAQ Capital Market
CTEC,Global X CleanTech ETF,NASDAQ Global MarketSM
CTHR,Charles & Colvard Ltd. - Common Stock,NASDAQ Capital Market
CTKB,\"Cytek Biosciences, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CTLP,\"Cantaloupe, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CTMX,\"CytomX Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CTNM,\"Contineum Therapeutics, Inc. - Common stock\",NASDAQ Global Select MarketSM
CTNT,Cheetah Net Supply Chain Service Inc. - Class A Common Stock,NASDAQ Capital Market
CTRM,Castor Maritime Inc. - Common Shares,NASDAQ Capital Market
CTRN,\"Citi Trends, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CTSH,Cognizant Technology Solutions Corporation - Class A Common Stock,NASDAQ Global Select MarketSM
CTSO,Cytosorbents Corporation - Common Stock,NASDAQ Capital Market
CTXR,\"Citius Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
CUBA,\"The Herzfeld Caribbean Basin Fund, Inc. - Closed End Fund\",NASDAQ Capital Market
CUE,\"Cue Biopharma, Inc. - Common Stock\",NASDAQ Capital Market
CULL,\"Cullman Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
CURI,CuriosityStream Inc.  - Class A Common Stock,NASDAQ Capital Market
CURIW,CuriosityStream Inc.  - Warrant,NASDAQ Capital Market
CUTR,\"Cutera, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CVAC,CureVac N.V. - Ordinary Shares,NASDAQ Global MarketSM
CVBF,CVB Financial Corporation - Common Stock,NASDAQ Global Select MarketSM
CVCO,\"Cavco Industries, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CVGI,\"Commercial Vehicle Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CVGW,\"Calavo Growers, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CVII,Churchill Capital Corp VII - Class A Common Stock,NASDAQ Global MarketSM
CVIIU,Churchill Capital Corp VII - Unit,NASDAQ Global MarketSM
CVIIW,Churchill Capital Corp VII - Warrant,NASDAQ Global MarketSM
CVKD,\"Cadrenal Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
CVLG,\"Covenant Logistics Group, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
CVLT,\"Commvault Systems, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CVLY,\"Codorus Valley Bancorp, Inc - Common Stock\",NASDAQ Global MarketSM
CVRX,\"CVRx, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CVV,CVD Equipment Corporation - Common Stock,NASDAQ Capital Market
CWBC,Community West Bancshares - Common Stock,NASDAQ Capital Market
CWCO,Consolidated Water Co. Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
CWD,CaliberCos Inc. - Class A Common Stock,NASDAQ Capital Market
CWST,\"Casella Waste Systems, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
CXAI,CXApp Inc. - Class A Common Stock,NASDAQ Capital Market
CXAIW,CXApp Inc. - Warrant,NASDAQ Capital Market
CXDO,\"Crexendo, Inc. - Common Stock\",NASDAQ Capital Market
CXSE,WisdomTree China ex-State-Owned Enterprises Fund,NASDAQ Global MarketSM
CYBR,CyberArk Software Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
CYCC,\"Cyclacel Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
CYCCP,\"Cyclacel Pharmaceuticals, Inc. - 6% Convertible Preferred Stock\",NASDAQ Capital Market
CYCN,\"Cyclerion Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
CYN,Cyngn Inc. - Common stock,NASDAQ Capital Market
CYRX,\"CryoPort, Inc. - Common Stock\",NASDAQ Capital Market
CYTH,\"Cyclo Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
CYTHW,\"Cyclo Therapeutics, Inc. - Warrant\",NASDAQ Capital Market
CYTK,\"Cytokinetics, Incorporated - Common Stock\",NASDAQ Global Select MarketSM
CYTO,Altamira Therapeutics Ltd. - Common Shares,NASDAQ Capital Market
CZAR,Themes Natural Monopoly ETF,NASDAQ Global MarketSM
CZFS,\"Citizens Financial Services, Inc. - Common Stock\",NASDAQ Capital Market
CZNC,Citizens & Northern Corp - Common Stock,NASDAQ Capital Market
CZR,\"Caesars Entertainment, Inc. - Common Stock\",NASDAQ Global Select MarketSM
CZWI,\"Citizens Community Bancorp, Inc. - Common Stock\",NASDAQ Global MarketSM
DADA,Dada Nexus Limited - American Depositary Shares,NASDAQ Global Select MarketSM
DAIO,Data I/O Corporation - Common Stock,NASDAQ Capital Market
DAKT,\"Daktronics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
DALI,First Trust Dorsey Wright DALI 1 ETF,NASDAQ Global MarketSM
DALN,DallasNews Corporation - Series A Common Stock,NASDAQ Capital Market
DAPP,VanEck Digital Transformation ETF,NASDAQ Global MarketSM
DARE,\"Dare Bioscience, Inc. - Common Stock\",NASDAQ Capital Market
DASH,\"DoorDash, Inc. - Common Stock\",NASDAQ Global Select MarketSM
DATS,\"DatChat, Inc. - Common Stock\",NASDAQ Capital Market
DATSW,\"DatChat, Inc. - Series A Warrant\",NASDAQ Capital Market
DAVE,Dave Inc.  - Class A Common Stock,NASDAQ Global MarketSM
DAVEW,Dave Inc.  - Warrants,NASDAQ Global MarketSM
DAWN,\"Day One Biopharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
DAX,Global X DAX Germany ETF,NASDAQ Global MarketSM
DBGI,\"Digital Brands Group, Inc. - Common Stock\",NASDAQ Capital Market
DBGIW,\"Digital Brands Group, Inc. - Warrant\",NASDAQ Capital Market
DBVT,DBV Technologies S.A. - American Depositary Shares,NASDAQ Global Select MarketSM
DBX,\"Dropbox, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
DCBO,Docebo Inc. - Common Shares,NASDAQ Global Select MarketSM
DCGO,DocGo Inc. - Common Stock,NASDAQ Capital Market
DCOM,\"Dime Community Bancshares, Inc. - Common Stock\",NASDAQ Global Select MarketSM
DCOMP,\"Dime Community Bancshares, Inc. - Fixed-Rate Non-Cumulative Perpetual Preferred Stock, Series A\",NASDAQ Global Select MarketSM
DCPH,\"Deciphera Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
DCTH,\"Delcath Systems, Inc. - Common Stock\",NASDAQ Capital Market
DDI,\"DoubleDown Interactive Co., Ltd. - American Depository Shares\",NASDAQ Global Select MarketSM
DDIV,First Trust Dorsey Wright Momentum & Dividend ETF,NASDAQ Global MarketSM
DDOG,\"Datadog, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
DECA,Denali Capital Acquisition Corp. - Class A Ordinary Shares,NASDAQ Global MarketSM
DECAU,Denali Capital Acquisition Corp. - Unit,NASDAQ Global MarketSM
DECAW,Denali Capital Acquisition Corp. - Warrant,NASDAQ Global MarketSM
DEMZ,Democratic Large Cap Core ETF,NASDAQ Global MarketSM
DENN,Denny's Corporation - Common Stock,NASDAQ Capital Market
DERM,Journey Medical Corporation - Common Stock,NASDAQ Capital Market
DFGP,Dimensional Global Core Plus Fixed Income ETF,NASDAQ Global MarketSM
DFGX,Dimensional Global ex US Core Fixed Income ETF,NASDAQ Global MarketSM
DFLI,Dragonfly Energy Holdings Corp - Common Stock,NASDAQ Global MarketSM
DFLIW,Dragonfly Energy Holdings Corp - Warrant,NASDAQ Capital Market
DGCB,Dimensional Global Credit ETF,NASDAQ Global MarketSM
DGHI,Digihost Technology Inc. - Common Subordinate Voting Shares,NASDAQ Capital Market
DGICA,\"Donegal Group, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
DGICB,\"Donegal Group, Inc. - Class B Common Stock\",NASDAQ Global Select MarketSM
DGII,Digi International Inc. - Common Stock,NASDAQ Global Select MarketSM
DGLY,\"Digital Ally, Inc. - Common Stock\",NASDAQ Capital Market
DGRE,WisdomTree Emerging Markets Quality Dividend Growth Fund,NASDAQ Global MarketSM
DGRS,WisdomTree U.S. SmallCap Quality Dividend Growth Fund,NASDAQ Global MarketSM
DGRW,WisdomTree U.S. Quality Dividend Growth Fund,NASDAQ Global MarketSM
DH,Definitive Healthcare Corp. - Class A Common Stock,NASDAQ Global Select MarketSM
DHAC,Digital Health Acquisition Corp. - Common Stock,NASDAQ Capital Market
DHACU,Digital Health Acquisition Corp. - Unit,NASDAQ Capital Market
DHACW,Digital Health Acquisition Corp. - Warrant,NASDAQ Capital Market
DHAI,\"DIH Holding US, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
DHAIW,\"DIH Holding US, Inc. - Warrant\",NASDAQ Capital Market
DHC,Diversified Healthcare Trust  - Common Shares of Beneficial Interest,NASDAQ Global Select MarketSM
DHCNI,Diversified Healthcare Trust  - 5.625% Senior Notes due 2042,NASDAQ Global Select MarketSM
DHCNL,Diversified Healthcare Trust  - 6.25% Senior Notes Due 2046,NASDAQ Global Select MarketSM
DHIL,\"Diamond Hill Investment Group, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
DIBS,\"1stdibs.com, Inc. - Common Stock\",NASDAQ Global MarketSM
DIOD,Diodes Incorporated - Common Stock,NASDAQ Global Select MarketSM
DIST,Distoken Acquisition Corporation - Ordinary Shares,NASDAQ Global MarketSM
DISTR,Distoken Acquisition Corporation - Right,NASDAQ Global MarketSM
DISTW,Distoken Acquisition Corporation - Warrant,NASDAQ Global MarketSM
DIVD,Altrius Global Dividend ETF,NASDAQ Global MarketSM
DJCO,Daily Journal Corp. (S.C.) - Common Stock,NASDAQ Capital Market
DJT,Trump Media & Technology Group Corp. - Common Stock,NASDAQ Global MarketSM
DJTWW,Trump Media & Technology Group Corp. - Warrants,NASDAQ Global MarketSM
DKNG,DraftKings Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
DLHC,DLH Holdings Corp. - Common Stock,NASDAQ Capital Market
DLO,DLocal Limited - Class A Common Shares,NASDAQ Global Select MarketSM
DLPN,\"Dolphin Entertainment, Inc. - Common Stock\",NASDAQ Capital Market
DLTH,Duluth Holdings Inc. - Class B Common Stock,NASDAQ Global Select MarketSM
DLTR,\"Dollar Tree, Inc. - Common Stock\",NASDAQ Global Select MarketSM
DMAC,DiaMedica Therapeutics Inc. - Common Stock,NASDAQ Capital Market
DMAT,Global X Disruptive Materials ETF,NASDAQ Global MarketSM
DMLP,\"Dorchester Minerals, L.P. - Common Units Representing Limited Partnership Interests\",NASDAQ Global Select MarketSM
DMRC,Digimarc Corporation - Common Stock,NASDAQ Global Select MarketSM
DMTK,\"DermTech, Inc. - Common Stock\",NASDAQ Capital Market
DMXF,iShares ESG Advanced MSCI EAFE ETF,NASDAQ Global MarketSM
DNLI,Denali Therapeutics Inc. - Common Stock,NASDAQ Global Select MarketSM
DNTH,\"Dianthus Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
DNUT,\"Krispy Kreme, Inc. - Common Stock\",NASDAQ Global Select MarketSM
DOCU,\"DocuSign, Inc. - Common Stock\",NASDAQ Global Select MarketSM
DOGZ,Dogness (International) Corporation - Class A Common Stock,NASDAQ Capital Market
DOMH,Dominari Holdings Inc. - Common Stock,NASDAQ Capital Market
DOMO,\"Domo, Inc. - Class B Common Stock\",NASDAQ Global MarketSM
DOOO,BRP Inc. - Common Subordinate Voting Shares,NASDAQ Global Select MarketSM
DORM,\"Dorman Products, Inc. - Common Stock\",NASDAQ Global Select MarketSM
DOX,Amdocs Limited - Ordinary Shares,NASDAQ Global Select MarketSM
DOYU,DouYu International Holdings Limited - American Depositary Shares,NASDAQ Global Select MarketSM
DPCS,DP Cap Acquisition Corp I - Class A Ordinary Shares,NASDAQ Capital Market
DPCSU,DP Cap Acquisition Corp I - Unit,NASDAQ Capital Market
DPCSW,DP Cap Acquisition Corp I - Warrants,NASDAQ Capital Market
DPRO,Draganfly Inc. - Common Shares,NASDAQ Capital Market
DRCT,\"Direct Digital Holdings, Inc. - Class A Common Stock\",NASDAQ Capital Market
DRIO,DarioHealth Corp. - Common Stock,NASDAQ Capital Market
DRIV,Global X Autonomous & Electric Vehicles ETF,NASDAQ Global MarketSM
DRMA,\"Dermata Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
DRMAW,\"Dermata Therapeutics, Inc. - Warrant\",NASDAQ Capital Market
DRRX,DURECT Corporation - Common Stock,NASDAQ Capital Market
DRS,\"Leonardo DRS, Inc. - Common Stock\",NASDAQ Global Select MarketSM
DRTS,Alpha Tau Medical Ltd. - Ordinary Shares,NASDAQ Capital Market
DRTSW,Alpha Tau Medical Ltd. - Warrant,NASDAQ Capital Market
DRUG,Bright Minds Biosciences Inc. - common stock,NASDAQ Capital Market
DRVN,Driven Brands Holdings Inc. - Common Stock,NASDAQ Global Select MarketSM
DSGN,\"Design Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
DSGR,\"Distribution Solutions Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
DSGX,The Descartes Systems Group Inc. - Common Stock,NASDAQ Global Select MarketSM
DSP,Viant Technology Inc. - common stock,NASDAQ Global Select MarketSM
DSWL,\"Deswell Industries, Inc. - Common Shares\",NASDAQ Global MarketSM
DTCK,Davis Commodities Limited - Ordinary Shares,NASDAQ Capital Market
DTCR,Global X Data Center & Digital Infrastructure ETF,NASDAQ Global MarketSM
DTI,Drilling Tools International Corporation  - Common Stock,NASDAQ Capital Market
DTIL,\"Precision BioSciences, Inc. - Common Stock\",NASDAQ Capital Market
DTSS,Datasea Inc. - Common Stock,NASDAQ Capital Market
DTST,Data Storage Corporation - Common Stock,NASDAQ Capital Market
DTSTW,Data Storage Corporation - Warrant,NASDAQ Capital Market
DUET,DUET Acquisition Corp. - Class A Common Stock,NASDAQ Global MarketSM
DUETU,DUET Acquisition Corp. - Unit,NASDAQ Global MarketSM
DUETW,DUET Acquisition Corp. - Warrant,NASDAQ Global MarketSM
DUO,Fangdd Network Group Ltd. - American Depositary Shares,NASDAQ Global MarketSM
DUOL,\"Duolingo, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
DUOT,\"Duos Technologies Group, Inc. - Common Stock\",NASDAQ Capital Market
DVAL,BrandywineGLOBAL-Dynamic US Large Cap Value ETF,NASDAQ Global MarketSM
DVAX,Dynavax Technologies Corporation - Common Stock,NASDAQ Global Select MarketSM
DVLU,First Trust Dorsey Wright Momentum & Value ETF,NASDAQ Global MarketSM
DVOL,First Trust Dorsey Wright Momentum & Low Volatility ETF,NASDAQ Global MarketSM
DVY,iShares Select Dividend ETF,NASDAQ Global MarketSM
DWAS,Invesco Dorsey Wright SmallCap Momentum ETF,NASDAQ Global MarketSM
DWAW,AdvisorShares Dorsey Wright FSM All Cap World ETF,NASDAQ Global MarketSM
DWSH,AdvisorShares Dorsey Wright Short ETF,NASDAQ Global MarketSM
DWSN,Dawson Geophysical Company - Common Stock,NASDAQ Global Select MarketSM
DWUS,AdvisorShares Dorsey Wright FSM US Core ETF,NASDAQ Global MarketSM
DXCM,\"DexCom, Inc. - Common Stock\",NASDAQ Global Select MarketSM
DXJS,WisdomTree Japan Hedged SmallCap Equity Fund,NASDAQ Global MarketSM
DXLG,\"Destination XL Group, Inc. - Common Stock\",NASDAQ Global MarketSM
DXPE,\"DXP Enterprises, Inc. - Common Stock\",NASDAQ Global Select MarketSM
DXR,Daxor Corporation - Closed End Fund,NASDAQ Capital Market
DXYN,\"The Dixie Group, Inc. - Common Stock\",NASDAQ Capital Market
DYAI,\"Dyadic International, Inc. - Common Stock\",NASDAQ Capital Market
DYCQ,DT Cloud Acquisition Corporation - Ordinary shares,NASDAQ Global MarketSM
DYCQR,DT Cloud Acquisition Corporation - Right,NASDAQ Global MarketSM
DYCQU,DT Cloud Acquisition Corporation - Unit,NASDAQ Global MarketSM
DYFI,IDX Dynamic Fixed Income ETF,NASDAQ Global MarketSM
DYN,\"Dyne Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
DYNI,IDX Dynamic Innovation ETF,NASDAQ Global MarketSM
DYNT,Dynatronics Corporation - Common Stock,NASDAQ Capital Market
DYTA,SGI Dynamic Tactical ETF,NASDAQ Global MarketSM
DZSI,DZS Inc. - Common Stock,NASDAQ Capital Market
EA,Electronic Arts Inc. - Common Stock,NASDAQ Global Select MarketSM
EAST,\"Eastside Distilling, Inc. - Common Stock\",NASDAQ Capital Market
EBAY,eBay Inc. - Common Stock,NASDAQ Global Select MarketSM
EBC,\"Eastern Bankshares, Inc. - Common Stock\",NASDAQ Global Select MarketSM
EBIZ,Global X E-commerce ETF,NASDAQ Global MarketSM
EBMT,\"Eagle Bancorp Montana, Inc. - Common Stock\",NASDAQ Global MarketSM
EBON,Ebang International Holdings Inc. - Class A Ordinary Shares,NASDAQ Global Select MarketSM
EBTC,Enterprise Bancorp Inc - Common Stock,NASDAQ Global Select MarketSM
ECBK,\"ECB Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
ECDA,\"ECD Automotive Design, Inc. - Common Stock\",NASDAQ Global MarketSM
ECDAW,\"ECD Automotive Design, Inc. - Warrant\",NASDAQ Capital Market
ECOR,\"electroCore, Inc. - Common Stock\",NASDAQ Capital Market
ECOW,Pacer Emerging Markets Cash Cows 100 ETF,NASDAQ Global MarketSM
ECPG,Encore Capital Group Inc - Common Stock,NASDAQ Global Select MarketSM
ECX,ECARX Holdings Inc. - Class A Ordinary shares,NASDAQ Global MarketSM
ECXWW,ECARX Holdings Inc. - Warrants,NASDAQ Capital Market
EDAP,\"EDAP TMS S.A. - American Depositary Shares, each representing One Ordinary Share\",NASDAQ Global MarketSM
EDBL,Edible Garden AG Incorporated - Common Stock,NASDAQ Capital Market
EDBLW,Edible Garden AG Incorporated - Warrant,NASDAQ Capital Market
EDIT,\"Editas Medicine, Inc. - Common Stock\",NASDAQ Global Select MarketSM
EDOC,Global X Telemedicine & Digital Health ETF,NASDAQ Global MarketSM
EDRY,EuroDry Ltd. - Common Shares,NASDAQ Capital Market
EDSA,\"Edesa Biotech, Inc. - Common Shares\",NASDAQ Capital Market
EDTK,Skillful Craftsman Education Technology Limited - Ordinary Share,NASDAQ Capital Market
EDUC,Educational Development Corporation - Common Stock,NASDAQ Global MarketSM
EEFT,\"Euronet Worldwide, Inc. - Common Stock\",NASDAQ Global Select MarketSM
EEIQ,EpicQuest Education Group International Limited - Common Stock,NASDAQ Capital Market
EEMA,iShares MSCI Emerging Markets Asia ETF,NASDAQ Global MarketSM
EFAS,Global X MSCI SuperDividend EAFE ETF,NASDAQ Global MarketSM
EFOI,\"Energy Focus, Inc. - Common Stock\",NASDAQ Capital Market
EFRA,iShares Environmental Infrastructure and Industrials ETF,NASDAQ Global MarketSM
EFSC,Enterprise Financial Services Corporation - Common Stock,NASDAQ Global Select MarketSM
EFSCP,\"Enterprise Financial Services Corporation - Depositary Shares Each Representing a 1/40th Interest in a Share of 5% Fixed Rate Non-Cumulative Perpetual Preferred Stock, Series A\",NASDAQ Global Select MarketSM
EFTR,\"eFFECTOR Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
EFTRW,\"eFFECTOR Therapeutics, Inc. - Warrant\",NASDAQ Capital Market
EGAN,eGain Corporation - Common Stock,NASDAQ Capital Market
EGBN,\"Eagle Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
EGHT,8x8 Inc - Common stock,NASDAQ Global Select MarketSM
EGIO,\"Edgio, Inc. - Common Stock\",NASDAQ Capital Market
EGRX,\"Eagle Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global MarketSM
EH,EHang Holdings Limited - ADS,NASDAQ Global MarketSM
EHLS,Even Herd Long Short ETF,NASDAQ Global MarketSM
EHTH,\"eHealth, Inc. - Common Stock\",NASDAQ Global Select MarketSM
EJH,E-Home Household Service Holdings Limited - Ordinary shares,NASDAQ Capital Market
EKG,First Trust Nasdaq Lux Digital Health Solutions ETF,NASDAQ Global MarketSM
EKSO,\"Ekso Bionics Holdings, Inc. - Common Stock\",NASDAQ Capital Market
ELAB,\"Elevai Labs, Inc. - Common Stock\",NASDAQ Capital Market
ELBM,Electra Battery Materials Corporation - Common Stock,NASDAQ Capital Market
ELDN,\"Eledon Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
ELEV,\"Elevation Oncology, Inc. - Common stock\",NASDAQ Global Select MarketSM
ELSE,\"Electro-Sensors, Inc. - Common Stock\",NASDAQ Capital Market
ELTK,Eltek Ltd. - Ordinary Shares,NASDAQ Capital Market
ELTX,\"Elicio Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
ELUT,\"Elutia, Inc. - Class A Common Stock\",NASDAQ Capital Market
ELVA,Electrovaya Inc. - Common Shares,NASDAQ Capital Market
ELVN,\"Enliven Therapeutics, Inc.  - Common Stock\",NASDAQ Global Select MarketSM
ELWS,\"Earlyworks Co., Ltd. - American Depositary Shares\",NASDAQ Capital Market
ELYM,\"Eliem Therapeutics, Inc - Common Stock\",NASDAQ Global MarketSM
EM,Smart Share Global Limited - American Depositary Shares,NASDAQ Capital Market
EMB,iShares J.P. Morgan USD Emerging Markets Bond ETF,NASDAQ Global MarketSM
EMBC,Embecta Corp. - Common Stock,NASDAQ Global Select MarketSM
EMCB,WisdomTree Emerging Markets Corporate Bond Fund,NASDAQ Global MarketSM
EMCG,Embrace Change Acquisition Corp - Ordinary Shares,NASDAQ Global MarketSM
EMCGR,Embrace Change Acquisition Corp - Rights,NASDAQ Global MarketSM
EMCGU,Embrace Change Acquisition Corp - Units,NASDAQ Global MarketSM
EMCGW,Embrace Change Acquisition Corp - Warrants,NASDAQ Global MarketSM
EMIF,iShares Emerging Markets Infrastructure ETF,NASDAQ Global MarketSM
EMKR,EMCORE Corporation - Common Stock,NASDAQ Capital Market
EML,Eastern Company (The) - Common Stock,NASDAQ Global MarketSM
EMLD,FTAC Emerald Acquisition Corp. - Class A Common Stock,NASDAQ Global MarketSM
EMLDU,FTAC Emerald Acquisition Corp. - Unit,NASDAQ Global MarketSM
EMLDW,FTAC Emerald Acquisition Corp. - Warrant,NASDAQ Global MarketSM
EMXC,iShares MSCI Emerging Markets ex China ETF,NASDAQ Global MarketSM
EMXF,iShares ESG Advanced MSCI EM ETF,NASDAQ Global MarketSM
ENG,ENGlobal Corporation - Common Stock,NASDAQ Capital Market
ENGN,enGene Holdings Inc. - Common Stock,NASDAQ Capital Market
ENGNW,enGene Holdings Inc. - Warrants,NASDAQ Capital Market
ENLT,Enlight Renewable Energy Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
ENLV,Enlivex Therapeutics Ltd. - Ordinary Shares,NASDAQ Capital Market
ENPH,\"Enphase Energy, Inc. - Common Stock\",NASDAQ Global MarketSM
ENSC,\"Ensysce Biosciences, Inc. - Common Stock\",NASDAQ Capital Market
ENSG,\"The Ensign Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ENTA,\"Enanta Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ENTG,\"Entegris, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ENTO,Entero Therapeutics Inc. - Common Stock,NASDAQ Capital Market
ENTR,ERShares Entrepreneurs ETF,NASDAQ Global MarketSM
ENTX,Entera Bio Ltd. - Ordinary Shares,NASDAQ Capital Market
ENVB,\"Enveric Biosciences, Inc.  - Common Stock\",NASDAQ Capital Market
ENVX,Enovix Corporation - Common Stock,NASDAQ Global Select MarketSM
ENZL,iShares MSCI New Zealand ETF,NASDAQ Global MarketSM
EOLS,\"Evolus, Inc. - Common Stock\",NASDAQ Global MarketSM
EOSE,\"Eos Energy Enterprises, Inc. - Common Stock\",NASDAQ Capital Market
EOSEW,\"Eos Energy Enterprises, Inc. - Warrant\",NASDAQ Capital Market
EPIX,ESSA Pharma Inc. - Common Stock,NASDAQ Capital Market
EPOW,\"Sunrise New Energy Co., Ltd - Class A Ordinary Shares\",NASDAQ Capital Market
EPRX,Eupraxia Pharmaceuticals Inc. - Common Stock,NASDAQ Capital Market
EPSN,Epsilon Energy Ltd. - Common Shares,NASDAQ Global MarketSM
EQ,\"Equillium, Inc. - Common Stock\",NASDAQ Capital Market
EQIX,\"Equinix, Inc. - Common Stock\",NASDAQ Global Select MarketSM
EQRR,ProShares Equities for Rising Rates ETF,NASDAQ Global MarketSM
ERAS,\"Erasca, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ERET,iShares Environmentally Aware Real Estate ETF,NASDAQ Global MarketSM
ERIC,Ericsson - American Depositary Shares each representing 1 underlying Class B share,NASDAQ Global Select MarketSM
ERIE,Erie Indemnity Company - Class A Common Stock,NASDAQ Global Select MarketSM
ERII,\"Energy Recovery, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ERNA,Eterna Therapeutics Inc. - Common Stock,NASDAQ Capital Market
ERNZ,TrueShares Active Yield ETF,NASDAQ Global MarketSM
ESCA,\"Escalade, Incorporated - Common Stock\",NASDAQ Global MarketSM
ESEA,Euroseas Ltd. - Common Stock,NASDAQ Capital Market
ESGD,iShares ESG Aware MSCI EAFE ETF,NASDAQ Global MarketSM
ESGE,iShares ESG Aware MSCI EM ETF,NASDAQ Global MarketSM
ESGL,ESGL Holdings Limited - Class A Ordinary Shares,NASDAQ Capital Market
ESGLW,ESGL Holdings Limited - Warrants,NASDAQ Capital Market
ESGR,Enstar Group Limited - Ordinary Shares,NASDAQ Global Select MarketSM
ESGRO,\"Enstar Group Limited - Depository Shares 7.00% Perpetual Non-Cumulative Preference Shares, Series E\",NASDAQ Global Select MarketSM
ESGRP,Enstar Group Limited - Depositary Shares Each Representing 1/1000th of an interest in Preference Shares,NASDAQ Global Select MarketSM
ESGU,iShares ESG Aware MSCI USA ETF,NASDAQ Global MarketSM
ESHA,ESH Acquisition Corp. - Class A Common Stock,NASDAQ Global MarketSM
ESHAR,ESH Acquisition Corp. - Right,NASDAQ Global MarketSM
ESLA,\"Estrella Immunopharma, Inc. - Common Stock\",NASDAQ Capital Market
ESLAW,\"Estrella Immunopharma, Inc. - Warrant\",NASDAQ Capital Market
ESLT,Elbit Systems Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
ESMV,iShares ESG MSCI USA Min Vol Factor ETF,NASDAQ Global MarketSM
ESOA,Energy Services of America Corporation - Common Stock,NASDAQ Capital Market
ESPO,VanEck Video Gaming and eSports ETF,NASDAQ Global MarketSM
ESPR,\"Esperion Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
ESQ,\"Esquire Financial Holdings, Inc. - Common Stock\",NASDAQ Capital Market
ESSA,\"ESSA Bancorp, Inc. - common stock\",NASDAQ Global Select MarketSM
ESTA,Establishment Labs Holdings Inc. - Common Shares,NASDAQ Capital Market
ETEC,iShares Breakthrough Environmental Solutions ETF,NASDAQ Global MarketSM
ETNB,\"89bio, Inc. - Common Stock\",NASDAQ Global MarketSM
ETON,\"Eton Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global MarketSM
ETSY,\"Etsy, Inc. - Common Stock\",NASDAQ Global Select MarketSM
EU,enCore Energy Corp. - Common Stock,NASDAQ Capital Market
EUDA,Euda Health Holdings Limited - Ordinary Shares,NASDAQ Capital Market
EUDAW,Euda Health Holdings Limited - Warrant,NASDAQ Capital Market
EUFN,iShares MSCI Europe Financials ETF,NASDAQ Global MarketSM
EVAX,Evaxion Biotech A/S - American Depositary Share,NASDAQ Capital Market
EVBG,\"Everbridge, Inc. - Common Stock\",NASDAQ Global MarketSM
EVCM,EverCommerce Inc. - Common Stock,NASDAQ Global Select MarketSM
EVER,\"EverQuote, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
EVGN,Evogene Ltd. - Ordinary Shares,NASDAQ Capital Market
EVGO,EVgo Inc. - Common Stock,NASDAQ Global Select MarketSM
EVGOW,\"EVgo Inc. - Warrants, each whole warrant exercisable for one share of Class A Common Stock at an exercise price of $11.50\",NASDAQ Global Select MarketSM
EVGR,Evergreen Corporation - Class A Ordinary Share,NASDAQ Global MarketSM
EVGRU,Evergreen Corporation - Unit,NASDAQ Global MarketSM
EVGRW,Evergreen Corporation - Warrant,NASDAQ Global MarketSM
EVLV,\"Evolv Technologies Holdings, Inc. - Class A Common Stock\",NASDAQ Capital Market
EVLVW,\"Evolv Technologies Holdings, Inc. - Warrant\",NASDAQ Capital Market
EVMT,Invesco Electric Vehicle Metals Commodity Strategy No K-1 ETF,NASDAQ Global MarketSM
EVO,Evotec SE - American Depositary Shares each representing 1/2 of one ordinary share,NASDAQ Global Select MarketSM
EVOK,\"Evoke Pharma, Inc. - Common Stock\",NASDAQ Capital Market
EVRG,\"Evergy, Inc. - Common Stock\",NASDAQ Global Select MarketSM
EVTV,\"Envirotech Vehicles, Inc. - Common stock\",NASDAQ Capital Market
EWBC,\"East West Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
EWCZ,\"European Wax Center, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
EWJV,iShares MSCI Japan Value ETF,NASDAQ Global MarketSM
EWTX,\"Edgewise Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
EWZS,iShares MSCI Brazil Small-Cap ETF,NASDAQ Global MarketSM
EXAI,Exscientia Plc - American Depositary Shares,NASDAQ Global Select MarketSM
EXAS,Exact Sciences Corporation - Common Stock,NASDAQ Capital Market
EXC,Exelon Corporation - Common Stock,NASDAQ Global Select MarketSM
EXEL,\"Exelixis, Inc. - Common Stock\",NASDAQ Global Select MarketSM
EXFY,\"Expensify, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
EXLS,\"ExlService Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
EXPE,\"Expedia Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
EXPI,\"eXp World Holdings, Inc. - Common Stock\",NASDAQ Global MarketSM
EXPO,\"Exponent, Inc. - Common Stock\",NASDAQ Global Select MarketSM
EXTR,\"Extreme Networks, Inc. - Common Stock\",NASDAQ Global Select MarketSM
EYE,\"National Vision Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
EYEG,AB Corporate Bond ETF,NASDAQ Global MarketSM
EYEN,\"Eyenovia, Inc. - Common Stock\",NASDAQ Capital Market
EYPT,\"EyePoint Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global MarketSM
EZFL,\"EzFill Holdings, Inc. - Common Stock\",NASDAQ Capital Market
EZGO,EZGO Technologies Ltd. - Ordinary Shares,NASDAQ Capital Market
EZPW,\"EZCORP, Inc. - Class A Non-Voting Common Stock\",NASDAQ Global Select MarketSM
FA,First Advantage Corporation - Common Stock,NASDAQ Global Select MarketSM
FAAR,First Trust Alternative Absolute Return Strategy ETF,NASDAQ Global MarketSM
FAAS,DigiAsia Corp. - Ordinary Shares,NASDAQ Capital Market
FAASW,DigiAsia Corp. - Warrant,NASDAQ Capital Market
FAB,First Trust Multi Cap Value AlphaDEX Fund,NASDAQ Global MarketSM
FAD,First Trust Multi Cap Growth AlphaDEX Fund,NASDAQ Global MarketSM
FALN,iShares Fallen Angels USD Bond ETF,NASDAQ Global MarketSM
FAMI,\"Farmmi, INC. - Ordinary Shares\",NASDAQ Capital Market
FANG,\"Diamondback Energy, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FANH,\"Fanhua Inc. - American depositary shares, each representing 20 ordinary shares\",NASDAQ Global Select MarketSM
FARM,Farmer Brothers Company - Common Stock,NASDAQ Global Select MarketSM
FARO,\"FARO Technologies, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FAST,Fastenal Company - Common Stock,NASDAQ Global Select MarketSM
FAT,FAT Brands Inc. - Common Stock,NASDAQ Capital Market
FATBB,FAT Brands Inc. - Class B Common Stock,NASDAQ Capital Market
FATBP,FAT Brands Inc. - 8.25% Series B Cumulative Preferred Stock,NASDAQ Capital Market
FATBW,FAT Brands Inc. - Warrant,NASDAQ Capital Market
FATE,\"Fate Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
FBIO,\"Fortress Biotech, Inc. - Common Stock\",NASDAQ Capital Market
FBIOP,\"Fortress Biotech, Inc. - 9.375% Series A Cumulative Redeemable Perpetual Preferred Stock\",NASDAQ Capital Market
FBIZ,\"First Business Financial Services, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FBL,GraniteShares 2x Long META Daily ETF,NASDAQ Global MarketSM
FBLG,\"FibroBiologics, Inc. - Common Stock\",NASDAQ Global MarketSM
FBMS,\"The First Bancshares, Inc. - Common Stock\",NASDAQ Global MarketSM
FBNC,First Bancorp - Common Stock,NASDAQ Global Select MarketSM
FBOT,Fidelity Disruptive Automation ETF,NASDAQ Global MarketSM
FBRX,\"Forte Biosciences, Inc.  - Common Stock\",NASDAQ Capital Market
FBYD,\"Falcon's Beyond Global, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
FBYDW,\"Falcon's Beyond Global, Inc. - Warrants\",NASDAQ Capital Market
FBZ,First Trust Brazil AlphaDEX Fund,NASDAQ Global MarketSM
FCA,First Trust China AlphaDEX Fund,NASDAQ Global MarketSM
FCAL,First Trust California Municipal High income ETF,NASDAQ Global MarketSM
FCAP,\"First Capital, Inc. - Common Stock\",NASDAQ Capital Market
FCBC,\"First Community Bankshares, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FCCO,First Community Corporation - Common Stock,NASDAQ Capital Market
FCEF,First Trust Income Opportunities ETF,NASDAQ Global MarketSM
FCEL,\"FuelCell Energy, Inc. - Common Stock\",NASDAQ Global MarketSM
FCFS,\"FirstCash Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FCNCA,\"First Citizens BancShares, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
FCNCO,\"First Citizens BancShares, Inc. - 5.625% Non-Cumulative Perpetual Preferred Stock, Series C\",NASDAQ Global Select MarketSM
FCNCP,\"First Citizens BancShares, Inc. - Depositary Shares Each Representing a 1/40th Interest in a Share of 5.375% Non-Cumulative Perpetual Preferred Stock, Series A\",NASDAQ Global Select MarketSM
FCUV,Focus Universal Inc. - Common Stock,NASDAQ Global MarketSM
FCVT,First Trust SSI Strategic Convertible Securities ETF,NASDAQ Global MarketSM
FDBC,\"Fidelity D & D Bancorp, Inc. - Common Stock\",NASDAQ Global MarketSM
FDCF,Fidelity Disruptive Communications ETF,NASDAQ Global MarketSM
FDFF,Fidelity Disruptive Finance ETF,NASDAQ Global MarketSM
FDIF,Fidelity Disruptors ETF,NASDAQ Global MarketSM
FDIG,Fidelity Crypto Industry and Digital Payments ETF,NASDAQ Global MarketSM
FDIV,MarketDesk Focused U.S. Dividend ETF,NASDAQ Global MarketSM
FDMT,\"4D Molecular Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FDNI,First Trust Dow Jones International Internet ETF,NASDAQ Global MarketSM
FDT,First Trust Developed Markets Ex-US AlphaDEX Fund,NASDAQ Global MarketSM
FDTS,First Trust Developed Markets ex-US Small Cap AlphaDEX Fund,NASDAQ Global MarketSM
FDTX,Fidelity Disruptive Technology ETF,NASDAQ Global MarketSM
FDUS,Fidus Investment Corporation - Closed End Fund,NASDAQ Global Select MarketSM
FEAM,\"5E Advanced Materials, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FEBO,Fenbo Holdings Limited - Ordinary Shares,NASDAQ Capital Market
FEIM,\"Frequency Electronics, Inc. - Common Stock\",NASDAQ Global MarketSM
FELE,\"Franklin Electric Co., Inc. - Common Stock\",NASDAQ Global Select MarketSM
FEM,First Trust Emerging Markets AlphaDEX Fund,NASDAQ Global MarketSM
FEMB,First Trust Emerging Markets Local Currency Bond ETF,NASDAQ Global MarketSM
FEMS,First Trust Emerging Markets Small Cap AlphaDEX Fund,NASDAQ Global MarketSM
FEMY,Femasys Inc. - Common Stock,NASDAQ Capital Market
FENC,Fennec Pharmaceuticals Inc. - Common Stock,NASDAQ Capital Market
FEP,First Trust Europe AlphaDEX Fund,NASDAQ Global MarketSM
FEPI,REX FANG & Innovation Equity Premium Income ETF,NASDAQ Global MarketSM
FER,Ferrovial SE - Ordinary Shares,NASDAQ Global Select MarketSM
FEUZ,First Trust Eurozone AlphaDEX ETF,NASDAQ Global MarketSM
FEX,First Trust Large Cap Core AlphaDEX Fund,NASDAQ Global MarketSM
FEXD,Fintech Ecosystem Development Corp. - Class A Common Stock,NASDAQ Capital Market
FEXDR,Fintech Ecosystem Development Corp. - Right,NASDAQ Capital Market
FEXDU,Fintech Ecosystem Development Corp. - Units,NASDAQ Capital Market
FEXDW,Fintech Ecosystem Development Corp. - Warrant,NASDAQ Capital Market
FFBC,First Financial Bancorp. - Common Stock,NASDAQ Global Select MarketSM
FFIC,Flushing Financial Corporation - Common Stock,NASDAQ Global Select MarketSM
FFIE,Faraday Future Intelligent Electric Inc. - Common Stock,NASDAQ Capital Market
FFIEW,Faraday Future Intelligent Electric Inc. - Warrant,NASDAQ Capital Market
FFIN,\"First Financial Bankshares, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FFIV,\"F5, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FFNW,\"First Financial Northwest, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FGBI,\"First Guaranty Bancshares, Inc. - Common Stock\",NASDAQ Global MarketSM
FGBIP,\"First Guaranty Bancshares, Inc. - 6.75% Series A Fixed-Rate Non-Cumulative Perpetual Preferred Stock\",NASDAQ Global MarketSM
FGEN,\"FibroGen, Inc - Common Stock\",NASDAQ Global Select MarketSM
FGF,Fundamental Global Inc. - Common Stock,NASDAQ Global MarketSM
FGFPP,Fundamental Global Inc. - 8.00% Cumulative Series A Preferred Stock,NASDAQ Global MarketSM
FGI,FGI Industries Ltd. - Ordinary Shares,NASDAQ Capital Market
FGIWW,FGI Industries Ltd. - warrant,NASDAQ Capital Market
FGM,First Trust Germany AlphaDEX Fund,NASDAQ Global MarketSM
FHB,\"First Hawaiian, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FHTX,Foghorn Therapeutics Inc. - Common Stock,NASDAQ Global MarketSM
FIAC,Focus Impact Acquisition Corp. - Class A Common Stock,NASDAQ Global MarketSM
FIACU,Focus Impact Acquisition Corp. - Unit,NASDAQ Global MarketSM
FIACW,Focus Impact Acquisition Corp. - Warrant,NASDAQ Global MarketSM
FIBK,\"First Interstate BancSystem, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FICS,First Trust International Developed Capital Strength ETF,NASDAQ Global MarketSM
FID,First Trust S&P International Dividend Aristocrats ETF,NASDAQ Global MarketSM
FINE,Themes European Luxury ETF,NASDAQ Global MarketSM
FINW,FinWise Bancorp - Common Stock,NASDAQ Global MarketSM
FINX,Global X FinTech ETF,NASDAQ Global MarketSM
FIP,FTAI Infrastructure Inc. - Common Stock,NASDAQ Global Select MarketSM
FISI,\"Financial Institutions, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FITB,Fifth Third Bancorp - Common Stock,NASDAQ Global Select MarketSM
FITBI,Fifth Third Bancorp - Depositary Share repstg 1/1000th Ownership Interest Perp Pfd Series I,NASDAQ Global Select MarketSM
FITBO,\"Fifth Third Bancorp - Depositary Shares each representing a 1/1000th ownership interest in a share of Non-Cumulative Perpetual Preferred Stock, Series K\",NASDAQ Global Select MarketSM
FITBP,\"Fifth Third Bancorp - Depositary Shares each representing 1/40th share of Fifth Third 6.00% Non-Cumulative Perpetual Class B Preferred Stock, Series A\",NASDAQ Global Select MarketSM
FIVE,\"Five Below, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FIVN,\"Five9, Inc. - Common Stock\",NASDAQ Global MarketSM
FIXD,First Trust TCW Opportunistic Fixed Income ETF,NASDAQ Global MarketSM
FIXT,Procure Disaster Recovery Strategy ETF,NASDAQ Global MarketSM
FIZZ,National Beverage Corp. - Common Stock,NASDAQ Global Select MarketSM
FJP,First Trust Japan AlphaDEX Fund,NASDAQ Global MarketSM
FKU,First Trust United Kingdom AlphaDEX Fund,NASDAQ Global MarketSM
FKWL,Franklin Wireless Corp. - common stock,NASDAQ Capital Market
FLDB,Fidelity Low Duration Bond ETF,NASDAQ Global MarketSM
FLEX,Flex Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
FLFV,Feutune Light Acquisition Corporation - Class A Common Stock,NASDAQ Global MarketSM
FLFVR,Feutune Light Acquisition Corporation - Right,NASDAQ Global MarketSM
FLFVU,Feutune Light Acquisition Corporation - Unit,NASDAQ Global MarketSM
FLFVW,Feutune Light Acquisition Corporation - Warrant,NASDAQ Global MarketSM
FLGC,Flora Growth Corp. - Common Stock,NASDAQ Capital Market
FLGT,\"Fulgent Genetics, Inc. - Common Stock\",NASDAQ Global MarketSM
FLIC,The First of Long Island Corporation - Common Stock,NASDAQ Capital Market
FLJ,FLJ Group Limited - American Depositary Shares,NASDAQ Global MarketSM
FLL,\"Full House Resorts, Inc. - Common Stock\",NASDAQ Capital Market
FLN,First Trust Latin America AlphaDEX Fund,NASDAQ Global MarketSM
FLNC,\"Fluence Energy, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
FLNT,\"Fluent, Inc. - Common Stock\",NASDAQ Capital Market
FLUX,\"Flux Power Holdings, Inc. - Common Stock\",NASDAQ Capital Market
FLWS,\"1-800-FLOWERS.COM, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
FLXS,\"Flexsteel Industries, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FLYW,Flywire Corporation - Voting Common Stock,NASDAQ Global Select MarketSM
FMAO,\"Farmers & Merchants Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
FMB,First Trust Managed Municipal ETF,NASDAQ Global MarketSM
FMBH,\"First Mid Bancshares, Inc. - Common Stock\",NASDAQ Global MarketSM
FMED,Fidelity Disruptive Medicine ETF,NASDAQ Global MarketSM
FMET,Fidelity Metaverse ETF,NASDAQ Global MarketSM
FMHI,First Trust Municipal High Income ETF,NASDAQ Global MarketSM
FMNB,Farmers National Banc Corp. - Common Stock,NASDAQ Capital Market
FMST,Foremost Lithium Resource & Technology Ltd. - Common stock,NASDAQ Capital Market
FMSTW,Foremost Lithium Resource & Technology Ltd. - Warrant,NASDAQ Capital Market
FNCB,FNCB Bancorp Inc. - Common Stock,NASDAQ Capital Market
FNCH,\"Finch Therapeutics Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FNGR,\"FingerMotion, Inc. - common stock\",NASDAQ Capital Market
FNK,First Trust Mid Cap Value AlphaDEX Fund,NASDAQ Global MarketSM
FNKO,\"Funko, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
FNLC,\"First Bancorp, Inc (ME) - Common Stock\",NASDAQ Global Select MarketSM
FNVT,Finnovate Acquisition Corp. - Class A Ordinary Shares,NASDAQ Global MarketSM
FNVTU,Finnovate Acquisition Corp. - Units,NASDAQ Global MarketSM
FNVTW,Finnovate Acquisition Corp. - Warrants,NASDAQ Global MarketSM
FNWB,First Northwest Bancorp - Common Stock,NASDAQ Global MarketSM
FNWD,Finward Bancorp - common stock,NASDAQ Capital Market
FNX,First Trust Mid Cap Core AlphaDEX Fund,NASDAQ Global MarketSM
FNY,First Trust Mid Cap Growth AlphaDEX Fund,NASDAQ Global MarketSM
FOLD,\"Amicus Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
FONR,Fonar Corporation - Common Stock,NASDAQ Capital Market
FORA,Forian Inc. - Common Stock,NASDAQ Capital Market
FORD,\"Forward Industries, Inc. - Common Stock\",NASDAQ Capital Market
FORL,Four Leaf Acquisition Corporation - Class A Common Stock,NASDAQ Capital Market
FORLU,Four Leaf Acquisition Corporation - Unit,NASDAQ Capital Market
FORLW,Four Leaf Acquisition Corporation - Warrants,NASDAQ Capital Market
FORM,\"FormFactor, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FORR,\"Forrester Research, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FORTY,Formula Systems (1985) Ltd. - American Depositary Shares,NASDAQ Global Select MarketSM
FOSL,\"Fossil Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FOSLL,\"Fossil Group, Inc. - 7% Senior Notes due 2026\",NASDAQ Global Select MarketSM
FOX,Fox Corporation - Class B Common Stock,NASDAQ Global Select MarketSM
FOXA,Fox Corporation - Class A Common Stock,NASDAQ Global Select MarketSM
FOXF,Fox Factory Holding Corp. - Common Stock,NASDAQ Global Select MarketSM
FPA,First Trust Asia Pacific Ex-Japan AlphaDEX Fund,NASDAQ Global MarketSM
FPAY,\"FlexShopper, Inc. - Common Stock\",NASDAQ Capital Market
FPXE,First Trust IPOX Europe Equity Opportunities ETF,NASDAQ Global MarketSM
FPXI,First Trust International Equity Opportunities ETF,NASDAQ Global MarketSM
FRAF,Franklin Financial Services Corporation - Common Stock,NASDAQ Capital Market
FRBA,First Bank  - Common Stock,NASDAQ Global MarketSM
FREE,\"Whole Earth Brands, Inc. - Class A Common Stock\",NASDAQ Capital Market
FREEW,\"Whole Earth Brands, Inc. - Warrant\",NASDAQ Capital Market
FRES,Fresh2 Group Limited - American Depositary Shares,NASDAQ Capital Market
FRGT,\"Freight Technologies, Inc. - Ordinary Shares\",NASDAQ Capital Market
FRHC,Freedom Holding Corp. - Common Stock,NASDAQ Capital Market
FRLA,Fortune Rise Acquisition Corporation - Class A Common Stock,NASDAQ Capital Market
FRLAU,Fortune Rise Acquisition Corporation - Units,NASDAQ Capital Market
FRLAW,Fortune Rise Acquisition Corporation - Warrant,NASDAQ Capital Market
FRME,First Merchants Corporation - Common Stock,NASDAQ Global Select MarketSM
FRMEP,\"First Merchants Corporation - Depository Shares, each representing a 1/100th interest in a share of 7.50% Non-Cumulative Perpetual Preferred Stock, A\",NASDAQ Global Select MarketSM
FROG,JFrog Ltd. - Ordinary shares,NASDAQ Global Select MarketSM
FRPH,\"FRP Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FRPT,\"Freshpet, Inc. - Common Stock\",NASDAQ Global MarketSM
FRSH,Freshworks Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
FRST,Primis Financial Corp. - Common Stock,NASDAQ Global MarketSM
FRSX,Foresight Autonomous Holdings Ltd. - American Depositary Shares,NASDAQ Capital Market
FRZA,\"Forza X1, Inc. - Common Stock\",NASDAQ Capital Market
FSBC,Five Star Bancorp - Common Stock,NASDAQ Global Select MarketSM
FSBW,\"FS Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
FSEA,\"First Seacoast Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
FSFG,\"First Savings Financial Group, Inc. - Common Stock\",NASDAQ Capital Market
FSLR,\"First Solar, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FSTR,L.B. Foster Company - Common Stock,NASDAQ Global Select MarketSM
FSV,FirstService Corporation - Common Shares,NASDAQ Global Select MarketSM
FSZ,First Trust Switzerland AlphaDEX Fund,NASDAQ Global MarketSM
FTA,First Trust Large Cap Value AlphaDEX Fund,NASDAQ Global MarketSM
FTAG,First Trust Indxx Global Agriculture ETF,NASDAQ Global MarketSM
FTAI,FTAI Aviation Ltd. - Common Stock,NASDAQ Global Select MarketSM
FTAIM,FTAI Aviation Ltd. - 9.500% Fixed-Rate Reset Series D Cumulative Perpetual Redeemable Preferred Shares,NASDAQ Global Select MarketSM
FTAIN,FTAI Aviation Ltd. - 8.25% Fixed-Rate Reset Series C Cumulative Perpetual Redeemable Preferred Shares,NASDAQ Global Select MarketSM
FTAIO,FTAI Aviation Ltd. - 8.00% Fixed-to-Floating Rate Series B Cumulative Perpetual Redeemable Preferred Shares,NASDAQ Global Select MarketSM
FTAIP,FTAI Aviation Ltd. - 8.25% Fixed-to-Floating Rate Series A Cumulative Perpetual Redeemable Preferred Shares,NASDAQ Global Select MarketSM
FTC,First Trust Large Cap Growth AlphaDEX Fund,NASDAQ Global MarketSM
FTCI,\"FTC Solar, Inc. - Common Stock\",NASDAQ Global MarketSM
FTCS,First Trust Capital Strength ETF,NASDAQ Global MarketSM
FTDR,\"Frontdoor, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FTDS,First Trust Dividend Strength ETF,NASDAQ Global MarketSM
FTEK,\"Fuel Tech, Inc. - Common Stock\",NASDAQ Capital Market
FTEL,Fitell Corporation - Ordinary Shares,NASDAQ Capital Market
FTFT,Future FinTech Group Inc. - Common Stock,NASDAQ Capital Market
FTGC,First Trust Global Tactical Commodity Strategy Fund,NASDAQ Global MarketSM
FTGS,First Trust Growth Strength ETF,NASDAQ Global MarketSM
FTHI,First Trust BuyWrite Income ETF,NASDAQ Global MarketSM
FTHM,Fathom Holdings Inc. - Common Stock,NASDAQ Capital Market
FTII,FutureTech II Acquisition Corp. - Class A Common Stock,NASDAQ Global MarketSM
FTIIU,FutureTech II Acquisition Corp. - Unit,NASDAQ Global MarketSM
FTIIW,FutureTech II Acquisition Corp. - Warrant,NASDAQ Global MarketSM
FTLF,\"FitLife Brands, Inc. - Common Stock\",NASDAQ Capital Market
FTNT,\"Fortinet, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FTQI,First Trust Nasdaq BuyWrite Income ETF,NASDAQ Global MarketSM
FTRE,Fortrea Holdings Inc. - Common Stock,NASDAQ Global Select MarketSM
FTRI,First Trust Indxx Global Natural Resources Income ETF,NASDAQ Global MarketSM
FTSL,First Trust Senior Loan Fund,NASDAQ Global MarketSM
FTSM,First Trust Enhanced Short Maturity ETF,NASDAQ Global MarketSM
FTXG,First Trust Nasdaq Food & Beverage ETF,NASDAQ Global MarketSM
FTXH,First Trust Nasdaq Pharmaceuticals ETF,NASDAQ Global MarketSM
FTXL,First Trust Nasdaq Semiconductor ETF,NASDAQ Global MarketSM
FTXN,First Trust Nasdaq Oil & Gas ETF,NASDAQ Global MarketSM
FTXO,First Trust Nasdaq Bank ETF,NASDAQ Global MarketSM
FTXR,First Trust Nasdaq Transportation ETF,NASDAQ Global MarketSM
FUFU,BitFuFu Inc. - Class A Ordinary Shares,NASDAQ Capital Market
FUFUW,BitFuFu Inc. - Warrant,NASDAQ Capital Market
FULC,\"Fulcrum Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
FULT,Fulton Financial Corporation - Common Stock,NASDAQ Global Select MarketSM
FULTP,\"Fulton Financial Corporation - Depositary Shares Each Representing a 1/40th Interest in a Share of Fixed Rate Non-Cumulative Perpetual Preferred Stock, Series A\",NASDAQ Global Select MarketSM
FUNC,First United Corporation - Common Stock,NASDAQ Global Select MarketSM
FUND,\"Sprott Focus Trust, Inc. - Closed End Fund\",NASDAQ Global Select MarketSM
FUSB,\"First US Bancshares, Inc. - Common Stock\",NASDAQ Capital Market
FUSN,Fusion Pharmaceuticals Inc. - Common Shares,NASDAQ Global Select MarketSM
FUTU,Futu Holdings Limited - American Depositary Shares,NASDAQ Global MarketSM
FV,First Trust Dorsey Wright Focus 5 ETF,NASDAQ Global MarketSM
FVC,First Trust Dorsey Wright Dynamic Focus 5 ETF,NASDAQ Global MarketSM
FVCB,\"FVCBankcorp, Inc. - Common Stock\",NASDAQ Capital Market
FWONA,Liberty Media Corporation - Series A Liberty Formula One Common Stock,NASDAQ Global Select MarketSM
FWONK,Liberty Media Corporation - Series C Liberty Formula One Common Stock,NASDAQ Global Select MarketSM
FWRD,Forward Air Corporation - Common Stock,NASDAQ Global Select MarketSM
FWRG,\"First Watch Restaurant Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FXNC,First National Corporation - Common Stock,NASDAQ Capital Market
FYBR,\"Frontier Communications Parent, Inc. - Common Stock\",NASDAQ Global Select MarketSM
FYC,First Trust Small Cap Growth AlphaDEX Fund,NASDAQ Global MarketSM
FYT,First Trust Small Cap Value AlphaDEX Fund,NASDAQ Global MarketSM
FYX,First Trust Small Cap Core AlphaDEX Fund,NASDAQ Global MarketSM
GABC,\"German American Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
GAIA,\"Gaia, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
GAIN,Gladstone Investment Corporation - Business Development Company,NASDAQ Global Select MarketSM
GAINL,Gladstone Investment Corporation - 8.00% Notes due 2028,NASDAQ Global Select MarketSM
GAINN,Gladstone Investment Corporation - 5.00% Notes Due 2026,NASDAQ Global Select MarketSM
GAINZ,Gladstone Investment Corporation - 4.875% Notes due 2028,NASDAQ Global Select MarketSM
GALT,Galectin Therapeutics Inc. - Common Stock,NASDAQ Capital Market
GAMB,Gambling.com Group Limited - Ordinary Shares,NASDAQ Global MarketSM
GAMC,Golden Arrow Merger Corp. - Class A Common Stock,NASDAQ Capital Market
GAMCU,Golden Arrow Merger Corp. - Unit,NASDAQ Capital Market
GAMCW,Golden Arrow Merger Corp. - Warrant,NASDAQ Capital Market
GAME,\"GameSquare Holdings, Inc. - Common stock\",NASDAQ Capital Market
GAN,GAN Limited - Ordinary Shares,NASDAQ Capital Market
GANX,\"Gain Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
GAQ,Generation Asia I Acquisition Limited - Class A Ordinary Shares,NASDAQ Global MarketSM
GASS,\"StealthGas, Inc. - common stock\",NASDAQ Global Select MarketSM
GATE,Marblegate Acquisition Corp. - Class A Common Stock,NASDAQ Capital Market
GATEU,Marblegate Acquisition Corp. - Unit,NASDAQ Capital Market
GATEW,Marblegate Acquisition Corp. - Warrant,NASDAQ Capital Market
GBBK,Global Blockchain Acquisition Corp. - Common Stock,NASDAQ Global MarketSM
GBBKR,Global Blockchain Acquisition Corp. - Right,NASDAQ Global MarketSM
GBBKW,Global Blockchain Acquisition Corp. - Warrant,NASDAQ Global MarketSM
GBDC,\"Golub Capital BDC, Inc. - Closed End Fund\",NASDAQ Global Select MarketSM
GBIO,Generation Bio Co. - Common stock,NASDAQ Global Select MarketSM
GBNY,\"Generations Bancorp NY, Inc. - Common Stock\",NASDAQ Capital Market
GCBC,\"Greene County Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
GCMG,GCM Grosvenor Inc. - Class A Common Stock,NASDAQ Global MarketSM
GCMGW,GCM Grosvenor Inc. - Warrant,NASDAQ Global MarketSM
GCT,GigaCloud Technology Inc - Class A Ordinary Shares,NASDAQ Global MarketSM
GCTK,\"GlucoTrack, Inc. - Common Stock\",NASDAQ Capital Market
GDC,GD Culture Group Limited - Common Stock,NASDAQ Capital Market
GDEN,\"Golden Entertainment, Inc. - Common Stock\",NASDAQ Global MarketSM
GDEV,GDEV Inc. - Ordinary Shares,NASDAQ Global MarketSM
GDEVW,GDEV Inc. - Warrant,NASDAQ Global MarketSM
GDHG,Golden Heaven Group Holdings Ltd.  - Class A Ordinary Shares,NASDAQ Capital Market
GDRX,\"GoodRx Holdings, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
GDS,GDS Holdings Limited - American Depositary Shares,NASDAQ Global MarketSM
GDST,Goldenstone Acquisition Limited - Common Stock,NASDAQ Capital Market
GDSTR,Goldenstone Acquisition Limited - Rights,NASDAQ Capital Market
GDSTU,Goldenstone Acquisition Limited - Units,NASDAQ Capital Market
GDSTW,Goldenstone Acquisition Limited - Warrants,NASDAQ Capital Market
GDTC,CytoMed Therapeutics Limited - Ordinary Shares,NASDAQ Capital Market
GDYN,\"Grid Dynamics Holdings, Inc. - Class A Common Stock\",NASDAQ Capital Market
GECC,Great Elm Capital Corp. - Closed End Fund,NASDAQ Global MarketSM
GECCI,Great Elm Capital Corp. - 8.50% NOTES DUE 2029,NASDAQ Global MarketSM
GECCM,Great Elm Capital Corp. - 6.75% Notes Due 2025,NASDAQ Global MarketSM
GECCO,Great Elm Capital Corp. - 5.875% Notes due 2026,NASDAQ Global MarketSM
GECCZ,Great Elm Capital Corp. - 8.75% Notes due 2028,NASDAQ Global MarketSM
GEG,\"Great Elm Group, Inc.  - Common Stock\",NASDAQ Global Select MarketSM
GEGGL,\"Great Elm Group, Inc.  - 7.25% Notes due 2027\",NASDAQ Global MarketSM
GEHC,GE HealthCare Technologies Inc. - Common Stock,NASDAQ Global Select MarketSM
GEN,Gen Digital Inc. - Common Stock,NASDAQ Global Select MarketSM
GENE,Genetic Technologies Ltd - American Depositary Shares,NASDAQ Capital Market
GENK,\"GEN Restaurant Group, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
GEOS,Geospace Technologies Corporation - Common Stock,NASDAQ Global Select MarketSM
GERN,Geron Corporation - Common Stock,NASDAQ Global Select MarketSM
GEVO,\"Gevo, Inc. - Common Stock\",NASDAQ Capital Market
GFAI,\"Guardforce AI Co., Limited - Ordinary Shares\",NASDAQ Capital Market
GFAIW,\"Guardforce AI Co., Limited - Warrant\",NASDAQ Capital Market
GFGF,Guru Favorite Stocks ETF,NASDAQ Global MarketSM
GFS,GlobalFoundries Inc. - Ordinary Share,NASDAQ Global Select MarketSM
GGAL,\"Grupo Financiero Galicia S.A. - American Depositary Shares, Class B Shares underlying\",NASDAQ Capital Market
GGLL,Direxion Daily GOOGL Bull 2X Shares,NASDAQ Global MarketSM
GGLS,Direxion Daily GOOGL Bear 1X Shares,NASDAQ Global MarketSM
GGR,Gogoro Inc. - Ordinary Shares,NASDAQ Global Select MarketSM
GGROW,Gogoro Inc. - Warrant,NASDAQ Global Select MarketSM
GH,\"Guardant Health, Inc. - Common Stock\",NASDAQ Global Select MarketSM
GHIX,\"Gores Holdings IX, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
GHIXU,\"Gores Holdings IX, Inc. - Unit\",NASDAQ Global MarketSM
GHIXW,\"Gores Holdings IX, Inc. - Warrant\",NASDAQ Global MarketSM
GHRS,GH Research PLC - Ordinary Shares,NASDAQ Global MarketSM
GHSI,\"Guardion Health Sciences, Inc. - Common Stock\",NASDAQ Capital Market
GIFI,\"Gulf Island Fabrication, Inc. - Common Stock\",NASDAQ Global Select MarketSM
GIGM,GigaMedia Limited - Ordinary Shares,NASDAQ Capital Market
GIII,\"G-III Apparel Group, LTD. - Common Stock\",NASDAQ Global Select MarketSM
GILD,\"Gilead Sciences, Inc. - Common Stock\",NASDAQ Global Select MarketSM
GILT,Gilat Satellite Networks Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
GINX,SGI Enhanced Global Income ETF,NASDAQ Global MarketSM
GIPR,Generation Income Properties Inc. - Common stock,NASDAQ Capital Market
GIPRW,Generation Income Properties Inc. - Warrant,NASDAQ Capital Market
GLAC,Global Lights Acquisition Corp - Ordinary Shares,NASDAQ Global MarketSM
GLACR,Global Lights Acquisition Corp - Rights,NASDAQ Global MarketSM
GLACU,Global Lights Acquisition Corp - Unit,NASDAQ Global MarketSM
GLAD,Gladstone Capital Corporation - Closed End Fund,NASDAQ Global Select MarketSM
GLADZ,Gladstone Capital Corporation - 7.75% Notes due 2028,NASDAQ Global Select MarketSM
GLBE,Global-E Online Ltd. - ordinary shares,NASDAQ Global Select MarketSM
GLBS,Globus Maritime Limited - Common Stock,NASDAQ Capital Market
GLBZ,Glen Burnie Bancorp - Common Stock,NASDAQ Capital Market
GLDD,Great Lakes Dredge & Dock Corporation - Common Stock,NASDAQ Global Select MarketSM
GLDI,\"Credit Suisse X-Links Gold Shares Covered Call ETNs due February 2, 2033\",NASDAQ Global MarketSM
GLLI,Globalink Investment Inc. - Common Stock,NASDAQ Capital Market
GLLIR,Globalink Investment Inc. - Rights,NASDAQ Capital Market
GLLIU,Globalink Investment Inc. - Units,NASDAQ Capital Market
GLLIW,Globalink Investment Inc. - Warrants,NASDAQ Capital Market
GLMD,Galmed Pharmaceuticals Ltd. - Ordinary Shares,NASDAQ Capital Market
GLNG,Golar LNG Limited - Common Shares,NASDAQ Global Select MarketSM
GLPG,Galapagos NV - American Depositary Shares,NASDAQ Global Select MarketSM
GLPI,\"Gaming and Leisure Properties, Inc. - Common Stock\",NASDAQ Global Select MarketSM
GLRE,\"Greenlight Reinsurance, Ltd. - Class A Ordinary Shares\",NASDAQ Global Select MarketSM
GLSI,\"Greenwich LifeSciences, Inc. - Common stock\",NASDAQ Capital Market
GLST,\"Global Star Acquisition, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
GLSTR,\"Global Star Acquisition, Inc. - Right\",NASDAQ Global MarketSM
GLSTU,\"Global Star Acquisition, Inc. - Unit\",NASDAQ Global MarketSM
GLSTW,\"Global Star Acquisition, Inc. - Warrants\",NASDAQ Global MarketSM
GLTO,\"Galecto, Inc. - Common Stock\",NASDAQ Capital Market
GLUE,\"Monte Rosa Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
GLYC,\"GlycoMimetics, Inc. - Common Stock\",NASDAQ Global MarketSM
GMAB,Genmab A/S - American Depositary Shares,NASDAQ Global Select MarketSM
GMFI,Aetherium Acquisition Corp. - Class A Common Stock,NASDAQ Global MarketSM
GMFIU,Aetherium Acquisition Corp. - Unit,NASDAQ Global MarketSM
GMFIW,Aetherium Acquisition Corp. - Warrant,NASDAQ Global MarketSM
GMGI,\"Golden Matrix Group, Inc. - Common Stock\",NASDAQ Capital Market
GMM,Global Mofy Metaverse Limited - Ordinary Shares,NASDAQ Capital Market
GNFT,GENFIT S.A. - American Depositary Shares,NASDAQ Global Select MarketSM
GNLN,\"Greenlane Holdings, Inc. - Class A Common Stock\",NASDAQ Capital Market
GNLX,Genelux Corporation - Common Stock,NASDAQ Capital Market
GNMA,iShares GNMA Bond ETF,NASDAQ Global MarketSM
GNOM,Global X Genomics & Biotechnology ETF,NASDAQ Global MarketSM
GNPX,\"Genprex, Inc. - Common Stock\",NASDAQ Capital Market
GNSS,Genasys Inc. - Common Stock,NASDAQ Capital Market
GNTA,Genenta Science S.p.A. - American Depositary Shares,NASDAQ Capital Market
GNTX,Gentex Corporation - Common Stock,NASDAQ Global Select MarketSM
GO,Grocery Outlet Holding Corp. - Common Stock,NASDAQ Global Select MarketSM
GOCO,\"GoHealth, Inc. - Class A Common Stock\",NASDAQ Capital Market
GODN,Golden Star Acquisition Corporation - Ordinary Shares,NASDAQ Global MarketSM
GODNR,Golden Star Acquisition Corporation - Rights,NASDAQ Global MarketSM
GODNU,Golden Star Acquisition Corporation - Unit,NASDAQ Global MarketSM
GOEV,Canoo Inc.  - Class A Common Stock,NASDAQ Capital Market
GOEVW,Canoo Inc.  - Warrant,NASDAQ Capital Market
GOGL,Golden Ocean Group Limited - Common Stock,NASDAQ Global Select MarketSM
GOGO,Gogo Inc. - Common Stock,NASDAQ Global Select MarketSM
GOOD,Gladstone Commercial Corporation - Real Estate Investment Trust,NASDAQ Global Select MarketSM
GOODN,Gladstone Commercial Corporation - 6.625% Series E Cumulative Redeemable Preferred Stock,NASDAQ Global Select MarketSM
GOODO,\"Gladstone Commercial Corporation - 6.00% Series G Cumulative Redeemable Preferred Stock, par value $0.001 per share\",NASDAQ Global Select MarketSM
GOOG,Alphabet Inc. - Class C Capital Stock,NASDAQ Global Select MarketSM
GOOGL,Alphabet Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
GORV,\"Lazydays Holdings, Inc. - Common Stock\",NASDAQ Capital Market
GOSS,\"Gossamer Bio, Inc. - Common Stock\",NASDAQ Global Select MarketSM
GOVI,Invesco Equal Weight 0-30 Year Treasury ETF,NASDAQ Global MarketSM
GOVX,\"GeoVax Labs, Inc. - Common Stock\",NASDAQ Capital Market
GOVXW,\"GeoVax Labs, Inc. - Warrants\",NASDAQ Capital Market
GP,GreenPower Motor Company Inc. - Common Shares,NASDAQ Capital Market
GPAC,Global Partner Acquisition Corp II - Class A Ordinary Share,NASDAQ Capital Market
GPACU,Global Partner Acquisition Corp II - Unit,NASDAQ Capital Market
GPACW,Global Partner Acquisition Corp II - Warrant,NASDAQ Capital Market
GPAK,Gamer Pakistan Inc. - Common Stock,NASDAQ Capital Market
GPATU,GP-Act III Acquisition Corp. - Units,NASDAQ Global MarketSM
GPCR,Structure Therapeutics Inc. - American Depositary Shares,NASDAQ Global MarketSM
GPIQ,Goldman Sachs Nasdaq-100 Core Premium Income ETF,NASDAQ Global MarketSM
GPIX,Goldman Sachs S&P 500 Core Premium Income ETF,NASDAQ Global MarketSM
GPRE,\"Green Plains, Inc. - Common Stock\",NASDAQ Global Select MarketSM
GPRO,\"GoPro, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
GRAB,Grab Holdings Limited - Class A Ordinary Shares,NASDAQ Global Select MarketSM
GRABW,Grab Holdings Limited - Warrant,NASDAQ Global Select MarketSM
GRDI,GRIID Infrastructure Inc. - Common Stock,NASDAQ Global MarketSM
GRDIW,GRIID Infrastructure Inc. - Warrant,NASDAQ Global MarketSM
GREE,Greenidge Generation Holdings Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
GREEL,Greenidge Generation Holdings Inc. - 8.50% Senior Notes due 2026,NASDAQ Global Select MarketSM
GRFS,\"Grifols, S.A. - American Depositary Shares\",NASDAQ Global Select MarketSM
GRI,\"GRI Bio, Inc. - Common Stock\",NASDAQ Capital Market
GRID,First Trust NASDAQ Clean Edge Smart Grid Infrastructure Index Fund,NASDAQ Global MarketSM
GRIN,Grindrod Shipping Holdings Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
GRNQ,Greenpro Capital Corp. - Common Stock,NASDAQ Capital Market
GROM,Grom Social Enterprises Inc. - Common Stock,NASDAQ Capital Market
GROMW,Grom Social Enterprises Inc. - Warrants,NASDAQ Capital Market
GROW,\"U.S. Global Investors, Inc. - Class A Common Stock\",NASDAQ Capital Market
GRPN,\"Groupon, Inc. - Common Stock\",NASDAQ Global Select MarketSM
GRRR,Gorilla Technology Group Inc. - Ordinary shares,NASDAQ Capital Market
GRRRW,Gorilla Technology Group Inc. - Warrant,NASDAQ Capital Market
GRTS,\"Gritstone bio, Inc. - Common Stock\",NASDAQ Global Select MarketSM
GRTX,\"Galera Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
GRVY,\"GRAVITY Co., Ltd. - American depositary shares, each representing one common share.\",NASDAQ Global MarketSM
GRWG,GrowGeneration Corp. - Common Stock,NASDAQ Capital Market
GRYP,\"Gryphon Digital Mining, Inc - Common Stock\",NASDAQ Capital Market
GSBC,\"Great Southern Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
GSHD,\"Goosehead Insurance, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
GSIB,Themes Global Systemically Important Banks ETF,NASDAQ Global MarketSM
GSIT,\"GSI Technology, Inc. - Common Stock\",NASDAQ Global Select MarketSM
GSIW,Garden Stage Limited - Ordinary Shares,NASDAQ Capital Market
GSM,Ferroglobe PLC - Ordinary Shares,NASDAQ Capital Market
GSMGW,\"Cheer Holding, Inc.  - Warrant\",NASDAQ Capital Market
GSUN,Golden Sun Health Technology Group Limited - Class A Ordinary Shares,NASDAQ Capital Market
GT,The Goodyear Tire & Rubber Company - Common Stock,NASDAQ Global Select MarketSM
GTAC,Global Technology Acquisition Corp. I - Class A Ordinary Shares,NASDAQ Capital Market
GTACU,Global Technology Acquisition Corp. I - Unit,NASDAQ Capital Market
GTACW,Global Technology Acquisition Corp. I - Warrant,NASDAQ Capital Market
GTBP,\"GT Biopharma, Inc. - Common Stock\",NASDAQ Capital Market
GTEC,Greenland Technologies Holding Corporation - Ordinary Shares,NASDAQ Capital Market
GTHX,\"G1 Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
GTI,Graphjet Technology - Class A Ordinary Shares,NASDAQ Global MarketSM
GTIM,Good Times Restaurants Inc. - Common Stock,NASDAQ Capital Market
GTLB,GitLab Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
GTR,WisdomTree Target Range Fund,NASDAQ Global MarketSM
GTX,Garrett Motion Inc. - Common Stock,NASDAQ Global Select MarketSM
GURE,\"Gulf Resources, Inc. - Common Stock\",NASDAQ Global Select MarketSM
GUTS,\"Fractyl Health, Inc. - Common Stock\",NASDAQ Global MarketSM
GV,Visionary Holdings Inc. - Common Shares,NASDAQ Capital Market
GVH,Globavend Holdings Limited - Ord Shares,NASDAQ Capital Market
GVP,\"GSE Systems, Inc. - Common Stock\",NASDAQ Capital Market
GWAV,\"Greenwave Technology Solutions, Inc. - Common Stock\",NASDAQ Capital Market
GWRS,\"Global Water Resources, Inc. - common stock\",NASDAQ Global MarketSM
GXAI,Gaxos.ai Inc. - Common Stock,NASDAQ Capital Market
GXTG,Global X Thematic Growth ETF,NASDAQ Global MarketSM
GYRE,\"Gyre Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
GYRO,\"Gyrodyne , LLC - Common Stock\",NASDAQ Capital Market
HA,\"Hawaiian Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HAFC,Hanmi Financial Corporation - Common Stock,NASDAQ Global Select MarketSM
HAIA,Healthcare AI Acquisition Corp. - Class A Ordinary Shares,NASDAQ Capital Market
HAIAU,Healthcare AI Acquisition Corp. - Units,NASDAQ Capital Market
HAIAW,Healthcare AI Acquisition Corp. - Warrants,NASDAQ Capital Market
HAIN,\"The Hain Celestial Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HALO,\"Halozyme Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HAO,Haoxi Health Technology Limited - Class A Ord Share,NASDAQ Capital Market
HAS,\"Hasbro, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HAYN,\"Haynes International, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HBAN,Huntington Bancshares Incorporated - Common Stock,NASDAQ Global Select MarketSM
HBANL,\"Huntington Bancshares Incorporated - Depositary Shares, Each Representing a 1/40th Interest in a Share of 6.875% Series J Non-Cumulative Perpetual Preferred Stock\",NASDAQ Global Select MarketSM
HBANM,Huntington Bancshares Incorporated - Depositary Shares each representing a 1/1000th interest in a share of Huntington Series I Preferred Stock,NASDAQ Global Select MarketSM
HBANP,Huntington Bancshares Incorporated - Depositary Shares 4.500% Series H Non-Cumulative Perpetual Preferred Stock,NASDAQ Global Select MarketSM
HBCP,\"Home Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HBIO,\"Harvard Bioscience, Inc. - Common Stock\",NASDAQ Global MarketSM
HBNC,\"Horizon Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HBT,\"HBT Financial, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HCAT,\"Health Catalyst, Inc - Common stock\",NASDAQ Global Select MarketSM
HCKT,\"The Hackett Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HCM,HUTCHMED (China) Limited - American Depositary Shares,NASDAQ Global Select MarketSM
HCOW,Amplify Cash Flow High Income ETF,NASDAQ Global MarketSM
HCP,\"HashiCorp, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
HCSG,\"Healthcare Services Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HCTI,\"Healthcare Triangle, Inc. - Common Stock\",NASDAQ Capital Market
HCVI,Hennessy Capital Investment Corp. VI - Class A Common Stock,NASDAQ Global MarketSM
HCVIU,Hennessy Capital Investment Corp. VI - Unit,NASDAQ Global MarketSM
HCVIW,Hennessy Capital Investment Corp. VI - Warrant,NASDAQ Global MarketSM
HCWB,HCW Biologics Inc. - Common Stock,NASDAQ Global MarketSM
HDL,SUPER HI INTERNATIONAL HOLDING LTD. - American Depositary Shares,NASDAQ Global MarketSM
HDSN,\"Hudson Technologies, Inc. - Common Stock\",NASDAQ Capital Market
HEAR,Turtle Beach Corporation - Common Stock,NASDAQ Global MarketSM
HEES,\"H&E Equipment Services, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HELE,Helen of Troy Limited - Common Stock,NASDAQ Global Select MarketSM
HEPA,\"Hepion Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
HEPS,D-Market Electronic Services & Trading - American Depositary Shares,NASDAQ Global Select MarketSM
HERD,Pacer Cash Cows Fund of Funds ETF,NASDAQ Global MarketSM
HERO,Global X Video Games & Esports ETF,NASDAQ Global MarketSM
HEWG,iShares Currency Hedged MSCI Germany ETF,NASDAQ Global MarketSM
HFBL,\"Home Federal Bancorp, Inc. of Louisiana - Common Stock\",NASDAQ Capital Market
HFFG,HF Foods Group Inc. - Common Stock,NASDAQ Capital Market
HFWA,Heritage Financial Corporation - Common Stock,NASDAQ Global Select MarketSM
HGAS,Global Gas Corporation - Class A Common Stock,NASDAQ Capital Market
HGASW,Global Gas Corporation - Warrant,NASDAQ Capital Market
HGBL,Heritage Global Inc. - Common Stock,NASDAQ Capital Market
HHGC,HHG Capital Corporation - Ordinary Shares,NASDAQ Capital Market
HHGCR,HHG Capital Corporation - Rights,NASDAQ Capital Market
HHGCU,HHG Capital Corporation - Units,NASDAQ Capital Market
HHGCW,HHG Capital Corporation - Warrant,NASDAQ Capital Market
HHS,\"Harte Hanks, Inc. - Common Stock\",NASDAQ Global MarketSM
HIBB,\"Hibbett, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HIDE,Alpha Architect High Inflation and Deflation ETF,NASDAQ Global MarketSM
HIFS,Hingham Institution for Savings - Common Stock,NASDAQ Global MarketSM
HIHO,Highway Holdings Limited - Common Stock,NASDAQ Capital Market
HIMX,\"Himax Technologies, Inc. - American depositary shares, each of which represents two ordinary shares.\",NASDAQ Global Select MarketSM
HISF,First Trust High Income Strategic Focus ETF,NASDAQ Global MarketSM
HITI,High Tide Inc. - Common Shares,NASDAQ Capital Market
HIVE,HIVE Digital Technologies Ltd - Common Shares,NASDAQ Capital Market
HKIT,Hitek Global Inc. - Class A Ordinary Share,NASDAQ Capital Market
HLAL,Wahed FTSE USA Shariah ETF,NASDAQ Global MarketSM
HLIT,Harmonic Inc. - Common Stock,NASDAQ Global Select MarketSM
HLMN,Hillman Solutions Corp. - Common Stock,NASDAQ Global MarketSM
HLNE,Hamilton Lane Incorporated - Class A Common Stock,NASDAQ Global Select MarketSM
HLP,Hongli Group Inc. - Ordinary Shares,NASDAQ Capital Market
HLTH,Cue Health Inc. - Common Stock,NASDAQ Capital Market
HLVX,\"HilleVax, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HLXB,Helix Acquisition Corp. II - Class A Ordinary Shares,NASDAQ Global MarketSM
HMNF,\"HMN Financial, Inc. - Common Stock\",NASDAQ Global MarketSM
HMST,\"HomeStreet, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HNDL,Strategy Shares Nasdaq 7HANDL Index ETF,NASDAQ Global MarketSM
HNNA,\"Hennessy Advisors, Inc. - Common Stock\",NASDAQ Global MarketSM
HNNAZ,\"Hennessy Advisors, Inc. - 4.875% Notes due 2026\",NASDAQ Global MarketSM
HNRG,Hallador Energy Company - Common Stock,NASDAQ Capital Market
HNST,\"The Honest Company, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HNVR,\"Hanover Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HOFT,Hooker Furnishings Corporation - Common Stock,NASDAQ Global Select MarketSM
HOFV,Hall of Fame Resort & Entertainment Company - Common Stock,NASDAQ Capital Market
HOFVW,Hall of Fame Resort & Entertainment Company - Warrant,NASDAQ Capital Market
HOLI,\"Hollysys Automation Technologies, Ltd. - Common Stock\",NASDAQ Global Select MarketSM
HOLO,MicroCloud Hologram Inc. - Ordinary Shares,NASDAQ Capital Market
HOLOW,MicroCloud Hologram Inc. - Warrant,NASDAQ Capital Market
HOLX,\"Hologic, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HON,Honeywell International Inc. - Common Stock,NASDAQ Global Select MarketSM
HONE,\"HarborOne Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HOOD,\"Robinhood Markets, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
HOOK,HOOKIPA Pharma Inc. - Common Stock,NASDAQ Capital Market
HOPE,\"Hope Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HOTH,\"Hoth Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
HOUR,\"Hour Loop, Inc. - common stock\",NASDAQ Capital Market
HOVNP,Hovnanian Enterprises Inc - Depositary Share representing 1/1000th of 7.625% Series A Preferred Stock,NASDAQ Global MarketSM
HOVR,New Horizon Aircraft Ltd. - Class A Ordinary Shares,NASDAQ Capital Market
HOVRW,New Horizon Aircraft Ltd. - Warrant,NASDAQ Capital Market
HOWL,\"Werewolf Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HPCO,\"Hempacco Co., Inc. - Common Stock\",NASDAQ Capital Market
HPH,Highest Performances Holdings Inc. - American Depository Shares,NASDAQ Global MarketSM
HPK,\"HighPeak Energy, Inc. - Common Stock\",NASDAQ Global MarketSM
HPKEW,\"HighPeak Energy, Inc. - Warrant\",NASDAQ Global MarketSM
HQGO,Hartford US Quality Growth ETF,NASDAQ Global MarketSM
HQI,\"HireQuest, Inc. - Common Stock\",NASDAQ Capital Market
HQY,\"HealthEquity, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HRMY,\"Harmony Biosciences Holdings, Inc. - Common Stock\",NASDAQ Global MarketSM
HROW,\"Harrow, Inc. - Common Stock\",NASDAQ Global MarketSM
HROWL,\"Harrow, Inc. - 8.625% senior notes due 2026\",NASDAQ Global MarketSM
HROWM,\"Harrow, Inc. - 11.875% Senior Notes due 2027\",NASDAQ Global MarketSM
HRTS,Tema Obesity & Cardiometabolic ETF,NASDAQ Global MarketSM
HRTX,\"Heron Therapeutics, Inc.   - Common Stock\",NASDAQ Capital Market
HRYU,\"Hanryu Holdings, Inc. - Common Stock\",NASDAQ Capital Market
HRZN,Horizon Technology Finance Corporation - Common Stock,NASDAQ Global Select MarketSM
HSAI,\"Hesai Group - American Depositary Share, each ADS represents one Class B ordinary share\",NASDAQ Global Select MarketSM
HSCS,\"Heart Test Laboratories, Inc. - Common Stock\",NASDAQ Capital Market
HSCSW,\"Heart Test Laboratories, Inc. - Warrant\",NASDAQ Capital Market
HSDT,\"Helius Medical Technologies, Inc. - Class A Common Stock\",NASDAQ Capital Market
HSIC,\"Henry Schein, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HSII,\"Heidrick & Struggles International, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HSON,\"Hudson Global, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HSPO,Horizon Space Acquisition I Corp. - Ordinary Shares,NASDAQ Global MarketSM
HSPOR,Horizon Space Acquisition I Corp. - Right,NASDAQ Global MarketSM
HSPOU,Horizon Space Acquisition I Corp. - Unit,NASDAQ Global MarketSM
HSPOW,Horizon Space Acquisition I Corp. - Warrant,NASDAQ Global MarketSM
HST,\"Host Hotels & Resorts, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HSTM,\"HealthStream, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HTBI,\"HomeTrust Bancshares, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HTBK,Heritage Commerce Corp - Common Stock,NASDAQ Global Select MarketSM
HTCR,\"Heartcore Enterprises, Inc. - Common Stock\",NASDAQ Capital Market
HTHT,H World Group Limited - American Depositary Shares,NASDAQ Global Select MarketSM
HTIA,\"Healthcare Trust, Inc. - 7.375% Series A Cumulative Redeemable Perpetual Preferred Stock\",NASDAQ Global MarketSM
HTIBP,\"Healthcare Trust, Inc. - 7.125% Series B Cumulative Redeemable Perpetual Preferred Stock\",NASDAQ Global MarketSM
HTLD,\"Heartland Express, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HTLF,\"Heartland Financial USA, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HTLFP,\"Heartland Financial USA, Inc. - Depositary Shares, each representing a 1/400th ownership interest in a share of 7.00% Fixed-Rate Reset Non-Cumulative Perpetual Preferred Stock, Series E\",NASDAQ Global Select MarketSM
HTOO,Fusion Fuel Green PLC - Ordinary Shares,NASDAQ Global MarketSM
HTOOW,Fusion Fuel Green PLC - Warrant,NASDAQ Global MarketSM
HTZ,\"Hertz Global Holdings, Inc - Common Stock\",NASDAQ Global Select MarketSM
HTZWW,\"Hertz Global Holdings, Inc - Warrant\",NASDAQ Global Select MarketSM
HUBC,Hub Cyber Security Ltd. - Ordinary Shares,NASDAQ Global MarketSM
HUBCW,Hub Cyber Security Ltd. - Warrant expiring 2/27/28,NASDAQ Capital Market
HUBCZ,Hub Cyber Security Ltd. - Warrant,NASDAQ Global MarketSM
HUBG,\"Hub Group, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
HUDA,Hudson Acquisition I Corp. - Common Stock,NASDAQ Global MarketSM
HUDAR,Hudson Acquisition I Corp. - Right,NASDAQ Global MarketSM
HUDAU,Hudson Acquisition I Corp. - Unit,NASDAQ Global MarketSM
HUDI,\"Huadi International Group Co., Ltd. - Ordinary Shares\",NASDAQ Capital Market
HUGE,FSD Pharma Inc. - Class B Subordinate Voting Shares,NASDAQ Capital Market
HUIZ,Huize Holding Limited - American Depositary Shares,NASDAQ Global MarketSM
HUMA,\"Humacyte, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HUMAW,\"Humacyte, Inc. - Warrant\",NASDAQ Global Select MarketSM
HURC,\"Hurco Companies, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HURN,Huron Consulting Group Inc. - Common Stock,NASDAQ Global Select MarketSM
HUT,Hut 8 Corp. - Common Stock,NASDAQ Global Select MarketSM
HWBK,\"Hawthorn Bancshares, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HWC,Hancock Whitney Corporation - Common Stock,NASDAQ Global Select MarketSM
HWCPZ,Hancock Whitney Corporation - 6.25% Subordinated Notes due 2060,NASDAQ Global Select MarketSM
HWH,HWH International Inc. - Common Stock,NASDAQ Global MarketSM
HWKN,\"Hawkins, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HYDR,Global X Hydrogen ETF,NASDAQ Global MarketSM
HYFM,\"Hydrofarm Holdings Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
HYLS,First Trust Tactical High Yield ETF,NASDAQ Global MarketSM
HYMC,Hycroft Mining Holding Corporation - Class A Common Stock,NASDAQ Capital Market
HYMCL,Hycroft Mining Holding Corporation - Warrants,NASDAQ Capital Market
HYMCW,Hycroft Mining Holding Corporation - Warrant,NASDAQ Capital Market
HYPR,\"Hyperfine, Inc.  - Class A Common Stock\",NASDAQ Global MarketSM
HYW,Hywin Holdings Ltd. - American Depositary Shares,NASDAQ Global MarketSM
HYXF,iShares ESG Advanced High Yield Corporate Bond ETF,NASDAQ Global MarketSM
HYZD,WisdomTree Interest Rate Hedged High Yield Bond Fund,NASDAQ Global MarketSM
HYZN,Hyzon Motors Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
HYZNW,Hyzon Motors Inc. - Warrant,NASDAQ Global Select MarketSM
IAC,IAC Inc. - Common Stock,NASDAQ Global Select MarketSM
IART,Integra LifeSciences Holdings Corporation - Common Stock,NASDAQ Global Select MarketSM
IAS,Integral Ad Science Holding Corp. - Common Stock,NASDAQ Global Select MarketSM
IBAC,IB Acquisition Corp. - Common Stock,NASDAQ Global MarketSM
IBACR,IB Acquisition Corp. - Right,NASDAQ Global MarketSM
IBAT,iShares Energy Storage & Materials ETF,NASDAQ Global MarketSM
IBB,iShares Biotechnology ETF,NASDAQ Global MarketSM
IBBQ,Invesco Nasdaq Biotechnology ETF,NASDAQ Global MarketSM
IBCP,Independent Bank Corporation - Common Stock,NASDAQ Global Select MarketSM
IBEX,IBEX Limited - Common Share,NASDAQ Global MarketSM
IBIT,iShares Bitcoin Trust,NASDAQ Global MarketSM
IBKR,\"Interactive Brokers Group, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
IBOC,International Bancshares Corporation - Common Stock,NASDAQ Global Select MarketSM
IBOT,VanEck Robotics ETF,NASDAQ Global MarketSM
IBRX,\"ImmunityBio, Inc. - Common Stock\",NASDAQ Global Select MarketSM
IBTE,iShares iBonds Dec 2024 Term Treasury ETF,NASDAQ Global MarketSM
IBTF,iShares iBonds Dec 2025 Term Treasury ETF,NASDAQ Global MarketSM
IBTG,iShares iBonds Dec 2026 Term Treasury ETF,NASDAQ Global MarketSM
IBTH,iShares iBonds Dec 2027 Term Treasury ETF,NASDAQ Global MarketSM
IBTI,iShares iBonds Dec 2028 Term Treasury ETF,NASDAQ Global MarketSM
IBTJ,iShares iBonds Dec 2029 Term Treasury ETF,NASDAQ Global MarketSM
IBTK,iShares iBonds Dec 2030 Term Treasury ETF,NASDAQ Global MarketSM
IBTL,iShares iBonds Dec 2031 Term Treasury ETF,NASDAQ Global MarketSM
IBTM,iShares iBonds Dec 2032 Term Treasury ETF,NASDAQ Global MarketSM
IBTO,iShares iBonds Dec 2033 Term Treasury ETF,NASDAQ Global MarketSM
IBTX,\"Independent Bank Group, Inc - Common Stock\",NASDAQ Global Select MarketSM
ICAD,icad inc. - Common Stock,NASDAQ Capital Market
ICCC,ImmuCell Corporation - Common Stock,NASDAQ Capital Market
ICCH,\"ICC Holdings, Inc. - Common Stock\",NASDAQ Capital Market
ICCM,IceCure Medical Ltd. - Ordinary Shares,NASDAQ Capital Market
ICCT,iCoreConnect Inc. - Common stock,NASDAQ Capital Market
ICFI,\"ICF International, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ICG,Intchains Group Limited - American Depositary Shares,NASDAQ Capital Market
ICHR,Ichor Holdings - Ordinary Shares,NASDAQ Global Select MarketSM
ICLK,iClick Interactive Asia Group Limited - American Depositary Shares,NASDAQ Global MarketSM
ICLN,iShares Global Clean Energy ETF,NASDAQ Global MarketSM
ICLR,ICON plc - Ordinary Shares,NASDAQ Global Select MarketSM
ICMB,\"Investcorp Credit Management BDC, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ICOP,iShares Copper and Metals Mining ETF,NASDAQ Global MarketSM
ICU,SeaStar Medical Holding Corporation - Common Stock,NASDAQ Capital Market
ICUCW,SeaStar Medical Holding Corporation - Warrant,NASDAQ Capital Market
ICUI,\"ICU Medical, Inc. - Common Stock\",NASDAQ Global Select MarketSM
IDAI,T Stamp Inc. - Class A Common Stock,NASDAQ Capital Market
IDCC,\"InterDigital, Inc. - Common Stock\",NASDAQ Global Select MarketSM
IDEX,\"Ideanomics, Inc. - Common Stock\",NASDAQ Capital Market
IDN,\"Intellicheck, Inc. - Common Stock\",NASDAQ Global MarketSM
IDXX,\"IDEXX Laboratories, Inc. - Common Stock\",NASDAQ Global Select MarketSM
IDYA,\"IDEAYA Biosciences, Inc. - Common Stock\",NASDAQ Global Select MarketSM
IEF,iShares 7-10 Year Treasury Bond ETF,NASDAQ Global MarketSM
IEI,iShares 3-7 Year Treasury Bond ETF,NASDAQ Global MarketSM
IEP,Icahn Enterprises L.P. - Depositary Units representing Limited Partner Interests,NASDAQ Global Select MarketSM
IESC,\"IES Holdings, Inc. - Common Stock\",NASDAQ Global MarketSM
IEUS,iShares MSCI Europe Small-Cap ETF,NASDAQ Global MarketSM
IFBD,\"Infobird Co., Ltd - Ordinary Shares\",NASDAQ Capital Market
IFGL,iShares International Developed Real Estate ETF,NASDAQ Global MarketSM
IFRX,InflaRx N.V. - Common Stock,NASDAQ Global Select MarketSM
IFV,First Trust Dorsey Wright International Focus 5 ETF,NASDAQ Global MarketSM
IGF,iShares Global Infrastructure ETF,NASDAQ Global MarketSM
IGIB,iShares 5-10 Year Investment Grade Corporate Bond ETF,NASDAQ Global MarketSM
IGIC,International General Insurance Holdings Ltd. - Ordinary Shares,NASDAQ Capital Market
IGMS,\"IGM Biosciences, Inc. - Common Stock\",NASDAQ Global Select MarketSM
IGOV,iShares International Treasury Bond ETF,NASDAQ Global MarketSM
IGSB,iShares 1-5 Year Investment Grade Corporate Bond ETF,NASDAQ Global MarketSM
IGTA,Inception Growth Acquisition Limited - Common Stock,NASDAQ Capital Market
IGTAR,Inception Growth Acquisition Limited - Rights,NASDAQ Capital Market
IGTAU,Inception Growth Acquisition Limited - Units,NASDAQ Capital Market
IGTAW,Inception Growth Acquisition Limited - Warrants,NASDAQ Capital Market
IHRT,\"iHeartMedia, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
IHYF,Invesco High Yield Bond Factor ETF,NASDAQ Global MarketSM
III,\"Information Services Group, Inc. - Common Stock\",NASDAQ Global MarketSM
IIIV,\"i3 Verticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
IINN,Inspira Technologies Oxy B.H.N. Ltd. - Ordinary Shares,NASDAQ Capital Market
IINNW,Inspira Technologies Oxy B.H.N. Ltd. - Warrant,NASDAQ Capital Market
IJT,iShares S&P SmallCap 600 Growth ETF,NASDAQ Global MarketSM
IKNA,\"Ikena Oncology, Inc. - Common Stock\",NASDAQ Global MarketSM
IKT,\"Inhibikase Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
ILAG,Intelligent Living Application Group Inc. - Ordinary Shares,NASDAQ Capital Market
ILIT,iShares Lithium Miners and Producers ETF,NASDAQ Global MarketSM
ILMN,\"Illumina, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ILPT,Industrial Logistics Properties Trust - Common Shares of Beneficial Interest,NASDAQ Global Select MarketSM
IMAB,I-MAB - American Depositary Shares,NASDAQ Global MarketSM
IMAQ,International Media Acquisition Corp. - Class A Common Stock,NASDAQ Capital Market
IMAQR,International Media Acquisition Corp. - Rights,NASDAQ Capital Market
IMAQU,International Media Acquisition Corp. - Unit,NASDAQ Capital Market
IMAQW,International Media Acquisition Corp. - Warrants,NASDAQ Capital Market
IMCC,IM Cannabis Corp. - Common Shares,NASDAQ Capital Market
IMCR,Immunocore Holdings plc - American Depositary Shares,NASDAQ Global Select MarketSM
IMCV,iShares Morningstar Mid-Cap Value ETF,NASDAQ Global MarketSM
IMKTA,\"Ingles Markets, Incorporated - Class A Common Stock\",NASDAQ Global Select MarketSM
IMMP,Immutep Limited - American Depositary Shares,NASDAQ Global MarketSM
IMMR,Immersion Corporation - Common Stock,NASDAQ Global Select MarketSM
IMMX,\"Immix Biopharma, Inc. - Common Stock\",NASDAQ Capital Market
IMNM,\"Immunome, Inc. - Common Stock\",NASDAQ Capital Market
IMNN,\"Imunon, Inc. - Common Stock\",NASDAQ Capital Market
IMOM,Alpha Architect International Quantitative Momentum ETF,NASDAQ Global MarketSM
IMOS,ChipMOS TECHNOLOGIES INC. - American Depositary Shares,NASDAQ Global Select MarketSM
IMPP,Imperial Petroleum Inc. - Common Shares,NASDAQ Capital Market
IMPPP,Imperial Petroleum Inc. - 8.75% Series A Cumulative Redeemable Perpetual Preferred Shares,NASDAQ Capital Market
IMRN,Immuron Limited - American Depositary Shares,NASDAQ Capital Market
IMRX,Immuneering Corporation - Class A Common Stock,NASDAQ Global MarketSM
IMTE,Integrated Media Technology Limited - Ordinary Shares,NASDAQ Capital Market
IMTX,Immatics N.V. - Ordinary Shares,NASDAQ Capital Market
IMTXW,Immatics N.V. - Warrants,NASDAQ Capital Market
IMUX,\"Immunic, Inc.  - Common Stock\",NASDAQ Global Select MarketSM
IMVT,\"Immunovant, Inc.  - Common Stock\",NASDAQ Global Select MarketSM
IMXI,\"International Money Express, Inc. - Common Stock\",NASDAQ Capital Market
INAB,\"IN8bio, Inc. - Common Stock\",NASDAQ Global MarketSM
INAQ,Insight Acquisition Corp. - Class A Common Stock,NASDAQ Global MarketSM
INAQU,Insight Acquisition Corp. - Unit,NASDAQ Global MarketSM
INAQW,Insight Acquisition Corp. - Warrant,NASDAQ Capital Market
INBK,First Internet Bancorp - Common Stock,NASDAQ Global Select MarketSM
INBKZ,First Internet Bancorp - Fixed-to-Floating Rate Subordinated Notes Due 2029,NASDAQ Global Select MarketSM
INBS,Intelligent Bio Solutions Inc.  - Common Stock,NASDAQ Capital Market
INBX,\"Inhibrx, Inc. - Common Stock\",NASDAQ Global MarketSM
INCR,Intercure Ltd. - ordinary shares,NASDAQ Global MarketSM
INCY,Incyte Corporation - Common Stock,NASDAQ Global Select MarketSM
INDB,Independent Bank Corp. - Common Stock,NASDAQ Global Select MarketSM
INDH,WisdomTree India Hedged Equity Fund,NASDAQ Global MarketSM
INDI,\"indie Semiconductor, Inc. - Class A Common Stock\",NASDAQ Capital Market
INDP,\"Indaptus Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
INDV,Indivior PLC - Ordinary Shares,NASDAQ Global Select MarketSM
INDY,iShares India 50 ETF,NASDAQ Global MarketSM
INFN,Infinera Corporation - Common Stock,NASDAQ Global Select MarketSM
INFR,ClearBridge Sustainable Infrastructure ETF,NASDAQ Global MarketSM
INGN,\"Inogen, Inc - Common Stock\",NASDAQ Global Select MarketSM
INHD,Inno Holdings Inc. - Common Stock,NASDAQ Capital Market
INKT,\"MiNK Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
INM,InMed Pharmaceuticals Inc. - Common Shares,NASDAQ Capital Market
INMB,INmune Bio Inc. - Common stock,NASDAQ Capital Market
INMD,InMode Ltd.  - Ordinary Shares,NASDAQ Global Select MarketSM
INNV,InnovAge Holding Corp. - Common Stock,NASDAQ Global Select MarketSM
INO,\"Inovio Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
INOD,Innodata Inc. - Common Stock,NASDAQ Global MarketSM
INRO,BlackRock U.S. Industry Rotation ETF,NASDAQ Global MarketSM
INSE,\"Inspired Entertainment, Inc. - Common Stock\",NASDAQ Capital Market
INSG,Inseego Corp. - Common Stock,NASDAQ Global Select MarketSM
INSM,Insmed Incorporated - Common Stock,NASDAQ Global Select MarketSM
INTA,\"Intapp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
INTC,Intel Corporation - Common Stock,NASDAQ Global Select MarketSM
INTE,Integral Acquisition Corporation 1 - Class A Common Stock,NASDAQ Capital Market
INTEU,Integral Acquisition Corporation 1 - Unit,NASDAQ Capital Market
INTEW,Integral Acquisition Corporation 1 - Warrant,NASDAQ Capital Market
INTG,The Intergroup Corporation - Common Stock,NASDAQ Capital Market
INTJ,Intelligent Group Limited - Ordinary Shares,NASDAQ Capital Market
INTR,Inter & Co. Inc. - Class A Common Shares,NASDAQ Global Select MarketSM
INTS,\"Intensity Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
INTU,Intuit Inc. - Common Stock,NASDAQ Global Select MarketSM
INTZ,Intrusion Inc. - Common Stock,NASDAQ Capital Market
INVA,\"Innoviva, Inc. - Common Stock\",NASDAQ Global Select MarketSM
INVE,\"Identiv, Inc. - Common Stock\",NASDAQ Capital Market
INVO,\"INVO BioScience, Inc. - Common Stock\",NASDAQ Capital Market
INVZ,Innoviz Technologies Ltd. - Ordinary shares,NASDAQ Capital Market
INVZW,Innoviz Technologies Ltd. - Warrant,NASDAQ Capital Market
INZY,\"Inozyme Pharma, Inc. - Common Stock\",NASDAQ Global Select MarketSM
IOBT,\"IO Biotech, Inc. - Common Stock\",NASDAQ Global Select MarketSM
IONM,Assure Holdings Corp. - Common Stock,NASDAQ Capital Market
IONR,ioneer Ltd - American Depositary Shares,NASDAQ Capital Market
IONS,\"Ionis Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
IOSP,Innospec Inc. - Common Stock,NASDAQ Global Select MarketSM
IOVA,\"Iovance Biotherapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
IPA,ImmunoPrecise Antibodies Ltd. - Common Stock,NASDAQ Global MarketSM
IPAR,\"Inter Parfums, Inc. - Common Stock\",NASDAQ Global Select MarketSM
IPDN,\"Professional Diversity Network, Inc. - Common Stock\",NASDAQ Capital Market
IPGP,IPG Photonics Corporation - Common Stock,NASDAQ Global Select MarketSM
IPHA,Innate Pharma S.A. - American Depositary Shares,NASDAQ Global Select MarketSM
IPKW,Invesco International BuyBack Achievers ETF,NASDAQ Global MarketSM
IPSC,\"Century Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
IPW,iPower Inc. - Common Stock,NASDAQ Capital Market
IPWR,Ideal Power Inc. - Common Stock,NASDAQ Capital Market
IPX,IperionX Limited - American Depositary Share,NASDAQ Capital Market
IPXX,Inflection Point Acquisition Corp. II - Class A Ordinary Shares,NASDAQ Global MarketSM
IPXXU,Inflection Point Acquisition Corp. II - Unit,NASDAQ Global MarketSM
IPXXW,Inflection Point Acquisition Corp. II - Warrant,NASDAQ Global MarketSM
IQ,\"iQIYI, Inc. - American Depositary Shares\",NASDAQ Global Select MarketSM
IQQQ,ProShares Nasdaq-100 High Income ETF,NASDAQ Global MarketSM
IRAA,Iris Acquisition Corp - Class A Common Stock,NASDAQ Capital Market
IRAAU,Iris Acquisition Corp - Units,NASDAQ Capital Market
IRAAW,Iris Acquisition Corp - Warrant,NASDAQ Capital Market
IRBT,iRobot Corporation - Common Stock,NASDAQ Global Select MarketSM
IRDM,Iridium Communications Inc - Common Stock,NASDAQ Global Select MarketSM
IREN,Iris Energy Limited - Ordinary Shares,NASDAQ Global Select MarketSM
IRIX,IRIDEX Corporation - Common Stock,NASDAQ Global MarketSM
IRMD,iRadimed Corporation - Common Stock,NASDAQ Capital Market
IROH,Iron Horse Acquisitions Corp. - Common Stock,NASDAQ Global MarketSM
IROHR,Iron Horse Acquisitions Corp. - one right to one-fifth (1/5) of one share of common stock,NASDAQ Global MarketSM
IROHU,Iron Horse Acquisitions Corp. - Unit,NASDAQ Global MarketSM
IROHW,Iron Horse Acquisitions Corp. - Warrant,NASDAQ Global MarketSM
IRON,\"Disc Medicine, Inc. - Common Stock\",NASDAQ Global MarketSM
IROQ,\"IF Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
IRTC,\"iRhythm Technologies, Inc. - Common Stock\",NASDAQ Global Select MarketSM
IRWD,\"Ironwood Pharmaceuticals, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
ISHG,iShares 1-3 Year International Treasury Bond ETF,NASDAQ Global MarketSM
ISHP,First Trust S-Network E-Commerce ETF,NASDAQ Global MarketSM
ISPC,iSpecimen Inc. - Common Stock,NASDAQ Capital Market
ISPO,Inspirato Incorporated - Class A Common Stock,NASDAQ Global MarketSM
ISPOW,Inspirato Incorporated - Warrant,NASDAQ Global MarketSM
ISPR,Ispire Technology Inc. - Common Stock,NASDAQ Capital Market
ISRG,\"Intuitive Surgical, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ISRL,Israel Acquisitions Corp - Class A Ordinary Shares,NASDAQ Global MarketSM
ISRLU,Israel Acquisitions Corp - Unit,NASDAQ Global MarketSM
ISRLW,Israel Acquisitions Corp - Warrant,NASDAQ Global MarketSM
ISSC,\"Innovative Solutions and Support, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ISTB,iShares Core 1-5 Year USD Bond ETF,NASDAQ Global MarketSM
ISTR,Investar Holding Corporation - Common Stock,NASDAQ Global MarketSM
ISUN,\"iSun, Inc. - Common Stock\",NASDAQ Capital Market
ITCI,Intra-Cellular Therapies Inc. - Common Stock,NASDAQ Global Select MarketSM
ITI,\"Iteris, Inc. - Common Stock\",NASDAQ Capital Market
ITIC,Investors Title Company - Common Stock,NASDAQ Global Select MarketSM
ITOS,\"iTeos Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
ITRI,\"Itron, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ITRM,Iterum Therapeutics plc - Ordinary Share,NASDAQ Capital Market
ITRN,Ituran Location and Control Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
IUS,Invesco RAFI Strategic US ETF,NASDAQ Global MarketSM
IUSB,iShares Core Total USD Bond Market ETF,NASDAQ Global MarketSM
IUSG,iShares Core S&P U.S. Growth ETF,NASDAQ Global MarketSM
IUSV,iShares Core S&P U.S. Value ETF,NASDAQ Global MarketSM
IVA,Inventiva S.A. - American Depository Shares,NASDAQ Global MarketSM
IVAC,\"Intevac, Inc. - Common Stock\",NASDAQ Global Select MarketSM
IVAL,Alpha Architect International Quantitative Value ETF,NASDAQ Global MarketSM
IVCA,Investcorp India Acquisition Corp. - Class A Ordinary Shares,NASDAQ Global MarketSM
IVCAU,Investcorp India Acquisition Corp. - Unit,NASDAQ Global MarketSM
IVCAW,Investcorp India Acquisition Corp. - Warrant,NASDAQ Global MarketSM
IVCB,Investcorp Europe Acquisition Corp I - Class A Ordinary Shares,NASDAQ Global MarketSM
IVCBU,Investcorp Europe Acquisition Corp I - Unit,NASDAQ Global MarketSM
IVCBW,Investcorp Europe Acquisition Corp I - Warrant,NASDAQ Global MarketSM
IVCP,Swiftmerge Acquisition Corp. - Class A Ordinary Share,NASDAQ Capital Market
IVCPU,Swiftmerge Acquisition Corp. - Unit,NASDAQ Capital Market
IVCPW,Swiftmerge Acquisition Corp. - Warrants,NASDAQ Capital Market
IVDA,\"Iveda Solutions, Inc. - Common Stock\",NASDAQ Capital Market
IVDAW,\"Iveda Solutions, Inc. - Warrant\",NASDAQ Capital Market
IVEG,iShares Emergent Food and AgTech Multisector ETF,NASDAQ Global MarketSM
IVP,\"Inspire Veterinary Partners, Inc. - Class A Common Stock\",NASDAQ Capital Market
IVVD,\"Invivyd, Inc. - Common Stock\",NASDAQ Global MarketSM
IWTR,iShares MSCI Water Management Multisector ETF,NASDAQ Global MarketSM
IXAQ,IX Acquisition Corp. - Class A Ordinary Share,NASDAQ Global MarketSM
IXAQU,IX Acquisition Corp. - Unit,NASDAQ Global MarketSM
IXAQW,IX Acquisition Corp. - Warrant,NASDAQ Global MarketSM
IXHL,Incannex Healthcare Inc. - Common Stock,NASDAQ Global MarketSM
IXUS,iShares Core MSCI Total International Stock ETF,NASDAQ Global MarketSM
IZEA,\"IZEA Worldwide, Inc. - Common Stock\",NASDAQ Capital Market
IZM,ICZOOM Group Inc. - Class A Ordinary Shares,NASDAQ Capital Market
JACK,Jack In The Box Inc. - Common Stock,NASDAQ Global Select MarketSM
JAGX,\"Jaguar Health, Inc. - Common Stock\",NASDAQ Capital Market
JAKK,\"JAKKS Pacific, Inc. - Common Stock\",NASDAQ Global Select MarketSM
JAMF,Jamf Holding Corp. - Common Stock,NASDAQ Global Select MarketSM
JAN,JanOne Inc. - Common Stock,NASDAQ Capital Market
JANX,\"Janux Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
JAZZ,Jazz Pharmaceuticals plc - Ordinary Shares,NASDAQ Global Select MarketSM
JBHT,\"J.B. Hunt Transport Services, Inc. - Common Stock\",NASDAQ Global Select MarketSM
JBLU,JetBlue Airways Corporation - Common Stock,NASDAQ Global Select MarketSM
JBSS,\"John B. Sanfilippo & Son, Inc. - Common Stock\",NASDAQ Global Select MarketSM
JCSE,JE Cleantech Holdings Limited - Ordinary Shares,NASDAQ Capital Market
JCTCF,Jewett-Cameron Trading Company - Common Shares,NASDAQ Capital Market
JD,\"JD.com, Inc. - American Depositary Shares\",NASDAQ Global Select MarketSM
JDOC,JPMorgan Healthcare Leaders ETF,NASDAQ Global MarketSM
JDZG,JIADE LIMITED - Common stock,NASDAQ Capital Market
JEPQ,JPMorgan Nasdaq Equity Premium Income ETF,NASDAQ Global MarketSM
JEWL,Adamas One Corp. - Common Stock,NASDAQ Capital Market
JFBR,Jeffs' Brands Ltd - Ordinary Shares,NASDAQ Capital Market
JFBRW,Jeffs' Brands Ltd - Warrant,NASDAQ Capital Market
JFIN,Jiayin Group Inc. - American Depositary Shares,NASDAQ Global MarketSM
JFU,9F Inc. - American Depositary Shares,NASDAQ Global MarketSM
JG,Aurora Mobile Limited - American Depositary Shares,NASDAQ Capital Market
JGLO,JPMorgan Global Select Equity ETF,NASDAQ Global MarketSM
JIVE,JPMorgan International Value ETF,NASDAQ Global MarketSM
JJSF,J & J Snack Foods Corp. - Common Stock,NASDAQ Global Select MarketSM
JKHY,\"Jack Henry & Associates, Inc. - Common Stock\",NASDAQ Global Select MarketSM
JL,J-Long Group Limited - Ordinary Shares,NASDAQ Global MarketSM
JMSB,\"John Marshall Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
JNVR,Janover Inc. - Common Stock,NASDAQ Capital Market
JOUT,Johnson Outdoors Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
JPEF,JPMorgan Equity Focus ETF,NASDAQ Global MarketSM
JRSH,\"Jerash Holdings (US), Inc. - Common Stock\",NASDAQ Capital Market
JRVR,\"James River Group Holdings, Ltd. - Common Shares\",NASDAQ Global Select MarketSM
JSM,\"Navient Corporation - 6% Senior Notes due December 15, 2043\",NASDAQ Global Select MarketSM
JSMD,Janus Henderson Small/Mid Cap Growth Alpha ETF,NASDAQ Global MarketSM
JSML,Janus Henderson Small Cap Growth Alpha ETF,NASDAQ Global MarketSM
JSPR,\"Jasper Therapeutics, Inc. - Class A Common Stock\",NASDAQ Capital Market
JSPRW,\"Jasper Therapeutics, Inc. - Warrant\",NASDAQ Capital Market
JTAI,Jet.AI Inc. - Common Stock,NASDAQ Global MarketSM
JTAIW,Jet.AI Inc. - Warrant,NASDAQ Capital Market
JTAIZ,Jet.AI Inc. - Merger Consideration Warrants,NASDAQ Global MarketSM
JTEK,JPMorgan U.S. Tech Leaders ETF,NASDAQ Global MarketSM
JUNE,Junee Limited - Ordinary Shares,NASDAQ Capital Market
JVA,\"Coffee Holding Co., Inc. - Common Stock\",NASDAQ Capital Market
JVSA,JVSPAC Acquisition Corp. - Class A Ordinary Share,NASDAQ Capital Market
JVSAR,JVSPAC Acquisition Corp. - Right,NASDAQ Capital Market
JVSAU,JVSPAC Acquisition Corp. - Unit,NASDAQ Capital Market
JWEL,Jowell Global Ltd. - Ordinary Shares,NASDAQ Capital Market
JXJT,JX Luxventure Limited - Common Stock,NASDAQ Capital Market
JYD,Jayud Global Logistics Limited - Class A Ordinary Shares,NASDAQ Capital Market
JYNT,The Joint Corp. - Common Stock,NASDAQ Capital Market
JZ,Jianzhi Education Technology Group Company Limited - American Depositary Shares,NASDAQ Global Select MarketSM
JZXN,\"Jiuzi Holdings, Inc. - Ordinary Shares\",NASDAQ Capital Market
KA,\"Kineta, Inc. - Common Stock\",NASDAQ Capital Market
KACL,Kairous Acquisition Corp. Limited - Ordinary Shares,NASDAQ Capital Market
KACLR,Kairous Acquisition Corp. Limited - Rights,NASDAQ Capital Market
KACLU,Kairous Acquisition Corp. Limited - Unit,NASDAQ Capital Market
KACLW,Kairous Acquisition Corp. Limited - Warrants,NASDAQ Capital Market
KALA,\"KALA BIO, Inc. - Common Stock\",NASDAQ Capital Market
KALU,Kaiser Aluminum Corporation - Common Stock,NASDAQ Global Select MarketSM
KALV,\"KalVista Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global MarketSM
KARO,Karooooo Ltd. - Ordinary shares,NASDAQ Capital Market
KAVL,\"Kaival Brands Innovations Group, Inc. - Common Stock\",NASDAQ Capital Market
KBWB,Invesco KBW Bank ETF,NASDAQ Global MarketSM
KBWD,Invesco KBW High Dividend Yield Financial ETF,NASDAQ Global MarketSM
KBWP,Invesco KBW Property & Casualty Insurance ETF,NASDAQ Global MarketSM
KBWR,Invesco KBW Regional Banking ETF,NASDAQ Global MarketSM
KBWY,Invesco KBW Premium Yield Equity REIT ETF,NASDAQ Global MarketSM
KC,Kingsoft Cloud Holdings Limited - American Depositary Shares,NASDAQ Global Select MarketSM
KDP,Keurig Dr Pepper Inc. - Common Stock,NASDAQ Global Select MarketSM
KE,\"Kimball Electronics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
KEAT,Keating Active ETF,NASDAQ Global MarketSM
KELYA,\"Kelly Services, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
KELYB,\"Kelly Services, Inc. - Class B Common Stock\",NASDAQ Global Select MarketSM
KEQU,Kewaunee Scientific Corporation - Common Stock,NASDAQ Global MarketSM
KFFB,Kentucky First Federal Bancorp - Common Stock,NASDAQ Global MarketSM
KGEI,Kolibri Global Energy Inc. - Common stock,NASDAQ Capital Market
KHC,The Kraft Heinz Company - Common Stock,NASDAQ Global Select MarketSM
KIDS,OrthoPediatrics Corp. - Common Stock,NASDAQ Global MarketSM
KINS,\"Kingstone Companies, Inc - Common Stock\",NASDAQ Capital Market
KIRK,\"Kirkland's, Inc. - Common Stock\",NASDAQ Global Select MarketSM
KITT,\"Nauticus Robotics, Inc. - Common stock\",NASDAQ Capital Market
KITTW,\"Nauticus Robotics, Inc. - Warrant\",NASDAQ Capital Market
KLAC,KLA Corporation  - Common Stock,NASDAQ Global Select MarketSM
KLIC,\"Kulicke and Soffa Industries, Inc. - Common Stock\",NASDAQ Global Select MarketSM
KLTR,\"Kaltura, Inc. - Common Stock\",NASDAQ Global Select MarketSM
KLXE,\"KLX Energy Services Holdings, Inc.  - Common Stock\",NASDAQ Global Select MarketSM
KMDA,Kamada Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
KNDI,\"Kandi Technologies Group, Inc. - Ordinary Shares\",NASDAQ Global Select MarketSM
KNGZ,First Trust S&P 500 Diversified Dividend Aristocrats ETF,NASDAQ Global MarketSM
KNSA,\"Kiniksa Pharmaceuticals, Ltd. - Class A Common Stock\",NASDAQ Global Select MarketSM
KOD,Kodiak Sciences Inc - Common Stock,NASDAQ Global MarketSM
KOPN,Kopin Corporation - Common Stock,NASDAQ Capital Market
KOSS,Koss Corporation - Common Stock,NASDAQ Capital Market
KPLT,\"Katapult Holdings, Inc. - Common Stock\",NASDAQ Global MarketSM
KPLTW,\"Katapult Holdings, Inc. - Warrant\",NASDAQ Global MarketSM
KPRX,\"Kiora Pharmaceuticals, Inc.  - Common Stock\",NASDAQ Capital Market
KPTI,Karyopharm Therapeutics Inc. - Common Stock,NASDAQ Global Select MarketSM
KRKR,36Kr Holdings Inc. - American Depositary Shares,NASDAQ Capital Market
KRMA,Global X Conscious Companies ETF,NASDAQ Global MarketSM
KRMD,\"KORU Medical Systems, Inc. - Common Stock\",NASDAQ Capital Market
KRNL,\"Kernel Group Holdings, Inc. - Class A Ordinary Shares\",NASDAQ Capital Market
KRNLU,\"Kernel Group Holdings, Inc. - Units\",NASDAQ Capital Market
KRNLW,\"Kernel Group Holdings, Inc. - Warrants\",NASDAQ Capital Market
KRNT,Kornit Digital Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
KRNY,Kearny Financial - Common Stock,NASDAQ Global Select MarketSM
KRON,\"Kronos Bio, Inc. - Common Stock\",NASDAQ Global Select MarketSM
KROP,Global X AgTech & Food Innovation ETF,NASDAQ Global MarketSM
KROS,\"Keros Therapeutics, Inc. - common stock\",NASDAQ Global MarketSM
KRRO,\"Korro Bio, Inc. - Common Stock\",NASDAQ Capital Market
KRT,Karat Packaging Inc. - Common Stock,NASDAQ Global Select MarketSM
KRUS,\"Kura Sushi USA, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
KRYS,\"Krystal Biotech, Inc. - Common Stock\",NASDAQ Global Select MarketSM
KSCP,\"Knightscope, Inc. - Class A Common Stock\",NASDAQ Capital Market
KSPI,Joint Stock Company Kaspi.kz - American Depository Shares,NASDAQ Global Select MarketSM
KTCC,Key Tronic Corporation - Common Stock,NASDAQ Global MarketSM
KTOS,\"Kratos Defense & Security Solutions, Inc. - Common Stock\",NASDAQ Global Select MarketSM
KTRA,\"Kintara Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
KTTA,Pasithea Therapeutics Corp. - Common Stock,NASDAQ Capital Market
KTTAW,Pasithea Therapeutics Corp. - Warrant,NASDAQ Capital Market
KURA,\"Kura Oncology, Inc. - Common Stock\",NASDAQ Global Select MarketSM
KVAC,Keen Vision Acquisition Corporation - Ordinary Shares,NASDAQ Global MarketSM
KVACU,Keen Vision Acquisition Corporation - Units,NASDAQ Global MarketSM
KVACW,Keen Vision Acquisition Corporation - Warrant,NASDAQ Global MarketSM
KVHI,\"KVH Industries, Inc. - Common Stock\",NASDAQ Global Select MarketSM
KWE,\"KWESST Micro Systems Inc. - common stock, no R/S concurrent with offering\",NASDAQ Capital Market
KWESW,KWESST Micro Systems Inc. - warrant,NASDAQ Capital Market
KXIN,Kaixin Holdings - Ordinary Shares,NASDAQ Capital Market
KYMR,\"Kymera Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
KYTX,\"Kyverna Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
KZIA,Kazia Therapeutics Limited - American Depositary Shares,NASDAQ Capital Market
KZR,\"Kezar Life Sciences, Inc. - Common Stock\",NASDAQ Global Select MarketSM
LAB,Standard BioTools Inc. - Common Stock,NASDAQ Global Select MarketSM
LABP,\"Landos Biopharma, Inc. - Common Stock\",NASDAQ Capital Market
LAES,SEALSQ Corp - Ordinary Shares,NASDAQ Capital Market
LAKE,\"Lakeland Industries, Inc. - Common Stock\",NASDAQ Global MarketSM
LAMR,Lamar Advertising Company - Class A Common Stock,NASDAQ Global Select MarketSM
LANC,Lancaster Colony Corporation - Common Stock,NASDAQ Global Select MarketSM
LAND,Gladstone Land Corporation - Common Stock,NASDAQ Global MarketSM
LANDM,Gladstone Land Corporation - 5.00% Series D Cumulative Term Preferred Stock,NASDAQ Global MarketSM
LANDO,Gladstone Land Corporation - 6.00% Series B Cumulative Redeemable Preferred Stock,NASDAQ Global MarketSM
LANDP,Gladstone Land Corporation - 6.00% Series C Cumulative Redeemable Preferred Stock,NASDAQ Global MarketSM
LARK,Landmark Bancorp Inc. - Common Stock,NASDAQ Global MarketSM
LASE,Laser Photonics Corporation - Common Stock,NASDAQ Capital Market
LASR,\"nLIGHT, Inc. - Common Stock\",NASDAQ Global Select MarketSM
LATG,Chenghe Acquisition I Co. - Class A Ordinary Shares,NASDAQ Global MarketSM
LATGU,Chenghe Acquisition I Co. - Units,NASDAQ Global MarketSM
LAUR,\"Laureate Education, Inc. - Common Stock\",NASDAQ Global Select MarketSM
LAZR,\"Luminar Technologies, Inc.  - Class A Common Stock\",NASDAQ Global Select MarketSM
LBPH,\"Longboard Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global MarketSM
LBRDA,Liberty Broadband Corporation - Class A Common Stock,NASDAQ Global Select MarketSM
LBRDK,Liberty Broadband Corporation - Class C Common Stock,NASDAQ Global Select MarketSM
LBRDP,Liberty Broadband Corporation - Series A Cumulative Redeemable Preferred Stock,NASDAQ Global Select MarketSM
LBTYA,Liberty Global Ltd. - Class A Common Shares,NASDAQ Global Select MarketSM
LBTYB,Liberty Global Ltd. - Class B Common Shares,NASDAQ Global Select MarketSM
LBTYK,Liberty Global Ltd. - Class C Common Shares,NASDAQ Global Select MarketSM
LCFY,Locafy Limited - Ordinary Share,NASDAQ Capital Market
LCFYW,Locafy Limited - Warrant,NASDAQ Capital Market
LCID,\"Lucid Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
LCNB,LCNB Corporation - Common Stock,NASDAQ Capital Market
LCUT,\"Lifetime Brands, Inc. - Common Stock\",NASDAQ Global Select MarketSM
LDEM,iShares ESG MSCI EM Leaders ETF,NASDAQ Global MarketSM
LDSF,First Trust Low Duration Strategic Focus ETF,NASDAQ Global MarketSM
LDTC,LeddarTech Holdings Inc. - Common Shares,NASDAQ Global MarketSM
LDTCW,LeddarTech Holdings Inc. - Warrants,NASDAQ Global MarketSM
LDWY,\"Lendway, Inc. - Common Stock\",NASDAQ Capital Market
LE,\"Lands' End, Inc. - Common Stock\",NASDAQ Capital Market
LECO,\"Lincoln Electric Holdings, Inc. - Common Shares\",NASDAQ Global Select MarketSM
LEDS,SemiLEDS Corporation - Common Stock,NASDAQ Capital Market
LEE,\"Lee Enterprises, Incorporated - Common Stock\",NASDAQ Global Select MarketSM
LEGH,Legacy Housing Corporation - Common Stock,NASDAQ Global Select MarketSM
LEGN,Legend Biotech Corporation - American Depositary Shares,NASDAQ Global Select MarketSM
LEGR,First Trust Indxx Innovative Transaction & Process ETF,NASDAQ Global MarketSM
LENZ,\"LENZ Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
LESL,\"Leslie's, Inc. - Common Stock\",NASDAQ Global Select MarketSM
LEXX,Lexaria Bioscience Corp. - Common Stock,NASDAQ Capital Market
LEXXW,Lexaria Bioscience Corp. - Warrant,NASDAQ Capital Market
LFCR,\"Lifecore Biomedical, Inc. - Common Stock\",NASDAQ Global Select MarketSM
LFLY,\"Leafly Holdings, Inc. - Common Stock\",NASDAQ Capital Market
LFLYW,\"Leafly Holdings, Inc. - Warrant\",NASDAQ Capital Market
LFMD,\"LifeMD, Inc. - Common Stock\",NASDAQ Global MarketSM
LFMDP,\"LifeMD, Inc. - 8.875% Series A Cumulative Perpetual Preferred Stock\",NASDAQ Global MarketSM
LFST,\"LifeStance Health Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
LFUS,\"Littelfuse, Inc. - Common Stock\",NASDAQ Global Select MarketSM
LFVN,Lifevantage Corporation - Common Stock,NASDAQ Capital Market
LFWD,ReWalk Robotics Ltd. - Ordinary Shares,NASDAQ Capital Market
LGCB,Linkage Global Inc - Ordinary Shares,NASDAQ Capital Market
LGCL,Lucas GC Limited - Ordinary Shares,NASDAQ Capital Market
LGHL,Lion Group Holding Ltd. - American Depositary Share,NASDAQ Capital Market
LGHLW,Lion Group Holding Ltd. - Warrant,NASDAQ Capital Market
LGIH,\"LGI Homes, Inc. - Common Stock\",NASDAQ Global Select MarketSM
LGMK,\"LogicMark, Inc. - Common Stock\",NASDAQ Capital Market
LGND,Ligand Pharmaceuticals Incorporated - Common Stock,NASDAQ Global MarketSM
LGO,Largo Inc. - Common Shares,NASDAQ Global Select MarketSM
LGRO,Level Four Large Cap Growth Active ETF,NASDAQ Global MarketSM
LGVN,Longeveron Inc. - Class A Common stock,NASDAQ Capital Market
LI,Li Auto Inc. - American Depositary Shares,NASDAQ Global Select MarketSM
LICN,Lichen China Limited - Class A Ordinary Shares,NASDAQ Capital Market
LIDR,\"AEye, Inc. - Class A Common Stock\",NASDAQ Capital Market
LIDRW,\"AEye, Inc. - Warrant\",NASDAQ Capital Market
LIFE,\"aTyr Pharma, Inc. - Common Stock\",NASDAQ Capital Market
LIFW,\"MSP Recovery, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
LIFWW,\"MSP Recovery, Inc. - Warrant\",NASDAQ Global MarketSM
LIFWZ,\"MSP Recovery, Inc. - Warrant\",NASDAQ Global MarketSM
LILA,Liberty Latin America Ltd. - Class A Common Stock,NASDAQ Global Select MarketSM
LILAK,Liberty Latin America Ltd. - Class C Common Stock,NASDAQ Global Select MarketSM
LILM,Lilium N.V. - Class A Ordinary Shares,NASDAQ Global Select MarketSM
LILMW,Lilium N.V. - Warrants,NASDAQ Global Select MarketSM
LIN,Linde plc - Ordinary Shares,NASDAQ Global Select MarketSM
LINC,Lincoln Educational Services Corporation - Common Stock,NASDAQ Global Select MarketSM
LIND,Lindblad Expeditions Holdings Inc.  - Common Stock,NASDAQ Capital Market
LINK,\"Interlink Electronics, Inc. - Common Stock\",NASDAQ Capital Market
LION,Lionsgate Studios Corp. - Common Shares,NASDAQ Global Select MarketSM
LIPO,Lipella Pharmaceuticals Inc. - Common Stock,NASDAQ Capital Market
LIQT,\"LiqTech International, Inc. - Common Stock\",NASDAQ Capital Market
LITE,Lumentum Holdings Inc. - Common Stock,NASDAQ Global Select MarketSM
LITM,Snow Lake Resources Ltd. - Common Shares,NASDAQ Capital Market
LITP,Sprott Lithium Miners ETF,NASDAQ Global MarketSM
LIVE,Live Ventures Incorporated - Common Stock,NASDAQ Capital Market
LIVN,LivaNova PLC - Ordinary Shares,NASDAQ Global Select MarketSM
LIXT,\"Lixte Biotechnology Holdings, Inc. - Common Stock\",NASDAQ Capital Market
LIXTW,\"Lixte Biotechnology Holdings, Inc. - Warrants\",NASDAQ Capital Market
LKCO,Luokung Technology Corp - Ordinary Shares,NASDAQ Capital Market
LKFN,Lakeland Financial Corporation - Common Stock,NASDAQ Global Select MarketSM
LKQ,LKQ Corporation - Common Stock,NASDAQ Global Select MarketSM
LLYVA,Liberty Media Corporation - Series A Liberty Live Common Stock,NASDAQ Global Select MarketSM
LLYVK,Liberty Media Corporation - Series C Liberty Live Common Stock,NASDAQ Global Select MarketSM
LMAT,\"LeMaitre Vascular, Inc. - Common Stock\",NASDAQ Global MarketSM
LMB,\"Limbach Holdings, Inc. - Common Stock\",NASDAQ Capital Market
LMBS,First Trust Low Duration Opportunities ETF,NASDAQ Global MarketSM
LMFA,\"LM Funding America, Inc. - Common Stock\",NASDAQ Capital Market
LMNR,Limoneira Co - Common Stock,NASDAQ Global Select MarketSM
LNKB,\"LINKBANCORP, Inc. - Common Stock\",NASDAQ Capital Market
LNSR,\"LENSAR, Inc. - Common Stock\",NASDAQ Capital Market
LNT,Alliant Energy Corporation - Common Stock,NASDAQ Global Select MarketSM
LNTH,\"Lantheus Holdings, Inc. - Common Stock\",NASDAQ Global MarketSM
LNW,\"Light & Wonder, Inc.  - Common Stock\",NASDAQ Global Select MarketSM
LNZA,\"LanzaTech Global, Inc. - Common Stock\",NASDAQ Capital Market
LNZAW,\"LanzaTech Global, Inc. - Warrant\",NASDAQ Capital Market
LOAN,\"Manhattan Bridge Capital, Inc - Common Stock\",NASDAQ Capital Market
LOBO,LOBO EV TECHNOLOGIES LTD. - Ordinary shares,NASDAQ Capital Market
LOCO,\"El Pollo Loco Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
LOGC,ContextLogic Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
LOGI,Logitech International S.A. - Registered Shares,NASDAQ Global Select MarketSM
LOOP,\"Loop Industries, Inc. - Common Stock\",NASDAQ Global MarketSM
LOPE,\"Grand Canyon Education, Inc. - Common Stock\",NASDAQ Global Select MarketSM
LOT,Lotus Technology Inc. - American Depositary Shares,NASDAQ Global Select MarketSM
LOTWW,Lotus Technology Inc. - Warrants,NASDAQ Capital Market
LOVE,The Lovesac Company - Common Stock,NASDAQ Global MarketSM
LPCN,Lipocine Inc. - Common Stock,NASDAQ Capital Market
LPLA,LPL Financial Holdings Inc. - Common Stock,NASDAQ Global Select MarketSM
LPRO,Open Lending Corporation - Common Stock,NASDAQ Global MarketSM
LPSN,\"LivePerson, Inc. - Common Stock\",NASDAQ Global Select MarketSM
LPTH,\"LightPath Technologies, Inc. - Class A Common Stock\",NASDAQ Capital Market
LPTX,\"Leap Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
LQDA,Liquidia Corporation - Common Stock,NASDAQ Capital Market
LQDT,\"Liquidity Services, Inc. - Common Stock\",NASDAQ Global Select MarketSM
LQR,LQR House Inc. - Common Stock,NASDAQ Capital Market
LRCX,Lam Research Corporation - Common Stock,NASDAQ Global Select MarketSM
LRE,\"Lead Real Estate Co., Ltd - American Depositary Shares\",NASDAQ Global MarketSM
LRFC,Logan Ridge Finance Corporation - Common Stock,NASDAQ Global Select MarketSM
LRGE,ClearBridge Large Cap Growth ESG ETF,NASDAQ Global MarketSM
LRHC,La Rosa Holdings Corp. - Common Stock,NASDAQ Capital Market
LRMR,\"Larimar Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
LRND,IQ U.S. Large Cap R&D Leaders ETF,NASDAQ Global MarketSM
LSAK,\"Lesaka Technologies, Inc. - Common Stock\",NASDAQ Global Select MarketSM
LSBK,\"Lake Shore Bancorp, Inc. - Common Stock\",NASDAQ Global MarketSM
LSCC,Lattice Semiconductor Corporation - Common Stock,NASDAQ Global Select MarketSM
LSDI,Lucy Scientific Discovery Inc. - Common Stock,NASDAQ Capital Market
LSEA,Landsea Homes Corporation - Common Stock,NASDAQ Capital Market
LSEAW,Landsea Homes Corporation - Warrant,NASDAQ Capital Market
LSTA,\"Lisata Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
LSTR,\"Landstar System, Inc. - Common Stock\",NASDAQ Global Select MarketSM
LSXMA,Liberty Media Corporation - Series A Liberty SiriusXM Common Stock,NASDAQ Global Select MarketSM
LSXMB,Liberty Media Corporation - Series B Liberty SiriusXM Common Stock,NASDAQ Global Select MarketSM
LSXMK,Liberty Media Corporation - Series C Liberty SiriusXM Common Stock,NASDAQ Global Select MarketSM
LTBR,Lightbridge Corporation - Common Stock,NASDAQ Capital Market
LTRN,Lantern Pharma Inc. - Common Stock,NASDAQ Capital Market
LTRX,\"Lantronix, Inc. - Common Stock\",NASDAQ Capital Market
LTRY,\"Lottery.com, Inc. - Common Stock\",NASDAQ Global MarketSM
LTRYW,\"Lottery.com, Inc. - Warrant\",NASDAQ Global MarketSM
LUCD,Lucid Diagnostics Inc. - Common Stock,NASDAQ Capital Market
LUCY,\"Innovative Eyewear, Inc. - Common Stock\",NASDAQ Capital Market
LUCYW,\"Innovative Eyewear, Inc. - Warrants\",NASDAQ Capital Market
LULU,lululemon athletica inc. - Common Stock,NASDAQ Global Select MarketSM
LUMO,\"Lumos Pharma, Inc. - Common Stock\",NASDAQ Global MarketSM
LUNA,Luna Innovations Incorporated - Common Stock,NASDAQ Capital Market
LUNG,Pulmonx Corporation - Common Stock,NASDAQ Global Select MarketSM
LUNR,\"Intuitive Machines, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
LUNRW,\"Intuitive Machines, Inc. - Warrants\",NASDAQ Capital Market
LUXH,LuxUrban Hotels Inc. - Common Stock,NASDAQ Capital Market
LUXHP,LuxUrban Hotels Inc. - 13.00% Series A Cumulative Redeemable Preferred Stock,NASDAQ Capital Market
LVHD,Franklin U.S. Low Volatility High Dividend Index ETF,NASDAQ Global MarketSM
LVLU,\"Lulu's Fashion Lounge Holdings, Inc. - Common Stock\",NASDAQ Global MarketSM
LVO,\"LiveOne, Inc. - Common Stock\",NASDAQ Capital Market
LVRO,Lavoro Limited - Class A Ordinary Shares,NASDAQ Global MarketSM
LVROW,Lavoro Limited - Warrant,NASDAQ Global MarketSM
LVTX,LAVA Therapeutics N.V. - Ordinary Shares,NASDAQ Global Select MarketSM
LWAY,\"Lifeway Foods, Inc. - Common Stock\",NASDAQ Global MarketSM
LWLG,\"Lightwave Logic, Inc. - Common Stock\",NASDAQ Capital Market
LX,LexinFintech Holdings Ltd. - American Depositary Shares,NASDAQ Global Select MarketSM
LXEH,\"Lixiang Education Holding Co., Ltd. - American Depositary Shares\",NASDAQ Global MarketSM
LXEO,\"Lexeo Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
LXRX,\"Lexicon Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
LYEL,\"Lyell Immunopharma, Inc. - Common Stock\",NASDAQ Global Select MarketSM
LYFT,\"Lyft, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
LYRA,\"Lyra Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
LYT,Lytus Technologies Holdings PTV. Ltd. - Common Shares,NASDAQ Capital Market
LYTS,LSI Industries Inc. - Common Stock,NASDAQ Global Select MarketSM
LZ,\"LegalZoom.com, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MACA,Moringa Acquisition Corp - Class A Ordinary Shares,NASDAQ Capital Market
MACAU,Moringa Acquisition Corp - Units,NASDAQ Capital Market
MACAW,Moringa Acquisition Corp - Warrant,NASDAQ Capital Market
MAGQ,Roundhill Daily Inverse Magnificent Seven ETF,NASDAQ Global MarketSM
MAGS,Roundhill Magnificent Seven ETF,NASDAQ Global MarketSM
MAGX,Roundhill Daily 2X Long Magnificent Seven ETF,NASDAQ Global MarketSM
MAMA,\"Mama's Creations, Inc. - Common Stock\",NASDAQ Capital Market
MAMO,Massimo Group - Common Stock,NASDAQ Capital Market
MANH,\"Manhattan Associates, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MAPS,\"WM Technology, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
MAPSW,\"WM Technology, Inc. - Warrants\",NASDAQ Global Select MarketSM
MAQC,Maquia Capital Acquisition Corporation - Class A Common Stock,NASDAQ Capital Market
MAQCU,Maquia Capital Acquisition Corporation - Unit,NASDAQ Capital Market
MAQCW,Maquia Capital Acquisition Corporation - Warrant,NASDAQ Capital Market
MAR,Marriott International - Class A Common Stock,NASDAQ Global Select MarketSM
MARA,\"Marathon Digital Holdings, Inc. - Common Stock\",NASDAQ Capital Market
MARPS,Marine Petroleum Trust - Units of Beneficial Interest,NASDAQ Capital Market
MARX,Mars Acquisition Corp. - Ordinary Shares,NASDAQ Global MarketSM
MARXR,Mars Acquisition Corp. - Rights,NASDAQ Global MarketSM
MARXU,Mars Acquisition Corp. - Unit,NASDAQ Global MarketSM
MASI,Masimo Corporation - Common Stock,NASDAQ Global Select MarketSM
MASS,908 Devices Inc. - Common Stock,NASDAQ Global MarketSM
MAT,\"Mattel, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MATH,Metalpha Technology Holding Limited - Ordinary Shares,NASDAQ Capital Market
MATW,Matthews International Corporation - Class A Common Stock,NASDAQ Global Select MarketSM
MAXI,Simplify Bitcoin Strategy PLUS Income ETF,NASDAQ Global MarketSM
MAXN,\"Maxeon Solar Technologies, Ltd. - Ordinary Shares\",NASDAQ Global Select MarketSM
MAYS,\"J. W. Mays, Inc. - Common Stock\",NASDAQ Capital Market
MBB,iShares MBS ETF,NASDAQ Global MarketSM
MBCN,Middlefield Banc Corp. - Common Stock,NASDAQ Capital Market
MBIN,Merchants Bancorp - Common Stock,NASDAQ Capital Market
MBINM,\"Merchants Bancorp - Depositary Shares, Each Representing a 1/40th Interest in a Share of 8.25% Fixed-Rate Reset Series D Non-Cumulative Perpetual Preferred Stock\",NASDAQ Capital Market
MBINN,Merchants Bancorp - Depositary Shares 6.00% Fixed Rate Series C Non-Cumulative Perpetual Preferred Stock,NASDAQ Capital Market
MBINO,Merchants Bancorp - Depositary Shares Each Representing a 1/40th Interest in a Share of Series B Fixed-to-Floating Rate,NASDAQ Capital Market
MBIO,\"Mustang Bio, Inc. - Common Stock\",NASDAQ Capital Market
MBLY,Mobileye Global Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
MBNKP,\"Medallion Bank - Fixed-to-Floating Rate Non-Cumulative Perpetual Preferred Stock, Series F\",NASDAQ Capital Market
MBOT,Microbot Medical Inc.  - Common Stock,NASDAQ Capital Market
MBRX,\"Moleculin Biotech, Inc. - Common Stock\",NASDAQ Capital Market
MBUU,\"Malibu Boats, Inc. - Common Stock\",NASDAQ Global MarketSM
MBWM,Mercantile Bank Corporation - Common Stock,NASDAQ Global Select MarketSM
MCAA,Mountain  - Class A Ordinary Shares,NASDAQ Capital Market
MCAAU,Mountain  - Unit,NASDAQ Capital Market
MCAAW,Mountain  - Warrant,NASDAQ Capital Market
MCAC,Monterey Capital Acquisition Corporation - Class A Common Stock,NASDAQ Global MarketSM
MCACR,Monterey Capital Acquisition Corporation - Rights,NASDAQ Global MarketSM
MCACU,Monterey Capital Acquisition Corporation - Unit,NASDAQ Global MarketSM
MCACW,Monterey Capital Acquisition Corporation - Warrants,NASDAQ Global MarketSM
MCAG,Mountain Crest Acquisition Corp. V - Common Stock,NASDAQ Capital Market
MCAGR,Mountain Crest Acquisition Corp. V - Right,NASDAQ Capital Market
MCAGU,Mountain Crest Acquisition Corp. V - Unit,NASDAQ Capital Market
MCBC,Macatawa Bank Corporation - Common Stock,NASDAQ Global Select MarketSM
MCBS,\"MetroCity Bankshares, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MCFT,\"MasterCraft Boat Holdings, Inc. - Common Stock\",NASDAQ Global MarketSM
MCHI,iShares MSCI China ETF,NASDAQ Global MarketSM
MCHP,Microchip Technology Incorporated - Common Stock,NASDAQ Global Select MarketSM
MCHS,Matthews China Discovery Active ETF,NASDAQ Global MarketSM
MCHX,\"Marchex, Inc. - Class B Common Stock\",NASDAQ Global Select MarketSM
MCRB,\"Seres Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MCRI,\"Monarch Casino & Resort, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MCSE,Martin Currie Sustainable International Equity ETF,NASDAQ Global MarketSM
MCVT,\"Mill City Ventures III, Ltd. - Common Stock\",NASDAQ Capital Market
MDAI,\"Spectral AI, Inc. - Class A Common Stock\",NASDAQ Capital Market
MDAIW,\"Spectral AI, Inc. - Warrants\",NASDAQ Capital Market
MDB,\"MongoDB, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
MDBH,\"MDB Capital Holdings, LLC - Class A common \",NASDAQ Capital Market
MDCP,VictoryShares THB Mid Cap ETF,NASDAQ Global MarketSM
MDGL,\"Madrigal Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MDIA,Mediaco Holding Inc. - Class A Common Stock,NASDAQ Capital Market
MDIV,Multi-Asset Diversified Income Index Fund,NASDAQ Global MarketSM
MDJH,MDJM LTD - Ordinary Shares,NASDAQ Capital Market
MDLZ,\"Mondelez International, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
MDRR,\"Medalist Diversified REIT, Inc. - Common Stock\",NASDAQ Capital Market
MDRRP,\"Medalist Diversified REIT, Inc. - Series A Cumulative Redeemable Preferred Stock\",NASDAQ Capital Market
MDWD,MediWound Ltd. - Ordinary Shares,NASDAQ Global MarketSM
MDXG,\"MiMedx Group, Inc - Common Stock\",NASDAQ Capital Market
MDXH,MDxHealth SA - Ordinary Shares,NASDAQ Capital Market
ME,23andMe Holding Co. - Common Stock,NASDAQ Capital Market
MEDP,\"Medpace Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MEDS,\"TRxADE HEALTH, Inc. - Common Stock\",NASDAQ Capital Market
MEDX,Horizon Kinetics Medical ETF,NASDAQ Global MarketSM
MEGL,Magic Empire Global Limited - Ordinary Shares,NASDAQ Capital Market
MEIP,\"MEI Pharma, Inc. - Common Stock\",NASDAQ Capital Market
MELI,\"MercadoLibre, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MEMS,Matthews Emerging Markets Discovery Active ETF,NASDAQ Global MarketSM
MEOH,Methanex Corporation - Common Stock,NASDAQ Global Select MarketSM
MERC,Mercer International Inc. - Common Stock,NASDAQ Global Select MarketSM
MESA,\"Mesa Air Group, Inc. - Common Stock\",NASDAQ Capital Market
MESO,Mesoblast Limited - American Depositary Shares,NASDAQ Global Select MarketSM
META,\"Meta Platforms, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
METC,\"Ramaco Resources, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
METCB,\"Ramaco Resources, Inc. - Class B Common Stock\",NASDAQ Global Select MarketSM
METCL,\"Ramaco Resources, Inc. - 9.00% Senior Notes due 2026\",NASDAQ Global Select MarketSM
MFH,Mercurity Fintech Holding Inc. - American Ordinary Shares,NASDAQ Capital Market
MFI,mF International Limited - Ordinary Shares,NASDAQ Capital Market
MFIC,MidCap Financial Investment Corporation - Closed End Fund,NASDAQ Global Select MarketSM
MFICL,MidCap Financial Investment Corporation - 8.00% Notes due 2028,NASDAQ Global Select MarketSM
MFIN,Medallion Financial Corp. - Common Stock,NASDAQ Global Select MarketSM
MFLX,First Trust Flexible Municipal High Income ETF,NASDAQ Global MarketSM
MGEE,MGE Energy Inc. - Common Stock,NASDAQ Global Select MarketSM
MGIC,Magic Software Enterprises Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
MGIH,Millennium Group International Holdings Limited - Ordinary Shares,NASDAQ Capital Market
MGNI,\"Magnite, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MGNX,\"MacroGenics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MGOL,MGO Global Inc. - Common Stock,NASDAQ Capital Market
MGPI,\"MGP Ingredients, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MGRC,McGrath RentCorp - Common Stock,NASDAQ Global Select MarketSM
MGRM,Monogram Technologies Inc. - Common Stock,NASDAQ Capital Market
MGRX,\"Mangoceuticals, Inc. - Common Stock\",NASDAQ Capital Market
MGTX,MeiraGTx Holdings plc - Ordinary Shares,NASDAQ Global Select MarketSM
MGX,\"Metagenomi, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MGYR,\"Magyar Bancorp, Inc. - Common Stock\",NASDAQ Global MarketSM
MHLD,\"Maiden Holdings, Ltd. - Common Stock\",NASDAQ Capital Market
MHUA,\"Meihua International Medical Technologies Co., Ltd. - Ordinary Shares\",NASDAQ Global MarketSM
MICS,\"The Singing Machine Company, Inc. - Common Stock\",NASDAQ Capital Market
MIDD,The Middleby Corporation - Common Stock,NASDAQ Global Select MarketSM
MIGI,Mawson Infrastructure Group Inc. - Common Stock,NASDAQ Capital Market
MILN,Global X Millennial Consumer ETF,NASDAQ Global MarketSM
MIND,\"MIND Technology, Inc. - Common Stock\",NASDAQ Capital Market
MINDP,\"MIND Technology, Inc. - Series A 9.00% Series A Cumulative Preferred Stock\",NASDAQ Capital Market
MINM,\"Minim, Inc. - Common Stock\",NASDAQ Capital Market
MIRA,\"MIRA Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
MIRM,\"Mirum Pharmaceuticals, Inc. - common stock\",NASDAQ Global MarketSM
MIST,Milestone Pharmaceuticals Inc. - Common Shares,NASDAQ Global Select MarketSM
MITA,Coliseum Acquisition Corp. - Class A Ordinary Share,NASDAQ Capital Market
MITAU,Coliseum Acquisition Corp. - Unit,NASDAQ Capital Market
MITAW,Coliseum Acquisition Corp. - Warrant,NASDAQ Capital Market
MITK,\"Mitek Systems, Inc. - Common Stock\",NASDAQ Capital Market
MKAM,MKAM ETF,NASDAQ Global MarketSM
MKSI,\"MKS Instruments, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MKTW,\"MarketWise, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
MKTX,\"MarketAxess Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MLAB,\"Mesa Laboratories, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MLCO,Melco Resorts & Entertainment Limited - American Depositary Shares ,NASDAQ Global Select MarketSM
MLEC,Moolec Science SA - Ordinary shares,NASDAQ Capital Market
MLECW,Moolec Science SA - Warrant,NASDAQ Capital Market
MLGO,\"MicroAlgo, Inc. - Ordinary Shares\",NASDAQ Capital Market
MLKN,\"MillerKnoll, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MLTX,MoonLake Immunotherapeutics - Class A Ordinary Shares,NASDAQ Capital Market
MLYS,\"Mineralys Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MMAT,Meta Materials Inc. - Common Stock,NASDAQ Capital Market
MMLP,Martin Midstream Partners L.P. - Common Units Representing Limited Partnership Interests,NASDAQ Global Select MarketSM
MMSI,\"Merit Medical Systems, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MMV,MultiMetaVerse Holdings Limited - Class A Ordinary Share,NASDAQ Capital Market
MMVWW,MultiMetaVerse Holdings Limited - Warrant,NASDAQ Capital Market
MMYT,MakeMyTrip Limited - Ordinary Shares,NASDAQ Global Select MarketSM
MNDO,MIND C.T.I. Ltd. - Ordinary Shares,NASDAQ Global MarketSM
MNDR,Mobile-health Network Solutions - Class A Ordinary Shares,NASDAQ Capital Market
MNDY,monday.com Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
MNKD,MannKind Corporation - Common Stock,NASDAQ Global MarketSM
MNMD,Mind Medicine (MindMed) Inc. - Common Shares,NASDAQ Global Select MarketSM
MNOV,\"MediciNova, Inc. - Common Stock\",NASDAQ Global MarketSM
MNPR,Monopar Therapeutics Inc. - Common Stock,NASDAQ Capital Market
MNRO,\"Monro, Inc.  - Common Stock\",NASDAQ Global Select MarketSM
MNSB,\"MainStreet Bancshares, Inc. - Common Stock\",NASDAQ Capital Market
MNSBP,\"MainStreet Bancshares, Inc. - Depositary Shares\",NASDAQ Capital Market
MNST,Monster Beverage Corporation - Common Stock,NASDAQ Global Select MarketSM
MNTK,\"Montauk Renewables, Inc. - Common Stock\",NASDAQ Capital Market
MNTL,Tema Neuroscience and Mental Health ETF,NASDAQ Global MarketSM
MNTS,Momentus Inc. - Class A Common Stock,NASDAQ Capital Market
MNTSW,Momentus Inc. - Warrant,NASDAQ Capital Market
MNTX,\"Manitex International, Inc. - common stock\",NASDAQ Capital Market
MNY,MoneyHero Limited - Class A Ordinary Shares,NASDAQ Global MarketSM
MNYWW,MoneyHero Limited - Warrants,NASDAQ Global MarketSM
MOB,Mobilicom Limited - American Depositary Shares,NASDAQ Capital Market
MOBBW,Mobilicom Limited - Warrants,NASDAQ Capital Market
MOBX,\"Mobix Labs, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
MOBXW,\"Mobix Labs, Inc. - Warrants\",NASDAQ Capital Market
MODD,\"Modular Medical, Inc. - common stock\",NASDAQ Capital Market
MODL,VictoryShares WestEnd U.S. Sector ETF,NASDAQ Global MarketSM
MODV,ModivCare Inc. - Common Stock,NASDAQ Global Select MarketSM
MOFG,\"MidWestOne Financial Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MOGO,Mogo Inc. - Common Shares,NASDAQ Capital Market
MOLN,Molecular Partners AG - American Depositary Shares,NASDAQ Global Select MarketSM
MOMO,Hello Group Inc.  - American Depositary Shares,NASDAQ Global Select MarketSM
MOND,\"Mondee Holdings, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
MOOD,Relative Sentiment Tactical Allocation ETF,NASDAQ Global MarketSM
MOR,MorphoSys AG - American Depositary Shares,NASDAQ Global Select MarketSM
MORF,\"Morphic Holding, Inc. - Common Stock\",NASDAQ Global MarketSM
MORN,\"Morningstar, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MOVE,Movano Inc. - Common Stock,NASDAQ Capital Market
MPAA,\"Motorcar Parts of America, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MPB,Mid Penn Bancorp - Common Stock,NASDAQ Global MarketSM
MPWR,\"Monolithic Power Systems, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MQ,\"Marqeta, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
MRAI,\"Marpai, Inc. - Class A Common Stock\",NASDAQ Capital Market
MRAM,\"Everspin Technologies, Inc. - Common Stock\",NASDAQ Global MarketSM
MRBK,Meridian Corporation - Common Stock,NASDAQ Global Select MarketSM
MRCC,Monroe Capital Corporation - Closed End Fund,NASDAQ Global Select MarketSM
MRCY,Mercury Systems Inc - Common Stock,NASDAQ Global Select MarketSM
MREO,Mereo BioPharma Group plc - American Depositary Shares,NASDAQ Capital Market
MRIN,Marin Software Incorporated - Common Stock,NASDAQ Capital Market
MRKR,\"Marker Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
MRM,MEDIROM Healthcare Technologies Inc. - American Depositary Share,NASDAQ Capital Market
MRNA,\"Moderna, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MRNO,Murano Global Investments PLC - Ordinary Shares,NASDAQ Capital Market
MRNOW,Murano Global Investments PLC - Warrants,NASDAQ Capital Market
MRNS,\"Marinus Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global MarketSM
MRSN,\"Mersana Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MRTN,\"Marten Transport, Ltd. - Common Stock\",NASDAQ Global Select MarketSM
MRUS,Merus N.V. - Common Shares,NASDAQ Global MarketSM
MRVI,\"Maravai LifeSciences Holdings, Inc. - Class A common stock\",NASDAQ Global Select MarketSM
MRVL,\"Marvell Technology, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MRX,Marex Group plc - Ordinary Shares,NASDAQ Global Select MarketSM
MSAI,\"MultiSensor AI Holdings, Inc. - Common Stock\",NASDAQ Capital Market
MSAIW,\"MultiSensor AI Holdings, Inc. - Warrants\",NASDAQ Capital Market
MSBI,\"Midland States Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MSBIP,\"Midland States Bancorp, Inc. - Depositary Shares Each Representing a 1/40th Interest in a Share of 7.750% Fixed-Rate Reset Non-Cumulative Perpetual Preferred Stock, Series A\",NASDAQ Global Select MarketSM
MSEX,Middlesex Water Company - Common Stock,NASDAQ Global Select MarketSM
MSFD,Direxion Daily MSFT Bear 1X Shares,NASDAQ Global MarketSM
MSFL,GraniteShares 2x Long MSFT Daily ETF,NASDAQ Global MarketSM
MSFT,Microsoft Corporation - Common Stock,NASDAQ Global Select MarketSM
MSFU,Direxion Daily MSFT Bull 2X Shares,NASDAQ Global MarketSM
MSGM,Motorsport Games Inc. - Class A Common Stock,NASDAQ Capital Market
MSS,Maison Solutions Inc. - Class A Common Stock,NASDAQ Capital Market
MSSA,Metal Sky Star Acquisition Corporation - Ordinary shares,NASDAQ Global MarketSM
MSSAR,Metal Sky Star Acquisition Corporation - Right,NASDAQ Global MarketSM
MSSAU,Metal Sky Star Acquisition Corporation - Unit,NASDAQ Global MarketSM
MSSAW,Metal Sky Star Acquisition Corporation - Warrant,NASDAQ Global MarketSM
MSTR,MicroStrategy Incorporated - Class A Common Stock,NASDAQ Global Select MarketSM
MTC,\"MMTec, Inc. - Common Shares\",NASDAQ Capital Market
MTCH,\"Match Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MTEK,Maris-Tech Ltd. - ordinary shares,NASDAQ Capital Market
MTEKW,Maris-Tech Ltd. - Warrants,NASDAQ Capital Market
MTEM,\"Molecular Templates, Inc. - Common Stock\",NASDAQ Capital Market
MTEN,Mingteng International Corporation Inc. - Ordinary Shares,NASDAQ Capital Market
MTEX,\"Mannatech, Incorporated - Common Stock\",NASDAQ Global Select MarketSM
MTLS,Materialise NV - American Depositary Shares,NASDAQ Global Select MarketSM
MTRX,Matrix Service Company - Common Stock,NASDAQ Global Select MarketSM
MTSI,\"MACOM Technology Solutions Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MTTR,\"Matterport, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
MU,\"Micron Technology, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MULN,\"Mullen Automotive, Inc. - Common Stock\",NASDAQ Capital Market
MURA,Mural Oncology plc - Ordinary Shares,NASDAQ Global MarketSM
MVBF,MVB Financial Corp. - Common Stock,NASDAQ Capital Market
MVIS,\"MicroVision, Inc. - Common Stock\",NASDAQ Global MarketSM
MVST,\"Microvast Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MVSTW,\"Microvast Holdings, Inc. - Warrant\",NASDAQ Global Select MarketSM
MXCT,\"MaxCyte, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MXL,\"MaxLinear, Inc - Common Stock\",NASDAQ Global Select MarketSM
MYFW,\"First Western Financial, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MYGN,\"Myriad Genetics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MYMD,\"MyMD Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
MYNA,Mynaric AG - American Depository Shares,NASDAQ Global Select MarketSM
MYNZ,Mainz Biomed N.V. - Ordinary Shares,NASDAQ Capital Market
MYPS,\"PLAYSTUDIOS, Inc.  - Class A Common Stock\",NASDAQ Global MarketSM
MYPSW,\"PLAYSTUDIOS, Inc.  - Warrant\",NASDAQ Global MarketSM
MYRG,\"MYR Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
MYSZ,\"My Size, Inc. - Common Stock\",NASDAQ Capital Market
NA,Nano Labs Ltd - Class A Ordinary Shares,NASDAQ Global MarketSM
NAAS,NaaS Technology Inc. - American Depositary Shares,NASDAQ Capital Market
NAII,\"Natural Alternatives International, Inc. - Common Stock\",NASDAQ Global MarketSM
NAMS,NewAmsterdam Pharma Company N.V. - Ordinary Shares,NASDAQ Global MarketSM
NAMSW,NewAmsterdam Pharma Company N.V. - Warrant,NASDAQ Global MarketSM
NAOV,\"NanoVibronix, Inc. - Common Stock\",NASDAQ Capital Market
NARI,\"Inari Medical, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NATH,\"Nathan's Famous, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NATR,\"Nature's Sunshine Products, Inc. - Common Stock\",NASDAQ Capital Market
NAUT,\"Nautilus Biotechnology, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NAVI,Navient Corporation - Common Stock,NASDAQ Global Select MarketSM
NB,NioCorp Developments Ltd. - Common Stock,NASDAQ Global MarketSM
NBBK,\"NB Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
NBIX,\"Neurocrine Biosciences, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NBN,Northeast Bank - Common Stock,NASDAQ Global MarketSM
NBST,Newbury Street Acquisition Corporation - Common Stock,NASDAQ Capital Market
NBSTU,Newbury Street Acquisition Corporation - Units,NASDAQ Capital Market
NBSTW,Newbury Street Acquisition Corporation - Warrants,NASDAQ Capital Market
NBTB,NBT Bancorp Inc. - Common Stock,NASDAQ Global Select MarketSM
NBTX,Nanobiotix S.A. - ADSs,NASDAQ Global Select MarketSM
NCI,Neo-Concept International Group Holdings Limited - Ordinary Shares,NASDAQ Capital Market
NCMI,\"National CineMedia, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NCNA,NuCana plc - American Depositary Shares,NASDAQ Capital Market
NCNC,noco-noco Inc. - Ordinary Share,NASDAQ Capital Market
NCNCW,noco-noco Inc. - Warrant,NASDAQ Capital Market
NCNO,\"nCino, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NCPB,Nuveen Core Plus Bond ETF,NASDAQ Global MarketSM
NCPL,Netcapital Inc. - Common Stock,NASDAQ Capital Market
NCPLW,Netcapital Inc. - warrants,NASDAQ Capital Market
NCRA,\"Nocera, Inc. - common stock\",NASDAQ Capital Market
NCSM,\"NCS Multistage Holdings, Inc. - Common Stock\",NASDAQ Capital Market
NCTY,The9 Limited - American Depository Shares,NASDAQ Capital Market
NDAQ,\"Nasdaq, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NDLS,Noodles & Company - Common Stock,NASDAQ Global Select MarketSM
NDRA,ENDRA Life Sciences Inc. - Common Stock,NASDAQ Capital Market
NDSN,Nordson Corporation - Common Stock,NASDAQ Global Select MarketSM
NECB,\"NorthEast Community Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
NEGG,\"Newegg Commerce, Inc. - Common Shares\",NASDAQ Capital Market
NEO,\"NeoGenomics, Inc. - Common Stock\",NASDAQ Capital Market
NEOG,Neogen Corporation - Common Stock,NASDAQ Global Select MarketSM
NEON,Neonode Inc. - Common Stock,NASDAQ Capital Market
NEOV,NeoVolta Inc. - Common Stock,NASDAQ Capital Market
NEOVW,NeoVolta Inc. - Warrant,NASDAQ Capital Market
NEPH,\"Nephros, Inc. - Common Stock\",NASDAQ Capital Market
NERD,Roundhill Video Games ETF,NASDAQ Global MarketSM
NERV,\"Minerva Neurosciences, Inc - Common Stock\",NASDAQ Capital Market
NETD,Nabors Energy Transition Corp. II - Class A Ordinary Shares,NASDAQ Global MarketSM
NETDU,Nabors Energy Transition Corp. II - Unit,NASDAQ Global MarketSM
NETDW,Nabors Energy Transition Corp. II - Warrant,NASDAQ Global MarketSM
NEWT,\"NewtekOne, Inc. - Common Stock\",NASDAQ Global MarketSM
NEWTI,\"NewtekOne, Inc. - 8.00% Fixed Rate Senior Notes due 2028\",NASDAQ Global MarketSM
NEWTL,\"NewtekOne, Inc. - 5.75% Notes due 2024\",NASDAQ Global MarketSM
NEWTZ,\"NewtekOne, Inc. - 5.50% Notes Due 2026\",NASDAQ Global MarketSM
NEWZ,StockSnips AI-Powered Sentiment US All Cap ETF,NASDAQ Global MarketSM
NEXI,\"NexImmune, Inc. - Common Stock\",NASDAQ Capital Market
NEXN,Nexxen International Ltd. - American Depository Shares,NASDAQ Global MarketSM
NEXT,NextDecade Corporation - Common Stock,NASDAQ Capital Market
NFBK,\"Northfield Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NFE,New Fortress Energy Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
NFLX,\"Netflix, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NFTY,First Trust India Nifty 50 Equal Weight ETF,NASDAQ Global MarketSM
NGNE,Neurogene Inc. - Common Stock,NASDAQ Global MarketSM
NHTC,Natural Health Trends Corp. - Commn Stock,NASDAQ Capital Market
NICE,NICE Ltd - American Depositary Shares each representing one Ordinary Share,NASDAQ Global Select MarketSM
NICK,\"Nicholas Financial, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NIKL,Sprott Nickel Miners ETF,NASDAQ Global MarketSM
NIOBW,NioCorp Developments Ltd. - Warrant,NASDAQ Capital Market
NISN,\"NiSun Intl Enterprise Development Group Co, Ltd - Class A Common Shares\",NASDAQ Capital Market
NITO,\"N2OFF, Inc. - Common Stock\",NASDAQ Capital Market
NIU,Niu Technologies - American Depositary Shares,NASDAQ Global MarketSM
NIVF,NewGenIvf Group Limited - Class A Ordinary Shares,NASDAQ Global MarketSM
NIVFW,NewGenIvf Group Limited - Warrants,NASDAQ Capital Market
NKGN,\"NKGen Biotech, Inc. - Common Stock\",NASDAQ Global MarketSM
NKGNW,\"NKGen Biotech, Inc. - Warrants\",NASDAQ Capital Market
NKLA,Nikola Corporation - Common Stock,NASDAQ Global Select MarketSM
NKSH,\"National Bankshares, Inc. - Common Stock\",NASDAQ Capital Market
NKTR,Nektar Therapeutics - Common Stock,NASDAQ Capital Market
NKTX,\"Nkarta, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NLSP,NLS Pharmaceutics Ltd. - Common Shares,NASDAQ Capital Market
NLSPW,NLS Pharmaceutics Ltd. - Warrant,NASDAQ Capital Market
NMFC,New Mountain Finance Corporation - Common Stock,NASDAQ Global Select MarketSM
NMFCZ,New Mountain Finance Corporation - 8.250% Notes due 2028,NASDAQ Global MarketSM
NMHI,Nature's Miracle Holding Inc. - Common Stock,NASDAQ Global MarketSM
NMHIW,Nature's Miracle Holding Inc. - Warrants,NASDAQ Capital Market
NMIH,NMI Holdings Inc - Common Stock,NASDAQ Global MarketSM
NMRA,\"Neumora Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NMRK,\"Newmark Group, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
NMTC,NeuroOne Medical Technologies Corporation - Common Stock,NASDAQ Capital Market
NN,NextNav Inc. - Common stock,NASDAQ Capital Market
NNAG,99 Acquisition Group Inc. - Class A Common Stock,NASDAQ Global MarketSM
NNAGR,99 Acquisition Group Inc. - Right,NASDAQ Global MarketSM
NNAGU,99 Acquisition Group Inc. - Unit,NASDAQ Global MarketSM
NNAGW,99 Acquisition Group Inc. - Warrant,NASDAQ Global MarketSM
NNAVW,NextNav Inc. - Warrant,NASDAQ Capital Market
NNBR,\"NN, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NNDM,Nano Dimension Ltd. - American Depositary Shares,NASDAQ Capital Market
NNE,Nano Nuclear Energy Inc. - common stock,NASDAQ Capital Market
NNOX,NANO-X IMAGING LTD - Ordinary Shares,NASDAQ Global MarketSM
NODK,\"NI Holdings, Inc. - Common Stock\",NASDAQ Capital Market
NOTV,\"Inotiv, Inc. - Common Stock\",NASDAQ Capital Market
NOVT,Novanta Inc. - Common Shares,NASDAQ Global Select MarketSM
NOVV,Nova Vision Acquisition Corp. - Ordinary share,NASDAQ Capital Market
NOVVR,Nova Vision Acquisition Corp. - Rights,NASDAQ Capital Market
NOVVU,Nova Vision Acquisition Corp. - Unit,NASDAQ Capital Market
NOVVW,Nova Vision Acquisition Corp. - Warrant,NASDAQ Capital Market
NPAB,New Providence Acquisition Corp. II - Class A Common Stock,NASDAQ Global MarketSM
NPABU,New Providence Acquisition Corp. II - Unit,NASDAQ Global MarketSM
NPABW,New Providence Acquisition Corp. II - Warrant,NASDAQ Global MarketSM
NPCE,\"Neuropace, Inc. - Common Stock\",NASDAQ Global MarketSM
NPFI,Nuveen Preferred and Income ETF,NASDAQ Global MarketSM
NRBO,\"NeuroBo Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
NRC,National Research Corporation - Common Stock,NASDAQ Global Select MarketSM
NRDS,\"NerdWallet, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
NRES,Xtrackers RREEF Global Natural Resources ETF,NASDAQ Global MarketSM
NRIM,Northrim BanCorp Inc - Common Stock,NASDAQ Global Select MarketSM
NRIX,\"Nurix Therapeutics, Inc. - Common stock\",NASDAQ Global MarketSM
NRSN,NeuroSense Therapeutics Ltd. - Ordinary Shares,NASDAQ Capital Market
NRSNW,NeuroSense Therapeutics Ltd. - Warrant,NASDAQ Capital Market
NRXP,\"NRX Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
NRXPW,\"NRX Pharmaceuticals, Inc. - Warrant\",NASDAQ Capital Market
NSCR,Nuveen Sustainable Core ETF,NASDAQ Global MarketSM
NSI,National Security Emerging Markets Index ETF,NASDAQ Global MarketSM
NSIT,\"Insight Enterprises, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NSPR,InspireMD Inc. - Common Stock,NASDAQ Capital Market
NSSC,\"NAPCO Security Technologies, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NSTS,\"NSTS Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
NSYS,Nortech Systems Incorporated - Common Stock,NASDAQ Capital Market
NTAP,\"NetApp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NTBL,\"Notable Labs, Ltd. - Ordinary Shares\",NASDAQ Capital Market
NTCT,\"NetScout Systems, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NTES,\"NetEase, Inc. - American Depositary Shares, each representing 5 ordinary shares\",NASDAQ Global Select MarketSM
NTGR,\"NETGEAR, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NTIC,Northern Technologies International Corporation - Common Stock,NASDAQ Global MarketSM
NTLA,\"Intellia Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
NTNX,\"Nutanix, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
NTRA,\"Natera, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NTRB,Nutriband Inc. - Common Stock,NASDAQ Capital Market
NTRBW,Nutriband Inc. - Warrant,NASDAQ Capital Market
NTRP,\"NextTrip, Inc. - Common Stock\",NASDAQ Capital Market
NTRS,Northern Trust Corporation - Common Stock,NASDAQ Global Select MarketSM
NTRSO,\"Northern Trust Corporation - Depositary Shares Each Representing a 1/1,000th Interest in a Share of Series E Non-Cumulative Perpetual Preferred Stock\",NASDAQ Global Select MarketSM
NTWK,NETSOL Technologies Inc. - Common Stock,NASDAQ Capital Market
NTZG,Nuveen Global Net Zero Transition ETF,NASDAQ Global MarketSM
NUKK,Nukkleus Inc. - Ordinary Shares,NASDAQ Global MarketSM
NUKKW,Nukkleus Inc. - Warrants,NASDAQ Capital Market
NURO,\"NeuroMetrix, Inc. - Common Stock\",NASDAQ Capital Market
NUSB,Nuveen Ultra Short Income ETF,NASDAQ Global MarketSM
NUTX,Nutex Health Inc. - Common Stock,NASDAQ Capital Market
NUVL,\"Nuvalent, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
NUVO,Holdco Nuvo Group D.G Ltd. - Ordinary Shares,NASDAQ Global MarketSM
NUVOW,Holdco Nuvo Group D.G Ltd. - Warrants,NASDAQ Capital Market
NUWE,\"Nuwellis, Inc. - Common Stock\",NASDAQ Capital Market
NUZE,\"NuZee, Inc. - Common Stock\",NASDAQ Capital Market
NVAC,NorthView Acquisition Corporation - Common Stock,NASDAQ Global MarketSM
NVACR,NorthView Acquisition Corporation - Rights,NASDAQ Global MarketSM
NVACW,NorthView Acquisition Corporation - Warrant,NASDAQ Global MarketSM
NVAX,\"Novavax, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NVCR,NovoCure Limited - Ordinary Shares,NASDAQ Global Select MarketSM
NVCT,\"Nuvectis Pharma, Inc. - Common Stock\",NASDAQ Capital Market
NVD,GraniteShares 2x Short NVDA Daily ETF,NASDAQ Global MarketSM
NVDA,NVIDIA Corporation - Common Stock,NASDAQ Global Select MarketSM
NVDD,Direxion Daily NVDA Bear 1X Shares,NASDAQ Global MarketSM
NVDL,GraniteShares 2x Long NVDA Daily ETF,NASDAQ Global MarketSM
NVDS,Tradr 1.25X NVDA Bear Daily ETF,NASDAQ Global MarketSM
NVDU,Direxion Daily NVDA Bull 2X Shares,NASDAQ Global MarketSM
NVEC,NVE Corporation - Common Stock,NASDAQ Capital Market
NVEE,\"NV5 Global, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NVEI,Nuvei Corporation - Subordinate Voting Shares,NASDAQ Global Select MarketSM
NVFY,\"Nova Lifestyle, Inc - Common Stock\",NASDAQ Capital Market
NVMI,Nova Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
NVNI,Nvni Group Limited - Ordinary Shares,NASDAQ Capital Market
NVNIW,Nvni Group Limited - Warrants,NASDAQ Capital Market
NVNO,enVVeno Medical Corporation - Common Stock,NASDAQ Capital Market
NVOS,\"Novo Integrated Sciences, Inc. - Common Stock\",NASDAQ Capital Market
NVTS,Navitas Semiconductor Corporation - Common Stock,NASDAQ Global MarketSM
NVVE,Nuvve Holding Corp. - Common Stock,NASDAQ Capital Market
NVVEW,Nuvve Holding Corp. - Warrant,NASDAQ Capital Market
NVX,NOVONIX Limited - American Depository Shares,NASDAQ Global MarketSM
NWBI,\"Northwest Bancshares, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NWE,\"NorthWestern Energy Group, Inc.  - Common Stock\",NASDAQ Global Select MarketSM
NWFL,Norwood Financial Corp. - Common Stock,NASDAQ Global MarketSM
NWGL,Nature Wood Group Limited - American Depositary Shares,NASDAQ Capital Market
NWL,Newell Brands Inc. - Common Stock,NASDAQ Global Select MarketSM
NWLI,\"National Western Life Group, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
NWPX,Northwest Pipe Company - Common Stock,NASDAQ Global Select MarketSM
NWS,News Corporation - Class B Common Stock,NASDAQ Global Select MarketSM
NWSA,News Corporation - Class A Common Stock,NASDAQ Global Select MarketSM
NWTN,NWTN Inc. - Class B Ordinary Shares,NASDAQ Capital Market
NWTNW,NWTN Inc. - Warrant,NASDAQ Capital Market
NXGL,\"NexGel, Inc - Common Stock\",NASDAQ Capital Market
NXGLW,\"NexGel, Inc - Warrant\",NASDAQ Capital Market
NXL,\"Nexalin Technology, Inc. - Common Stock\",NASDAQ Capital Market
NXLIW,\"Nexalin Technology, Inc. - Warrant\",NASDAQ Capital Market
NXPI,NXP Semiconductors N.V. - Common Stock,NASDAQ Global Select MarketSM
NXPL,NextPlat Corp - Common Stock,NASDAQ Capital Market
NXPLW,NextPlat Corp - Warrants,NASDAQ Capital Market
NXST,\"Nexstar Media Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NXT,Nextracker Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
NXTC,\"NextCure, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NXTG,First Trust Indxx NextG ETF,NASDAQ Global MarketSM
NXTT,Next Technology Holding Inc. - Ordinary Shares,NASDAQ Capital Market
NXU,\"Nxu, Inc.  - Class A Common Stock\",NASDAQ Capital Market
NYAX,Nayax Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
NYMT,\"New York Mortgage Trust, Inc. - Common Stock\",NASDAQ Global Select MarketSM
NYMTL,\"New York Mortgage Trust, Inc. - 6.875% Series F Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock, $0.01 par value per share\",NASDAQ Global Select MarketSM
NYMTM,\"New York Mortgage Trust, Inc. - 7.875% Series E Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock\",NASDAQ Global Select MarketSM
NYMTN,\"New York Mortgage Trust, Inc. - 8.00% Series D Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock\",NASDAQ Global Select MarketSM
NYMTZ,\"New York Mortgage Trust, Inc. - 7.000% Series G Cumulative Redeemable Preferred Stock, $0.01 par value per share\",NASDAQ Global Select MarketSM
NYXH,Nyxoah SA - Ordinary Shares,NASDAQ Global MarketSM
NZAC,SPDR MSCI ACWI Climate Paris Aligned ETF,NASDAQ Global MarketSM
NZUS,SPDR MSCI USA Climate Paris Aligned ETF,NASDAQ Global MarketSM
OABI,\"OmniAb, Inc. - Common Stock\",NASDAQ Global MarketSM
OABIW,\"OmniAb, Inc. - Warrant\",NASDAQ Capital Market
OAKU,Oak Woods Acquisition Corporation - Class A Ordinary Shares,NASDAQ Capital Market
OAKUR,Oak Woods Acquisition Corporation - Right,NASDAQ Capital Market
OAKUU,Oak Woods Acquisition Corporation - Unit,NASDAQ Capital Market
OAKUW,Oak Woods Acquisition Corporation - Warrant,NASDAQ Capital Market
OB,Outbrain Inc. - Common Stock,NASDAQ Global Select MarketSM
OBIL,US Treasury 12 Month Bill ETF,NASDAQ Global MarketSM
OBIO,\"Orchestra BioMed Holdings, Inc. - Ordinary Shares\",NASDAQ Global MarketSM
OBLG,Oblong Inc. - Common Stock,NASDAQ Capital Market
OBT,\"Orange County Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
OCAX,OCA Acquisition Corp. - Class A Common Stock,NASDAQ Capital Market
OCAXU,OCA Acquisition Corp. - Unit,NASDAQ Capital Market
OCAXW,OCA Acquisition Corp. - Warrant,NASDAQ Capital Market
OCC,Optical Cable Corporation - Common Stock,NASDAQ Global MarketSM
OCCI,\"OFS Credit Company, Inc. - Closed End Fund\",NASDAQ Capital Market
OCCIN,\"OFS Credit Company, Inc. - 5.25% Series E Term Preferred Stock Due 2026\",NASDAQ Capital Market
OCCIO,\"OFS Credit Company, Inc. - 6.125% Series C Term Preferred Stock\",NASDAQ Capital Market
OCEA,\"Ocean Biomedical, Inc. - Common Stock\",NASDAQ Capital Market
OCEAW,\"Ocean Biomedical, Inc. - Warrants\",NASDAQ Capital Market
OCFC,OceanFirst Financial Corp. - Common Stock,NASDAQ Global Select MarketSM
OCFCP,OceanFirst Financial Corp. - Depositary Shares,NASDAQ Global Select MarketSM
OCG,Oriental Culture Holding LTD - Ordinary Shares,NASDAQ Capital Market
OCGN,\"Ocugen, Inc. - Common Stock\",NASDAQ Capital Market
OCS,Oculis Holding AG - Ordinary shares,NASDAQ Global MarketSM
OCSAW,Oculis Holding AG - Warrants,NASDAQ Global MarketSM
OCSL,Oaktree Specialty Lending Corporation - Closed End Fund,NASDAQ Global Select MarketSM
OCTO,Eightco Holdings Inc. - Common Stock,NASDAQ Capital Market
OCUL,\"Ocular Therapeutix, Inc. - Common Stock\",NASDAQ Global MarketSM
OCUP,\"Ocuphire Pharma, Inc. - Common Stock\",NASDAQ Capital Market
OCX,Oncocyte Corporation - Common Stock,NASDAQ Capital Market
ODD,ODDITY Tech Ltd. - Class A Ordinary Shares,NASDAQ Global MarketSM
ODDS,Pacer BlueStar Digital Entertainment ETF,NASDAQ Global MarketSM
ODFL,\"Old Dominion Freight Line, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ODP,The ODP Corporation - Common Stock,NASDAQ Global Select MarketSM
ODVWZ,Osisko Development Corp. - Warrant,NASDAQ Capital Market
OESX,\"Orion Energy Systems, Inc. - Common Stock\",NASDAQ Capital Market
OFIX,Orthofix Medical Inc.  - Common Stock,NASDAQ Global Select MarketSM
OFLX,\"Omega Flex, Inc. - Common Stock\",NASDAQ Global MarketSM
OFS,OFS Capital Corporation - Closed End Fund,NASDAQ Global Select MarketSM
OFSSH,OFS Capital Corporation - 4.95% Notes due 2028,NASDAQ Global Select MarketSM
OGI,Organigram Holdings Inc. - Common Shares,NASDAQ Global Select MarketSM
OKTA,\"Okta, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
OKYO,OKYO Pharma Limited - Ordinary Shares,NASDAQ Capital Market
OLB,\"The OLB Group, Inc. - Common Stock\",NASDAQ Capital Market
OLED,Universal Display Corporation - Common Stock,NASDAQ Global Select MarketSM
OLK,Olink Holding AB (publ) - American Depositary Shares,NASDAQ Global MarketSM
OLLI,\"Ollie's Bargain Outlet Holdings, Inc. - Common Stock\",NASDAQ Global MarketSM
OLMA,\"Olema Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
OLPX,\"Olaplex Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
OM,\"Outset Medical, Inc. - Common Stock\",NASDAQ Global Select MarketSM
OMAB,Grupo Aeroportuario del Centro Norte S.A.B. de C.V. - American Depositary Shares each representing 8 Series B shares,NASDAQ Global Select MarketSM
OMCL,\"Omnicell, Inc. - Common Stock\",NASDAQ Global Select MarketSM
OMER,Omeros Corporation - Common Stock,NASDAQ Global MarketSM
OMEX,\"Odyssey Marine Exploration, Inc. - Common Stock\",NASDAQ Capital Market
OMGA,\"Omega Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
OMH,Ohmyhome Limited - Ordinary Shares,NASDAQ Capital Market
OMIC,\"Singular Genomics Systems, Inc. - Common Stock\",NASDAQ Capital Market
ON,ON Semiconductor Corporation - Common Stock,NASDAQ Global Select MarketSM
ONB,Old National Bancorp - Common Stock,NASDAQ Global Select MarketSM
ONBPO,\"Old National Bancorp - Depositary Shares, Each Representing a 1/40th Interest in a Share of Series C Preferred Stock\",NASDAQ Global Select MarketSM
ONBPP,\"Old National Bancorp - Depositary Shares, Each Representing a 1/40th Interest in a Share of Series A Preferred Stock\",NASDAQ Global Select MarketSM
ONCO,\"Onconetix, Inc.  - Common Stock\",NASDAQ Capital Market
ONCT,\"Oncternal Therapeutics, Inc.  - Common Stock\",NASDAQ Capital Market
ONCY,Oncolytics Biotech Inc. - Common Shares,NASDAQ Capital Market
ONDS,Ondas Holdings Inc. - Common Stock,NASDAQ Capital Market
ONEQ,Fidelity Nasdaq Composite Index ETF,NASDAQ Global MarketSM
ONEW,OneWater Marine Inc. - Class A Common Stock,NASDAQ Global MarketSM
ONFO,Onfolio Holdings Inc. - Common Stock,NASDAQ Capital Market
ONFOW,Onfolio Holdings Inc. - Warrant,NASDAQ Capital Market
ONMD,OneMedNet Corp - Class A Common Stock,NASDAQ Global MarketSM
ONMDW,OneMedNet Corp - Warrant,NASDAQ Capital Market
ONVO,\"Organovo Holdings, Inc. - Common Stock\",NASDAQ Capital Market
ONYX,Onyx Acquisition Co. I - Class A Ordinary Shares,NASDAQ Capital Market
ONYXU,Onyx Acquisition Co. I - Unit,NASDAQ Capital Market
ONYXW,Onyx Acquisition Co. I - Warrant,NASDAQ Capital Market
OP,OceanPal Inc. - Common Stock,NASDAQ Capital Market
OPAL,OPAL Fuels Inc. - Class A Common Stock,NASDAQ Capital Market
OPBK,OP Bancorp - Common Stock,NASDAQ Global MarketSM
OPCH,\"Option Care Health, Inc. - Common Stock\",NASDAQ Global Select MarketSM
OPEN,Opendoor Technologies Inc  - Common Stock,NASDAQ Global Select MarketSM
OPGN,\"OpGen, Inc. - Common Stock\",NASDAQ Capital Market
OPHC,\"OptimumBank Holdings, Inc. - Common Stock\",NASDAQ Capital Market
OPI,Office Properties Income Trust - Common Shares of Beneficial Interest,NASDAQ Global Select MarketSM
OPINL,Office Properties Income Trust - 6.375% Senior Notes due 2050,NASDAQ Global MarketSM
OPK,\"Opko Health, Inc. - Common Stock\",NASDAQ Global Select MarketSM
OPOF,Old Point Financial Corporation - Common Stock,NASDAQ Capital Market
OPRA,Opera Limited - American Depositary Shares,NASDAQ Global Select MarketSM
OPRT,Oportun Financial Corporation - common stock,NASDAQ Global Select MarketSM
OPRX,OptimizeRx Corporation - Common Stock,NASDAQ Capital Market
OPT,Opthea Limited - American Depositary Shares,NASDAQ Global Select MarketSM
OPTN,\"OptiNose, Inc. - Common Stock\",NASDAQ Global Select MarketSM
OPTX,\"Syntec Optics Holdings, Inc. - Class A Common Stock\",NASDAQ Capital Market
OPTXW,\"Syntec Optics Holdings, Inc. - Warrant\",NASDAQ Capital Market
OPTZ,Optimize Strategy Index ETF,NASDAQ Global MarketSM
OPXS,\"Optex Systems Holdings, Inc. - Common Stock\",NASDAQ Capital Market
ORGN,\"Origin Materials, Inc. - Class A Common Stock\",NASDAQ Capital Market
ORGNW,\"Origin Materials, Inc. - Warrant\",NASDAQ Capital Market
ORGO,Organogenesis Holdings Inc.  - Class A ,NASDAQ Capital Market
ORGS,Orgenesis Inc. - Common Stock,NASDAQ Capital Market
ORIC,\"Oric Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ORLY,\"O'Reilly Automotive, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ORMP,Oramed Pharmaceuticals Inc. - Common Stock,NASDAQ Capital Market
ORRF,Orrstown Financial Services Inc - Common Stock,NASDAQ Capital Market
OSBC,\"Old Second Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
OSIS,\"OSI Systems, Inc. - Common Stock\",NASDAQ Global Select MarketSM
OSPN,OneSpan Inc. - Common Stock,NASDAQ Capital Market
OSS,\"One Stop Systems, Inc. - Common Stock\",NASDAQ Capital Market
OST,\"Ostin Technology Group Co., Ltd. - Ordinary Shares\",NASDAQ Capital Market
OSUR,\"OraSure Technologies, Inc. - Common Stock\",NASDAQ Global Select MarketSM
OSW,OneSpaWorld Holdings Limited - Common Shares,NASDAQ Capital Market
OTEX,Open Text Corporation - Common Shares,NASDAQ Global Select MarketSM
OTLK,\"Outlook Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
OTLY,Oatly Group AB - American Depositary Shares,NASDAQ Global Select MarketSM
OTRK,\"Ontrak, Inc. - Common Stock\",NASDAQ Capital Market
OTTR,Otter Tail Corporation - Common Stock,NASDAQ Global Select MarketSM
OVBC,Ohio Valley Banc Corp. - Common Stock,NASDAQ Global MarketSM
OVID,Ovid Therapeutics Inc. - Common Stock,NASDAQ Global Select MarketSM
OVLY,Oak Valley Bancorp (CA) - Common Stock,NASDAQ Capital Market
OXBR,Oxbridge Re Holdings Limited - Ordinary Shares,NASDAQ Capital Market
OXBRW,Oxbridge Re Holdings Limited - Warrant,NASDAQ Capital Market
OXLC,Oxford Lane Capital Corp. - Closed End Fund,NASDAQ Global Select MarketSM
OXLCL,Oxford Lane Capital Corp. - 6.75% Notes due 2031,NASDAQ Global Select MarketSM
OXLCM,Oxford Lane Capital Corp. - 6.75% Series 2024 Term Preferred Stock,NASDAQ Global Select MarketSM
OXLCN,Oxford Lane Capital Corp. - 7.125% Series 2029 Term Preferred Stock,NASDAQ Global Select MarketSM
OXLCO,\"Oxford Lane Capital Corp. - Preferred Stock Shares, 6.00% Series 2029\",NASDAQ Global Select MarketSM
OXLCP,Oxford Lane Capital Corp. - 6.25% Series 2027 Term Preferred Shares,NASDAQ Global Select MarketSM
OXLCZ,Oxford Lane Capital Corp. - 5.00% Notes due 2027,NASDAQ Global Select MarketSM
OXSQ,Oxford Square Capital Corp. - Closed End Fund,NASDAQ Global Select MarketSM
OXSQG,Oxford Square Capital Corp. - 5.50% Notes due 2028,NASDAQ Global Select MarketSM
OXSQZ,Oxford Square Capital Corp. - 6.25% Notes due 2026,NASDAQ Global Select MarketSM
OZEM,Roundhill GLP-1 & Weight Loss ETF,NASDAQ Global MarketSM
OZK,Bank OZK - Common Stock,NASDAQ Global Select MarketSM
OZKAP,Bank OZK - 4.625% Series A Non-Cumulative Perpetual Preferred Stock,NASDAQ Global Select MarketSM
PAA,\"Plains All American Pipeline, L.P. - Common Units representing Limited Partner Interests\",NASDAQ Global Select MarketSM
PABD,iShares Paris-Aligned Climate MSCI World ex USA ETF,NASDAQ Global MarketSM
PABU,iShares Paris-Aligned Climate MSCI USA ETF,NASDAQ Global MarketSM
PACB,\"Pacific Biosciences of California, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PAGP,\"Plains GP Holdings, L.P. - Class A Shares representing limited partner interests\",NASDAQ Global Select MarketSM
PAHC,Phibro Animal Health Corporation - Class A Common Stock,NASDAQ Global MarketSM
PAL,\"Proficient Auto Logistics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PALI,\"Palisade Bio, Inc. - Common Stock\",NASDAQ Capital Market
PALT,\"Paltalk, Inc. - Common Stock\",NASDAQ Capital Market
PANL,Pangaea Logistics Solutions Ltd. - Common Stock,NASDAQ Capital Market
PANW,\"Palo Alto Networks, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PARA,Paramount Global - Class B Common Stock,NASDAQ Global Select MarketSM
PARAA,Paramount Global - Class A Common Stock,NASDAQ Global Select MarketSM
PASG,\"Passage Bio, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PATK,\"Patrick Industries, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PAVM,PAVmed Inc. - Common Stock,NASDAQ Capital Market
PAVMZ,PAVmed Inc. - Series Z Warrant,NASDAQ Capital Market
PAVS,Paranovus Entertainment Technology Ltd. - Class A Ordinary Shares,NASDAQ Capital Market
PAX,Patria Investments Limited - Class A Common Shares,NASDAQ Global Select MarketSM
PAYO,Payoneer Global Inc. - Common Stock,NASDAQ Global MarketSM
PAYOW,Payoneer Global Inc. - Warrant,NASDAQ Global MarketSM
PAYS,\"Paysign, Inc. - Common Stock\",NASDAQ Capital Market
PAYX,\"Paychex, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PBBK,\"PB Bankshares, Inc. - Common Stock\",NASDAQ Capital Market
PBFS,\"Pioneer Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
PBHC,\"Pathfinder Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
PBM,Psyence Biomedical Ltd. - Ordinary Shares,NASDAQ Global MarketSM
PBMWW,Psyence Biomedical Ltd. - Warrant,NASDAQ Capital Market
PBPB,Potbelly Corporation - Common Stock,NASDAQ Global Select MarketSM
PBYI,Puma Biotechnology Inc - Common Stock,NASDAQ Global Select MarketSM
PCAR,PACCAR Inc. - Common Stock,NASDAQ Global Select MarketSM
PCB,PCB Bancorp - Common Stock,NASDAQ Global Select MarketSM
PCH,PotlatchDeltic Corporation - Common Stock,NASDAQ Global Select MarketSM
PCRX,\"Pacira BioSciences, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PCSA,\"Processa Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
PCT,\"PureCycle Technologies, Inc. - Common stock\",NASDAQ Capital Market
PCTTU,\"PureCycle Technologies, Inc. - Unit\",NASDAQ Capital Market
PCTTW,\"PureCycle Technologies, Inc. - Warrant\",NASDAQ Capital Market
PCTY,Paylocity Holding Corporation - Common Stock,NASDAQ Global Select MarketSM
PCVX,\"Vaxcyte, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PCYO,Pure Cycle Corporation - Common Stock,NASDAQ Capital Market
PDBA,Invesco Agriculture Commodity Strategy No K-1 ETF,NASDAQ Global MarketSM
PDBC,Invesco Optimum Yield Diversified Commodity Strategy No K-1 ETF,NASDAQ Global MarketSM
PDCO,\"Patterson Companies, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PDD,PDD Holdings Inc. - American Depositary Shares,NASDAQ Global Select MarketSM
PDEX,\"Pro-Dex, Inc. - Common Stock\",NASDAQ Capital Market
PDFS,\"PDF Solutions, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PDLB,\"Ponce Financial Group, Inc. - Common Stock\",NASDAQ Global MarketSM
PDP,Invesco Dorsey Wright Momentum ETF,NASDAQ Global MarketSM
PDSB,PDS Biotechnology Corporation - Common Stock,NASDAQ Capital Market
PDYN,Palladyne AI Corp. - Common stock,NASDAQ Global MarketSM
PDYNW,Palladyne AI Corp. - Warrant,NASDAQ Global MarketSM
PEBK,\"Peoples Bancorp of North Carolina, Inc. - Common Stock\",NASDAQ Global MarketSM
PEBO,Peoples Bancorp Inc. - Common Stock,NASDAQ Global Select MarketSM
PECO,\"Phillips Edison & Company, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PEGA,Pegasystems Inc. - Common Stock,NASDAQ Global Select MarketSM
PEGR,Project Energy Reimagined Acquisition Corp. - Class A Ordinary Share,NASDAQ Global MarketSM
PEGRU,Project Energy Reimagined Acquisition Corp. - Unit,NASDAQ Global MarketSM
PEGRW,Project Energy Reimagined Acquisition Corp. - Warrant,NASDAQ Global MarketSM
PEGY,Pineapple Energy Inc. - Common Stock,NASDAQ Capital Market
PENN,\"PENN Entertainment, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PEP,\"PepsiCo, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PEPG,PepGen Inc. - Common Stock,NASDAQ Global Select MarketSM
PERI,Perion Network Ltd - Ordinary Shares,NASDAQ Global Select MarketSM
PESI,\"Perma-Fix Environmental Services, Inc. - Common Stock\",NASDAQ Capital Market
PET,Wag! Group Co. - Common Stock,NASDAQ Global MarketSM
PETQ,\"PetIQ, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
PETS,\"PetMed Express, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PETWW,Wag! Group Co. - Warrant,NASDAQ Global MarketSM
PETZ,\"TDH Holdings, Inc. - Common Shares\",NASDAQ Capital Market
PEV,Phoenix Motor Inc. - Common Stock,NASDAQ Capital Market
PEY,Invesco High Yield Equity Dividend Achievers ETF,NASDAQ Global MarketSM
PEZ,Invesco Dorsey Wright Consumer Cyclicals Momentum ETF,NASDAQ Global MarketSM
PFBC,Preferred Bank - Common Stock,NASDAQ Global Select MarketSM
PFC,Premier Financial Corp.  - Common Stock,NASDAQ Global Select MarketSM
PFF,iShares Preferred and Income Securities ETF,NASDAQ Global MarketSM
PFG,Principal Financial Group Inc - Common Stock,NASDAQ Global Select MarketSM
PFI,Invesco Dorsey Wright Financial Momentum ETF,NASDAQ Global MarketSM
PFIE,\"Profire Energy, Inc. - Common Stock\",NASDAQ Capital Market
PFIS,Peoples Financial Services Corp.  - Common Stock,NASDAQ Global Select MarketSM
PFM,Invesco Dividend Achievers ETF,NASDAQ Global MarketSM
PFMT,Performant Financial Corporation - Common Stock,NASDAQ Global Select MarketSM
PFTA,Perception Capital Corp. III - Class A Ordinary Share,NASDAQ Capital Market
PFTAU,Perception Capital Corp. III - Unit,NASDAQ Capital Market
PFTAW,Perception Capital Corp. III - Warrant,NASDAQ Capital Market
PFX,PhenixFIN Corporation  - Common Stock,NASDAQ Global MarketSM
PFXNZ,PhenixFIN Corporation  - 5.25% Notes due 2028,NASDAQ Global MarketSM
PGC,Peapack-Gladstone Financial Corporation - Common Stock,NASDAQ Global Select MarketSM
PGEN,\"Precigen, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PGJ,Invesco Golden Dragon China ETF,NASDAQ Global MarketSM
PGNY,\"Progyny, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PGY,Pagaya Technologies Ltd. - Class A Ordinary Shares,NASDAQ Capital Market
PGYWW,Pagaya Technologies Ltd. - Warrants,NASDAQ Capital Market
PHAR,\"Pharming Group N.V. - ADS, each representing 10 ordinary shares\",NASDAQ Global MarketSM
PHAT,\"Phathom Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PHIO,Phio Pharmaceuticals Corp. - Common Stock,NASDAQ Capital Market
PHO,Invesco Water Resources ETF,NASDAQ Global MarketSM
PHUN,\"Phunware, Inc. - Common Stock\",NASDAQ Capital Market
PHVS,Pharvaris N.V. - Ordinary Shares,NASDAQ Global Select MarketSM
PI,\"Impinj, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PID,Invesco International Dividend Achievers ETF,NASDAQ Global MarketSM
PIE,Invesco Dorsey Wright Emerging Markets Momentum ETF,NASDAQ Global MarketSM
PIII,P3 Health Partners Inc. - Class A Common Stock,NASDAQ Capital Market
PIIIW,P3 Health Partners Inc. - Warrant,NASDAQ Capital Market
PIK,Kidpik Corp. - Common Stock,NASDAQ Capital Market
PINC,\"Premier, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
PIO,Invesco Global Water ETF,NASDAQ Global MarketSM
PIRS,\"Pieris Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
PIXY,\"ShiftPixy, Inc. - Common Stock\",NASDAQ Capital Market
PIZ,Invesco Dorsey Wright Developed Markets Momentum ETF,NASDAQ Global MarketSM
PKBK,\"Parke Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
PKOH,Park-Ohio Holdings Corp. - Common Stock,NASDAQ Global Select MarketSM
PKW,Invesco BuyBack Achievers ETF,NASDAQ Global MarketSM
PLAB,\"Photronics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PLAO,Patria Latin American Opportunity Acquisition Corp. - Class A Ordinary Shares,NASDAQ Global MarketSM
PLAOU,Patria Latin American Opportunity Acquisition Corp. - Unit,NASDAQ Global MarketSM
PLAOW,Patria Latin American Opportunity Acquisition Corp. - Warrant,NASDAQ Global MarketSM
PLAY,\"Dave & Buster's Entertainment, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PLBC,Plumas Bancorp - Common Stock,NASDAQ Capital Market
PLBY,\"PLBY Group, Inc.  - Common Stock\",NASDAQ Global MarketSM
PLCE,\"Children's Place, Inc. (The) - Common Stock\",NASDAQ Global Select MarketSM
PLL,Piedmont Lithium Inc.   - Common Stock,NASDAQ Capital Market
PLMI,Plum Acquisition Corp. I - Class A Ordinary Share,NASDAQ Capital Market
PLMIU,Plum Acquisition Corp. I - Units,NASDAQ Capital Market
PLMIW,Plum Acquisition Corp. I - Warrant,NASDAQ Capital Market
PLMJ,Plum Acquisition Corp. III - Class A Ordinary Shares,NASDAQ Capital Market
PLMJU,Plum Acquisition Corp. III - Unit,NASDAQ Capital Market
PLMJW,Plum Acquisition Corp. III - Warrant,NASDAQ Capital Market
PLMR,\"Palomar Holdings, Inc. - Common stock\",NASDAQ Global Select MarketSM
PLPC,Preformed Line Products Company - Common Stock,NASDAQ Global Select MarketSM
PLRX,\"Pliant Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PLSE,\"Pulse Biosciences, Inc - Common Stock\",NASDAQ Capital Market
PLTK,Playtika Holding Corp. - Common Stock,NASDAQ Global Select MarketSM
PLTN,Plutonian Acquisition Corp. - Common Stock,NASDAQ Capital Market
PLTNR,Plutonian Acquisition Corp. - Rights,NASDAQ Capital Market
PLTNU,Plutonian Acquisition Corp. - Unit,NASDAQ Capital Market
PLTNW,Plutonian Acquisition Corp. - Warrant,NASDAQ Capital Market
PLUG,\"Plug Power, Inc. - Common Stock\",NASDAQ Capital Market
PLUR,Pluri Inc. - Common Stock,NASDAQ Capital Market
PLUS,ePlus inc. - Common Stock,NASDAQ Global Select MarketSM
PLXS,Plexus Corp. - Common Stock,NASDAQ Global Select MarketSM
PLYA,Playa Hotels & Resorts N.V. - Ordinary Shares,NASDAQ Global Select MarketSM
PMCB,\"PharmaCyte  Biotech, Inc. - Common Stock\",NASDAQ Capital Market
PMD,Psychemedics Corporation - Common Stock,NASDAQ Capital Market
PMEC,Primech Holdings Ltd. - Ordinary Shares,NASDAQ Capital Market
PMN,ProMIS Neurosciences Inc. - Common Shares,NASDAQ Capital Market
PMTS,CPI Card Group Inc. - Common Stock,NASDAQ Global MarketSM
PMVP,\"PMV Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PNBK,Patriot National Bancorp Inc. - Common Stock,NASDAQ Global MarketSM
PNFP,\"Pinnacle Financial Partners, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PNFPP,\"Pinnacle Financial Partners, Inc. - Depositary shares of Pinnacle Financial Partners, Inc., each representing a 1/40th Interest in a share of its 6.75% Fixed-Rate Non-Cumulative Perpetual Preferred Stock, Series B\",NASDAQ Global Select MarketSM
PNQI,Invesco Nasdaq Internet ETF,NASDAQ Global MarketSM
PNRG,PrimeEnergy Resources Corporation - Common Stock,NASDAQ Capital Market
PNTG,\"The Pennant Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
POAI,Predictive Oncology Inc. - Common Stock,NASDAQ Capital Market
POCI,\"Precision Optics Corporation, Inc. - Common stock\",NASDAQ Capital Market
PODC,\"PodcastOne, Inc. - Common Stock\",NASDAQ Capital Market
PODD,Insulet Corporation - Common Stock,NASDAQ Global Select MarketSM
POET,POET Technologies Inc. - Common Shares,NASDAQ Capital Market
POLA,\"Polar Power, Inc. - Common Stock\",NASDAQ Capital Market
POOL,Pool Corporation - Common Stock,NASDAQ Global Select MarketSM
POWI,\"Power Integrations, Inc. - Common Stock\",NASDAQ Global Select MarketSM
POWL,\"Powell Industries, Inc. - Common Stock\",NASDAQ Global Select MarketSM
POWW,\"AMMO, Inc. - Common Stock\",NASDAQ Capital Market
POWWP,\"AMMO, Inc. - 8.75% Series A Cumulative Redeemable Perpetual Preferred Stock\",NASDAQ Capital Market
PPBI,Pacific Premier Bancorp Inc - Common Stock,NASDAQ Global Select MarketSM
PPBT,Purple Biotech Ltd. - American Depositary Shares,NASDAQ Capital Market
PPC,Pilgrim's Pride Corporation - Common Stock,NASDAQ Global Select MarketSM
PPH,VanEck Pharmaceutical ETF,NASDAQ Global MarketSM
PPIH,\"Perma-Pipe International Holdings, Inc. - Common Stock\",NASDAQ Global MarketSM
PPSI,\"Pioneer Power Solutions, Inc. - Common Stock\",NASDAQ Capital Market
PPTA,Perpetua Resources Corp. - Common Shares,NASDAQ Capital Market
PPYA,Papaya Growth Opportunity Corp. I - Class A Common Stock,NASDAQ Global MarketSM
PPYAU,Papaya Growth Opportunity Corp. I - Unit,NASDAQ Global MarketSM
PPYAW,Papaya Growth Opportunity Corp. I - Warrant,NASDAQ Global MarketSM
PRAA,\"PRA Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PRAX,\"Praxis Precision Medicines, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PRCH,\"Porch Group, Inc. - Common Stock\",NASDAQ Capital Market
PRCT,PROCEPT BioRobotics Corporation - Common Stock,NASDAQ Global MarketSM
PRDO,Perdoceo Education Corporation - Common Stock,NASDAQ Global Select MarketSM
PRE,Prenetics Global Limited - Class A Ordinary Share,NASDAQ Global MarketSM
PRENW,Prenetics Global Limited - Warrant,NASDAQ Capital Market
PRFT,\"Perficient, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PRFX,PainReform Ltd. - Ordinary Shares,NASDAQ Capital Market
PRFZ,Invesco FTSE RAFI US 1500 Small-Mid ETF,NASDAQ Global MarketSM
PRGS,Progress Software Corporation - Common Stock,NASDAQ Global Select MarketSM
PRLD,Prelude Therapeutics Incorporated - Common Stock,NASDAQ Global Select MarketSM
PRLH,Pearl Holdings Acquisition Corp - Class A Ordinary Shares,NASDAQ Global MarketSM
PRLHU,Pearl Holdings Acquisition Corp - Unit,NASDAQ Global MarketSM
PRLHW,Pearl Holdings Acquisition Corp - Warrant,NASDAQ Global MarketSM
PRME,\"Prime Medicine, Inc. - Common Stock\",NASDAQ Global MarketSM
PRN,Invesco Dorsey Wright Industrials Momentum ETF,NASDAQ Global MarketSM
PROC,\"Procaps Group, S.A. - Ordinary Shares\",NASDAQ Global MarketSM
PROCW,\"Procaps Group, S.A. - Warrants\",NASDAQ Global MarketSM
PROF,Profound Medical Corp. - common stock,NASDAQ Capital Market
PROK,ProKidney Corp. - Class A Ordinary Shares,NASDAQ Capital Market
PROP,Prairie Operating Co. - Common Stock,NASDAQ Capital Market
PROV,\"Provident Financial Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PRPH,\"ProPhase Labs, Inc. - Common Stock\",NASDAQ Capital Market
PRPL,\"Purple Innovation, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PRPO,\"Precipio, Inc. - Common Stock\",NASDAQ Capital Market
PRQR,ProQR Therapeutics N.V. - Ordinary Shares,NASDAQ Capital Market
PRSO,Peraso Inc. - Common Stock,NASDAQ Capital Market
PRST,\"Presto Automation, Inc. - Common Stock\",NASDAQ Global MarketSM
PRSTW,\"Presto Automation, Inc. - Warrant\",NASDAQ Global MarketSM
PRTA,Prothena Corporation plc - Ordinary Shares,NASDAQ Global Select MarketSM
PRTC,PureTech Health plc - American Depositary Shares,NASDAQ Global MarketSM
PRTG,Portage Biotech Inc. - Common Stock,NASDAQ Capital Market
PRTH,\"Priority Technology Holdings, Inc. - Common Stock\",NASDAQ Capital Market
PRTS,\"CarParts.com, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PRVA,\"Privia Health Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PRZO,ParaZero Technologies Ltd. - Ordinary Shares,NASDAQ Capital Market
PSC,Principal U.S. Small-Cap ETF,NASDAQ Global MarketSM
PSCC,Invesco S&P SmallCap Consumer Staples ETF,NASDAQ Global MarketSM
PSCD,Invesco S&P SmallCap Consumer Discretionary ETF,NASDAQ Global MarketSM
PSCE,Invesco S&P SmallCap Energy ETF,NASDAQ Global MarketSM
PSCF,Invesco S&P SmallCap Financials ETF,NASDAQ Global MarketSM
PSCH,Invesco S&P SmallCap Health Care ETF,NASDAQ Global MarketSM
PSCI,Invesco S&P SmallCap Industrials ETF,NASDAQ Global MarketSM
PSCM,Invesco S&P SmallCap Materials ETF,NASDAQ Global MarketSM
PSCT,Invesco S&P SmallCap Information Technology ETF,NASDAQ Global MarketSM
PSCU,Invesco S&P SmallCap Utilities & Communication Services ETF,NASDAQ Global MarketSM
PSEC,Prospect Capital Corporation - Closed End Fund,NASDAQ Global Select MarketSM
PSET,Principal Quality ETF,NASDAQ Global MarketSM
PSHG,Performance Shipping Inc. - Common Shares,NASDAQ Capital Market
PSL,Invesco Dorsey Wright Consumer Staples Momentum ETF,NASDAQ Global MarketSM
PSMT,\"PriceSmart, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PSNL,\"Personalis, Inc. - Common Stock\",NASDAQ Global MarketSM
PSNY,Polestar Automotive Holding UK Limited - Class A ADS,NASDAQ Global MarketSM
PSNYW,Polestar Automotive Holding UK Limited - Class C-1 ADS (ADW),NASDAQ Global MarketSM
PSTR,PeakShares Sector Rotation ETF,NASDAQ Global MarketSM
PSTV,\"PLUS THERAPEUTICS, Inc. - Common Stock\",NASDAQ Capital Market
PSTX,\"Poseida Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PSWD,Xtrackers Cybersecurity Select Equity ETF,NASDAQ Global MarketSM
PT,Pintec Technology Holdings Limited - American Depositary Shares,NASDAQ Global MarketSM
PTC,PTC Inc. - Common Stock,NASDAQ Global Select MarketSM
PTCT,\"PTC Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PTEC,Global X PropTech ETF,NASDAQ Global MarketSM
PTEN,\"Patterson-UTI Energy, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PTF,Invesco Dorsey Wright Technology Momentum ETF,NASDAQ Global MarketSM
PTGX,\"Protagonist Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
PTH,Invesco Dorsey Wright Healthcare Momentum ETF,NASDAQ Global MarketSM
PTIX,\"Protagenic Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
PTIXW,\"Protagenic Therapeutics, Inc. - Warrant\",NASDAQ Capital Market
PTLO,Portillo's Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
PTMN,Portman Ridge Finance Corporation - Closed End Fund,NASDAQ Global Select MarketSM
PTNQ,Pacer Trendpilot 100 ETF,NASDAQ Global MarketSM
PTON,\"Peloton Interactive, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PTPI,\"Petros Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
PTSI,\"P.A.M. Transportation Services, Inc. - Common Stock\",NASDAQ Global MarketSM
PTVE,Pactiv Evergreen Inc. - Common stock,NASDAQ Global Select MarketSM
PTWO,\"Pono Capital Two, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
PTWOU,\"Pono Capital Two, Inc. - Unit\",NASDAQ Global MarketSM
PTWOW,\"Pono Capital Two, Inc. - Warrants\",NASDAQ Global MarketSM
PUBM,\"PubMatic, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
PUCK,Goal Acquisitions Corp. - Common Stock,NASDAQ Capital Market
PUCKU,Goal Acquisitions Corp. - Unit,NASDAQ Capital Market
PUCKW,Goal Acquisitions Corp. - Warrant,NASDAQ Capital Market
PUI,Invesco Dorsey Wright Utilities Momentum ETF,NASDAQ Global MarketSM
PULM,\"Pulmatrix, Inc. - Common Stock\",NASDAQ Capital Market
PVBC,\"Provident Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
PWFL,\"PowerFleet, Inc. - Common Stock\",NASDAQ Global MarketSM
PWM,Prestige Wealth Inc. - Class A Ordinary Shares,NASDAQ Capital Market
PWOD,\"Penns Woods Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PWP,Perella Weinberg Partners - Class A Common Stock,NASDAQ Global Select MarketSM
PWUP,PowerUp Acquisition Corp. - Class A Ordinary Shares,NASDAQ Global MarketSM
PWUPU,PowerUp Acquisition Corp. - Unit,NASDAQ Global MarketSM
PWUPW,PowerUp Acquisition Corp. - Warrant,NASDAQ Global MarketSM
PXDT,\"Pixie Dust Technologies, Inc. - ADS, each representing one common share\",NASDAQ Capital Market
PXI,Invesco Dorsey Wright Energy Momentum ETF,NASDAQ Global MarketSM
PXLW,\"Pixelworks, Inc. - Common Stock\",NASDAQ Global MarketSM
PXS,Pyxis Tankers Inc. - Common Stock,NASDAQ Capital Market
PXSAP,Pyxis Tankers Inc. - 7.75% Series A Cumulative Convertible Preferred Shares,NASDAQ Capital Market
PXSAW,Pyxis Tankers Inc. - Warrant,NASDAQ Capital Market
PY,Principal Value ETF,NASDAQ Global MarketSM
PYCR,\"Paycor HCM, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PYPD,PolyPid Ltd. - Ordinary Shares,NASDAQ Capital Market
PYPL,\"PayPal Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PYXS,\"Pyxis Oncology, Inc. - Common Stock\",NASDAQ Global Select MarketSM
PYZ,Invesco Dorsey Wright Basic Materials Momentum ETF,NASDAQ Global MarketSM
PZZA,\"Papa John's International, Inc. - Common Stock\",NASDAQ Global Select MarketSM
QABA,First Trust NASDAQ ABA Community Bank Index Fund,NASDAQ Global MarketSM
QAT,iShares MSCI Qatar ETF,NASDAQ Global MarketSM
QCLN,First Trust NASDAQ Clean Edge Green Energy Index Fund,NASDAQ Global MarketSM
QCLR,Global X NASDAQ 100 Collar 95-110 ETF,NASDAQ Global MarketSM
QCOM,QUALCOMM Incorporated - Common Stock,NASDAQ Global Select MarketSM
QCRH,\"QCR Holdings, Inc. - Common Stock\",NASDAQ Global MarketSM
QDEL,QuidelOrtho Corporation - Common Stock,NASDAQ Global Select MarketSM
QDRO,Quadro Acquisition One Corp. - Class A Ordinary Shares,NASDAQ Capital Market
QDROU,Quadro Acquisition One Corp. - Unit,NASDAQ Capital Market
QDROW,Quadro Acquisition One Corp. - Warrant,NASDAQ Capital Market
QETA,Quetta Acquisition Corporation - Common Stock,NASDAQ Global MarketSM
QETAR,Quetta Acquisition Corporation - Right,NASDAQ Global MarketSM
QETAU,Quetta Acquisition Corporation - Unit,NASDAQ Global MarketSM
QFIN,\"Qifu Technology, Inc - American Depositary Shares\",NASDAQ Global Select MarketSM
QH,Quhuo Limited - American Depository Shares,NASDAQ Global MarketSM
QIPT,Quipt Home Medical Corp. - Common Shares,NASDAQ Capital Market
QIWI,QIWI plc - American Depositary Shares,NASDAQ Global Select MarketSM
QLGN,\"Qualigen Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
QLI,Qilian International Holding Group Ltd. - Ordinary Shares,NASDAQ Capital Market
QLYS,\"Qualys, Inc. - Common Stock\",NASDAQ Global Select MarketSM
QMCO,Quantum Corporation - Common Stock,NASDAQ Global MarketSM
QMID,WisdomTree U.S. MidCap Quality Growth Fund,NASDAQ Global MarketSM
QMOM,Alpha Architect U.S. Quantitative Momentum ETF,NASDAQ Global MarketSM
QNCX,\"Quince Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
QNRX,\"Quoin Pharmaceuticals, Ltd. - American Depositary Shares\",NASDAQ Capital Market
QNST,\"QuinStreet, Inc. - Common Stock\",NASDAQ Global Select MarketSM
QOMO,Qomolangma Acquisition Corp. - Common Stock,NASDAQ Capital Market
QOMOR,Qomolangma Acquisition Corp. - Right,NASDAQ Capital Market
QOMOU,Qomolangma Acquisition Corp. - Unit,NASDAQ Capital Market
QOMOW,Qomolangma Acquisition Corp. - Warrant,NASDAQ Capital Market
QOWZ,Invesco Nasdaq Free Cash Flow Achievers ETF,NASDAQ Global MarketSM
QQEW,First Trust NASDAQ-100 Equal Weighted Index Fund,NASDAQ Global MarketSM
QQJG,Invesco ESG NASDAQ Next Gen 100 ETF,NASDAQ Global MarketSM
QQMG,Invesco ESG NASDAQ 100 ETF,NASDAQ Global MarketSM
QQQ,\"Invesco QQQ Trust, Series 1\",NASDAQ Global MarketSM
QQQA,ProShares Nasdaq-100 Dorsey Wright Momentum ETF,NASDAQ Global MarketSM
QQQE,Direxion NASDAQ-100 Equal Weighted Index Shares,NASDAQ Global MarketSM
QQQI,NEOS Nasdaq 100 High Income ETF,NASDAQ Global MarketSM
QQQJ,Invesco NASDAQ Next Gen 100 ETF,NASDAQ Global MarketSM
QQQM,Invesco NASDAQ 100 ETF,NASDAQ Global MarketSM
QQQN,VictoryShares Nasdaq Next 50 ETF,NASDAQ Global MarketSM
QQQS,Invesco NASDAQ Future Gen 200 ETF,NASDAQ Global MarketSM
QQQX,Nuveen NASDAQ 100 Dynamic Overwrite Fund - Closed End Fund,NASDAQ Global Select MarketSM
QQQY,Defiance Nasdaq 100 Enhanced Options Income ETF,NASDAQ Global MarketSM
QQXT,First Trust NASDAQ-100 Ex-Technology Sector Index Fund,NASDAQ Global MarketSM
QRHC,Quest Resource Holding Corporation - Common Stock,NASDAQ Capital Market
QRMI,Global X NASDAQ 100 Risk Managed Income ETF,NASDAQ Global MarketSM
QRTEA,\"Qurate Retail, Inc. - Series A Common Stock\",NASDAQ Global Select MarketSM
QRTEB,\"Qurate Retail, Inc. - Series B Common Stock\",NASDAQ Global Select MarketSM
QRTEP,\"Qurate Retail, Inc. - 8.0% Fixed Rate Cumulative Redeemable Preferred Stock\",NASDAQ Global Select MarketSM
QRVO,\"Qorvo, Inc. - Common Stock\",NASDAQ Global Select MarketSM
QSG,QuantaSing Group Limited - American Depositary Shares,NASDAQ Global MarketSM
QSI,Quantum-Si Incorporated - Class A Common Stock,NASDAQ Global MarketSM
QSIAW,Quantum-Si Incorporated - Warrant,NASDAQ Global MarketSM
QSML,WisdomTree U.S. SmallCap Quality Growth Fund,NASDAQ Global MarketSM
QTEC,First Trust NASDAQ-100-Technology Sector Index Fund,NASDAQ Global MarketSM
QTI,\"QT Imaging Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
QTR,Global X NASDAQ 100 Tail Risk ETF,NASDAQ Global MarketSM
QTRX,Quanterix Corporation - Common Stock,NASDAQ Global MarketSM
QTTB,Q32 Bio Inc. - Common Stock,NASDAQ Global MarketSM
QUBT,Quantum Computing Inc. - Common Stock,NASDAQ Capital Market
QUIK,QuickLogic Corporation - Common Stock,NASDAQ Capital Market
QURE,uniQure N.V. - Ordinary Shares,NASDAQ Global Select MarketSM
QVAL,Alpha Architect U.S. Quantitative Value ETF,NASDAQ Global MarketSM
QYLD,Global X NASDAQ 100 Covered Call ETF,NASDAQ Global MarketSM
QYLE,Global X Nasdaq 100 ESG Covered Call ETF,NASDAQ Global MarketSM
QYLG,Global X Nasdaq 100 Covered Call & Growth ETF,NASDAQ Global MarketSM
RAIL,\"Freightcar America, Inc. - Common Stock\",NASDAQ Global Select MarketSM
RAND,Rand Capital Corporation - Closed End Fund,NASDAQ Capital Market
RANI,\"Rani Therapeutics Holdings, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
RAPT,\"RAPT Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
RARE,Ultragenyx Pharmaceutical Inc. - Common Stock,NASDAQ Global Select MarketSM
RAVE,\"Rave Restaurant Group, Inc. - Common Stock\",NASDAQ Capital Market
RAY,Raytech Holding Limited - ordinary shares,NASDAQ Capital Market
RAYA,Erayak Power Solution Group Inc. - Class A Ordinary Shares,NASDAQ Capital Market
RAYS,Global X Solar ETF,NASDAQ Global MarketSM
RBB,RBB Bancorp - Common Stock,NASDAQ Global Select MarketSM
RBBN,Ribbon Communications Inc.  - Common Stock,NASDAQ Global Select MarketSM
RBCAA,\"Republic Bancorp, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
RBKB,\"Rhinebeck Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
RCAT,\"Red Cat Holdings, Inc. - Common Stock\",NASDAQ Capital Market
RCEL,\"Avita Medical, Inc. - Common Stock\",NASDAQ Capital Market
RCKT,\"Rocket Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global MarketSM
RCKTW,\"Rocket Pharmaceuticals, Inc. - Warrant\",NASDAQ Capital Market
RCKY,\"Rocky Brands, Inc. - Common Stock\",NASDAQ Global Select MarketSM
RCM,R1 RCM Inc. - Common Stock,NASDAQ Global Select MarketSM
RCMT,\"RCM Technologies, Inc. - Common Stock\",NASDAQ Global MarketSM
RCON,\"Recon Technology, Ltd. - Class A Ordinary Shares\",NASDAQ Capital Market
RCRT,\"Recruiter.com Group, Inc. - Common Stock\",NASDAQ Capital Market
RCRTW,\"Recruiter.com Group, Inc. - Warrant\",NASDAQ Capital Market
RDCM,Radcom Ltd. - Ordinary Shares,NASDAQ Capital Market
RDFN,Redfin Corporation - Common Stock,NASDAQ Global Select MarketSM
RDHL,Redhill Biopharma Ltd. - American Depositary Shares,NASDAQ Capital Market
RDI,Reading International Inc - Class A Non-voting Common Stock,NASDAQ Capital Market
RDIB,Reading International Inc - Class B Voting Common Stock,NASDAQ Capital Market
RDNT,\"RadNet, Inc. - Common Stock\",NASDAQ Global MarketSM
RDUS,\"Radius Recycling, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
RDVT,\"Red Violet, Inc. - Common Stock \",NASDAQ Capital Market
RDVY,First Trust Rising Dividend Achievers ETF,NASDAQ Global MarketSM
RDWR,Radware Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
RDZN,\"Roadzen, Inc. - Ordinary Shares\",NASDAQ Global MarketSM
RDZNW,\"Roadzen, Inc. - Warrants\",NASDAQ Capital Market
REAI,Intelligent Real Estate ETF,NASDAQ Global MarketSM
REAL,\"The RealReal, Inc. - Common Stock\",NASDAQ Global Select MarketSM
REAX,\"The Real Brokerage, Inc. - Common Shares\",NASDAQ Capital Market
REBN,\"Reborn Coffee, Inc. - Common Stock\",NASDAQ Capital Market
REE,REE Automotive Ltd. - Class A Ordinary Shares,NASDAQ Capital Market
REFI,\"Chicago Atlantic Real Estate Finance, Inc. - Common Stock\",NASDAQ Global MarketSM
REFR,Research Frontiers Incorporated - Common Stock,NASDAQ Capital Market
REG,Regency Centers Corporation - Common Stock,NASDAQ Global Select MarketSM
REGCO,Regency Centers Corporation - 5.875% Series B Cumulative Redeemable Preferred Stock,NASDAQ Global Select MarketSM
REGCP,Regency Centers Corporation - 6.25% Series A Cumulative Redeemable Preferred Stock,NASDAQ Global Select MarketSM
REGN,\"Regeneron Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
REIT,ALPS Active REIT ETF,NASDAQ Global MarketSM
REKR,\"Rekor Systems, Inc. - Common Stock\",NASDAQ Capital Market
RELI,\"Reliance Global Group, Inc. - Common Stock\",NASDAQ Capital Market
RELIW,\"Reliance Global Group, Inc. - Series A Warrants\",NASDAQ Capital Market
RELL,\"Richardson Electronics, Ltd. - Common Stock\",NASDAQ Global Select MarketSM
RELY,\"Remitly Global, Inc. - Common stock\",NASDAQ Global Select MarketSM
RENB,Renovaro Inc. - Common Stock,NASDAQ Capital Market
RENE,Cartesian Growth Corporation II - Class A Ordinary Shares,NASDAQ Global MarketSM
RENEU,Cartesian Growth Corporation II - Unit,NASDAQ Global MarketSM
RENEW,Cartesian Growth Corporation II - Warrant,NASDAQ Global MarketSM
RENT,\"Rent the Runway, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
REPL,\"Replimune Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
RETO,\"ReTo Eco-Solutions, Inc. - Common Shares\",NASDAQ Capital Market
REVB,\"Revelation Biosciences, Inc. - Common Stock\",NASDAQ Capital Market
REVBW,\"Revelation Biosciences, Inc. - Warrant\",NASDAQ Capital Market
REYN,Reynolds Consumer Products Inc. - Common Stock,NASDAQ Global Select MarketSM
RFAC,RF Acquisition Corp. - Class A Common Stock,NASDAQ Global MarketSM
RFACR,RF Acquisition Corp. - Rights,NASDAQ Global MarketSM
RFACU,RF Acquisition Corp. - Unit,NASDAQ Global MarketSM
RFACW,RF Acquisition Corp. - Warrants,NASDAQ Global MarketSM
RFAIU,RF Acquisition Corp II - Unit,NASDAQ Global MarketSM
RFDI,First Trust RiverFront Dynamic Developed International ETF,NASDAQ Global MarketSM
RFEM,First Trust RiverFront Dynamic Emerging Markets ETF,NASDAQ Global MarketSM
RFEU,First Trust RiverFront Dynamic Europe ETF,NASDAQ Global MarketSM
RFIL,\"RF Industries, Ltd. - Common Stock\",NASDAQ Global MarketSM
RGC,Regencell Bioscience Holdings Limited - Ordinary Shares,NASDAQ Capital Market
RGCO,RGC Resources Inc. - Common Stock,NASDAQ Global MarketSM
RGEN,Repligen Corporation - Common Stock,NASDAQ Global Select MarketSM
RGF,\"The Real Good Food Company, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
RGLD,\"Royal Gold, Inc. - Common Stock\",NASDAQ Global Select MarketSM
RGLS,Regulus Therapeutics Inc. - Common Stock,NASDAQ Capital Market
RGNX,REGENXBIO Inc. - Common Stock,NASDAQ Global Select MarketSM
RGP,\"Resources Connection, Inc. - Common Stock\",NASDAQ Global Select MarketSM
RGS,Regis Corporation - Common Stock,NASDAQ Global MarketSM
RGTI,\"Rigetti Computing, Inc.  - Common stock\",NASDAQ Capital Market
RGTIW,\"Rigetti Computing, Inc.  - Redeemable warrants, each whole warrant exercisable for one Class A ordinary share at an exercise price of $11.50\",NASDAQ Capital Market
RICK,\"RCI Hospitality Holdings, Inc. - Common Stock\",NASDAQ Global MarketSM
RIGL,\"Rigel Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
RILY,\"B. Riley Financial, Inc. - Common Stock\",NASDAQ Global MarketSM
RILYG,\"B. Riley Financial, Inc. - 5.00% Senior Notes due 2026\",NASDAQ Global MarketSM
RILYK,\"B. Riley Financial, Inc. - 5.50% Senior Notes Due 2026\",NASDAQ Global MarketSM
RILYL,\"B. Riley Financial, Inc. - Depositary Shares, each representing a 1/1000th fractional interest in a share of Series B Cumulative Perpetual Preferred Stock\",NASDAQ Global MarketSM
RILYM,\"B. Riley Financial, Inc. - 6.375% Senior Notes due 2025\",NASDAQ Global MarketSM
RILYN,\"B. Riley Financial, Inc. - 6.50% Senior Notes Due 2026\",NASDAQ Global MarketSM
RILYO,\"B. Riley Financial, Inc. - 6.75% Senior Notes due 2024\",NASDAQ Global MarketSM
RILYP,\"B. Riley Financial, Inc. - Depositary Shares, each representing a 1/1000th fractional interest in a share of Series A Cumulative Perpetual Preferred Stock\",NASDAQ Global MarketSM
RILYT,\"B. Riley Financial, Inc. - 6.00% Senior Notes Due 2028\",NASDAQ Global MarketSM
RILYZ,\"B. Riley Financial, Inc. - 5.25% Senior Notes due 2028\",NASDAQ Global MarketSM
RING,iShares MSCI Global Gold Miners ETF,NASDAQ Global MarketSM
RIOT,\"Riot Platforms, Inc. - Common Stock\",NASDAQ Capital Market
RIVN,\"Rivian Automotive, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
RKDA,\"Arcadia Biosciences, Inc. - Common Stock\",NASDAQ Capital Market
RKLB,\"Rocket Lab USA, Inc. - Common Stock\",NASDAQ Capital Market
RLAY,\"Relay Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
RLMD,\"Relmada Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
RLYB,Rallybio Corporation - Common Stock,NASDAQ Global Select MarketSM
RMBI,\"Richmond Mutual Bancorporation, Inc. - Common Stock\",NASDAQ Capital Market
RMBL,\"RumbleOn, Inc. - Class B Common Stock\",NASDAQ Capital Market
RMBS,\"Rambus, Inc. - Common Stock\",NASDAQ Global Select MarketSM
RMCF,\"Rocky Mountain Chocolate Factory, Inc. - Common Stock\",NASDAQ Global MarketSM
RMCO,Royalty Management Holding Corporation - Class A Common Stock,NASDAQ Capital Market
RMCOW,Royalty Management Holding Corporation - Warrant,NASDAQ Capital Market
RMNI,\"Rimini Street, Inc. - Common Stock\",NASDAQ Global MarketSM
RMR,The RMR Group Inc. - Class A Common Stock,NASDAQ Capital Market
RMTI,\"Rockwell Medical, Inc. - Common Stock\",NASDAQ Capital Market
RNA,\"Avidity Biosciences, Inc. - Common Stock\",NASDAQ Global MarketSM
RNAC,\"Cartesian Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
RNAZ,\"TransCode Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
RNEM,Emerging Markets Equity Select ETF,NASDAQ Global MarketSM
RNEW,VanEck Green Infrastructure ETF,NASDAQ Global MarketSM
RNLX,Renalytix plc - American Depositary Shares,NASDAQ Global MarketSM
RNMC,Mid Cap US Equity Select ETF,NASDAQ Global MarketSM
RNRG,Global X Renewable Energy Producers ETF,NASDAQ Global MarketSM
RNSC,Small Cap US Equity Select ETF,NASDAQ Global MarketSM
RNW,ReNew Energy Global plc - Class A Shares,NASDAQ Global Select MarketSM
RNWWW,ReNew Energy Global plc - Warrant,NASDAQ Global Select MarketSM
RNXT,\"RenovoRx, Inc. - Common Stock\",NASDAQ Capital Market
ROAD,\"Construction Partners, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ROBT,First Trust Nasdaq Artificial Intelligence and Robotics ETF,NASDAQ Global MarketSM
ROCK,\"Gibraltar Industries, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ROCL,Roth CH Acquisition V Co. - Common Stock,NASDAQ Global MarketSM
ROCLU,Roth CH Acquisition V Co. - Unit,NASDAQ Global MarketSM
ROCLW,Roth CH Acquisition V Co. - Warrant,NASDAQ Global MarketSM
ROE,Astoria US Equal Weight Quality Kings ETF,NASDAQ Global MarketSM
ROIC,Retail Opportunity Investments Corp. - Common Stock,NASDAQ Global Select MarketSM
ROIV,Roivant Sciences Ltd. - Common Shares,NASDAQ Global Select MarketSM
ROKU,\"Roku, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
ROMA,Roma Green Finance Limited - Ordinary Shares,NASDAQ Capital Market
ROOT,\"Root, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ROP,\"Roper Technologies, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ROST,\"Ross Stores, Inc. - Common Stock\",NASDAQ Global Select MarketSM
RPAY,Repay Holdings Corporation - Class A Common Stock,NASDAQ Capital Market
RPD,\"Rapid7, Inc. - Common Stock\",NASDAQ Global MarketSM
RPHM,\"Reneo Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global MarketSM
RPID,\"Rapid Micro Biosystems, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
RPRX,Royalty Pharma plc - Class A Ordinary Shares,NASDAQ Global Select MarketSM
RPTX,Repare Therapeutics Inc. - Common Shares,NASDAQ Global Select MarketSM
RR,Richtech Robotics Inc. - Class B Common Stock,NASDAQ Capital Market
RRBI,\"Red River Bancshares, Inc. - Common Stock\",NASDAQ Global Select MarketSM
RRGB,\"Red Robin Gourmet Burgers, Inc. - Common Stock\",NASDAQ Global Select MarketSM
RRR,\"Red Rock Resorts, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
RSLS,\"ReShape Lifesciences, Inc. - Common Stock\",NASDAQ Capital Market
RSSS,\"Research Solutions, Inc - Common Stock\",NASDAQ Capital Market
RSVR,\"Reservoir Media, Inc.. - Common Stock\",NASDAQ Global MarketSM
RSVRW,\"Reservoir Media, Inc.. - Warrant\",NASDAQ Global MarketSM
RTC,Baijiayun Group Ltd - Class Ordinary Shares,NASDAQ Global MarketSM
RTH,VanEck Retail ETF,NASDAQ Global MarketSM
RUM,Rumble Inc. - Class A Common Stock,NASDAQ Global MarketSM
RUMBW,Rumble Inc. - Warrant,NASDAQ Global MarketSM
RUN,Sunrun Inc. - Common Stock,NASDAQ Global Select MarketSM
RUNN,Running Oak Efficient Growth ETF,NASDAQ Global MarketSM
RUSHA,\"Rush Enterprises, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
RUSHB,\"Rush Enterprises, Inc. - Class B Common Stock\",NASDAQ Global Select MarketSM
RVMD,\"Revolution Medicines, Inc. - Common Stock\",NASDAQ Global Select MarketSM
RVMDW,\"Revolution Medicines, Inc. - Warrant\",NASDAQ Global Select MarketSM
RVNC,\"Revance Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
RVPH,\"Reviva Pharmaceuticals Holdings, Inc. - Common Stock\",NASDAQ Capital Market
RVPHW,\"Reviva Pharmaceuticals Holdings, Inc. - Warrants\",NASDAQ Capital Market
RVSB,Riverview Bancorp Inc - Common Stock,NASDAQ Global Select MarketSM
RVSN,Rail Vision Ltd. - Ordinary Shares,NASDAQ Capital Market
RVSNW,Rail Vision Ltd. - Warrant,NASDAQ Capital Market
RVYL,Ryvyl Inc. - Common Stock,NASDAQ Capital Market
RWAY,Runway Growth Finance Corp. - Common Stock,NASDAQ Global Select MarketSM
RWAYL,Runway Growth Finance Corp. - 7.50% Notes due 2027,NASDAQ Global Select MarketSM
RWAYZ,Runway Growth Finance Corp. - 8.00% Notes due 2027,NASDAQ Global Select MarketSM
RWOD,Redwoods Acquisition Corp. - Common Stock,NASDAQ Global MarketSM
RWODR,Redwoods Acquisition Corp. - Rights,NASDAQ Global MarketSM
RWODU,Redwoods Acquisition Corp. - Unit,NASDAQ Global MarketSM
RWODW,Redwoods Acquisition Corp. - Warrants,NASDAQ Global MarketSM
RXRX,\"Recursion Pharmaceuticals, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
RXST,\"RxSight, Inc. - Common Stock\",NASDAQ Global MarketSM
RXT,\"Rackspace Technology, Inc. - Common Stock\",NASDAQ Global Select MarketSM
RYAAY,\"Ryanair Holdings plc - American Depositary Shares, each representing five Ordinary Shares\",NASDAQ Global Select MarketSM
RYTM,\"Rhythm Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global MarketSM
RZLT,\"Rezolute, Inc. - Common Stock (NV)\",NASDAQ Capital Market
SABR,Sabre Corporation - Common Stock,NASDAQ Global Select MarketSM
SABS,\"SAB Biotherapeutics, Inc. - Common Stock\",NASDAQ Capital Market
SABSW,\"SAB Biotherapeutics, Inc. - Warrant\",NASDAQ Capital Market
SAFT,\"Safety Insurance Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SAGE,\"Sage Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
SAI,SAI.TECH Global Corporation  - Class A Ordinary Shares,NASDAQ Capital Market
SAIA,\"Saia, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SAIC,Science Applications International Corporation - Common Stock,NASDAQ Global Select MarketSM
SAITW,SAI.TECH Global Corporation  - Warrant,NASDAQ Capital Market
SAMG,Silvercrest Asset Management Group Inc. - Common Stock,NASDAQ Global MarketSM
SANA,\"Sana Biotechnology, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SANG,Sangoma Technologies Corporation - Common Shares,NASDAQ Global Select MarketSM
SANM,Sanmina Corporation - Common Stock,NASDAQ Global Select MarketSM
SANW,S&W Seed Company - Common Stock,NASDAQ Capital Market
SARK,Tradr Short Innovation Daily ETF,NASDAQ Global MarketSM
SASR,\"Sandy Spring Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SATL,Satellogic Inc. - Class A Ordinary Shares,NASDAQ Capital Market
SATLW,Satellogic Inc. - Warrant,NASDAQ Capital Market
SATS,EchoStar Corporation - Common stock,NASDAQ Global Select MarketSM
SAVA,\"Cassava Sciences, Inc. - Common Stock\",NASDAQ Capital Market
SBAC,SBA Communications Corporation - Class A Common Stock,NASDAQ Global Select MarketSM
SBCF,Seacoast Banking Corporation of Florida - Common Stock,NASDAQ Global Select MarketSM
SBET,\"SharpLink Gaming, Inc. - Common Stock\",NASDAQ Capital Market
SBFG,\"SB Financial Group, Inc. - Common Stock\",NASDAQ Capital Market
SBFM,Sunshine Biopharma Inc. - Common stock,NASDAQ Capital Market
SBFMW,Sunshine Biopharma Inc. - Warrant,NASDAQ Capital Market
SBGI,\"Sinclair, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
SBLK,Star Bulk Carriers Corp. - Common Shares,NASDAQ Global Select MarketSM
SBRA,\"Sabra Health Care REIT, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SBSI,\"Southside Bancshares, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SBT,\"Sterling Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
SBUX,Starbucks Corporation - Common Stock,NASDAQ Global Select MarketSM
SCHL,Scholastic Corporation - Common Stock,NASDAQ Global Select MarketSM
SCKT,\"Socket Mobile, Inc. - Common Stock\",NASDAQ Capital Market
SCLX,Scilex Holding Company - Common Stock,NASDAQ Capital Market
SCLXW,Scilex Holding Company - Warrant,NASDAQ Capital Market
SCNI,Scinai Immunotherapeutics Ltd. - American Depositary Shares,NASDAQ Capital Market
SCOR,\"comScore, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SCPH,scPharmaceuticals Inc. - Common Stock,NASDAQ Global Select MarketSM
SCSC,\"ScanSource, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SCVL,\"Shoe Carnival, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SCWO,374Water Inc. - common stock,NASDAQ Capital Market
SCWX,SecureWorks Corp. - Class A Common Stock,NASDAQ Global Select MarketSM
SCYX,\"SCYNEXIS, Inc. - Common Stock\",NASDAQ Global MarketSM
SCZ,iShares MSCI EAFE Small-Cap ETF,NASDAQ Global MarketSM
SDA,SunCar Technology Group Inc. - Ordinary Shares,NASDAQ Capital Market
SDAWW,SunCar Technology Group Inc. - Warrant,NASDAQ Capital Market
SDG,iShares MSCI Global Sustainable Development Goals ETF,NASDAQ Global MarketSM
SDGR,\"Schrodinger, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SDIG,\"Stronghold Digital Mining, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
SDOT,Sadot Group Inc. - Common Stock,NASDAQ Capital Market
SDSI,American Century Short Duration Strategic Income ETF,NASDAQ Global MarketSM
SDVY,First Trust SMID Cap Rising Dividend Achievers ETF,NASDAQ Global MarketSM
SEAT,Vivid Seats Inc. - Class A common stock,NASDAQ Global Select MarketSM
SEATW,Vivid Seats Inc. - Warrant,NASDAQ Global Select MarketSM
SEDG,\"SolarEdge Technologies, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SEED,Origin Agritech Limited - Ordinary Shares,NASDAQ Capital Market
SEEL,\"Seelos Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
SEER,\"Seer, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
SEIC,SEI Investments Company - Common Stock,NASDAQ Global Select MarketSM
SELF,\"Global Self Storage, Inc. - Common Stock\",NASDAQ Capital Market
SELX,Semilux International Ltd. - Ordinary Shares,NASDAQ Capital Market
SENEA,Seneca Foods Corp. - Class A Common Stock,NASDAQ Global Select MarketSM
SENEB,Seneca Foods Corp. - Class B Common Stock,NASDAQ Global Select MarketSM
SEPA,SEP Acquisition Corp - Class A Common Stock,NASDAQ Capital Market
SEPAU,SEP Acquisition Corp - Unit,NASDAQ Capital Market
SEPAW,SEP Acquisition Corp - Warrants,NASDAQ Capital Market
SERA,\"Sera Prognostics, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
SERV,Serve Robotics Inc. - Common Stock,NASDAQ Capital Market
SETM,Sprott Energy Transition Materials ETF,NASDAQ Global MarketSM
SEVN,Seven Hills Realty Trust  - Common Stock,NASDAQ Capital Market
SEZL,Sezzle Inc. - Common Stock,NASDAQ Capital Market
SFBC,\"Sound Financial Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
SFIX,\"Stitch Fix, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
SFLO,VictoryShares Small Cap Free Cash Flow ETF,NASDAQ Global MarketSM
SFM,\"Sprouts Farmers Market, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SFNC,Simmons First National Corporation - Common Stock,NASDAQ Global Select MarketSM
SFST,\"Southern First Bancshares, Inc. - Common Stock\",NASDAQ Global MarketSM
SFWL,Shengfeng Development Limited - Class A Ordinary Shares,NASDAQ Capital Market
SGA,\"Saga Communications, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
SGBX,Safe & Green Holdings Corp. - Common Stock,NASDAQ Capital Market
SGC,\"Superior Group of Companies, Inc. - Common Stock\",NASDAQ Global MarketSM
SGD,Safe and Green Development Corporation - Common Stock,NASDAQ Capital Market
SGH,\"SMART Global Holdings, Inc. - Ordinary Shares\",NASDAQ Global Select MarketSM
SGHT,\"Sight Sciences, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SGLY,Singularity Future Technology Ltd. - Common Stock,NASDAQ Capital Market
SGMA,\"SigmaTron International, Inc. - Common Stock\",NASDAQ Capital Market
SGML,Sigma Lithium Corporation - common shares,NASDAQ Capital Market
SGMO,\"Sangamo Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SGMT,Sagimet Biosciences Inc. - Series A Common Stock,NASDAQ Global MarketSM
SGRP,\"SPAR Group, Inc. - Common Stock\",NASDAQ Capital Market
SGRY,\"Surgery Partners, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SHBI,\"Shore Bancshares, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SHC,Sotera Health Company - Common Stock,NASDAQ Global Select MarketSM
SHCR,\"Sharecare, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
SHCRW,\"Sharecare, Inc. - Warrant\",NASDAQ Global Select MarketSM
SHEN,Shenandoah Telecommunications Co - Common Stock,NASDAQ Global Select MarketSM
SHFS,\"SHF Holdings, Inc. - Class A Common Stock\",NASDAQ Capital Market
SHFSW,\"SHF Holdings, Inc. - Warrants\",NASDAQ Capital Market
SHIM,Shimmick Corporation - Common Stock,NASDAQ Global MarketSM
SHIP,Seanergy Maritime Holdings Corp. - Common Stock,NASDAQ Capital Market
SHLS,\"Shoals Technologies Group, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
SHLT,SHL Telemedicine Ltd - American Depositary Shares,NASDAQ Capital Market
SHMD,SCHMID Group N.V. - Class A Ordinary Shares,NASDAQ Global MarketSM
SHMDW,SCHMID Group N.V. - Warrants,NASDAQ Capital Market
SHOO,\"Steven Madden, Ltd. - Common Stock\",NASDAQ Global Select MarketSM
SHOT,\"Safety Shot, Inc. - Common Stock\",NASDAQ Capital Market
SHOTW,\"Safety Shot, Inc. - Warrant\",NASDAQ Capital Market
SHPH,\"Shuttle Pharmaceuticals Holdings, Inc. - common stock\",NASDAQ Capital Market
SHPW,\"Shapeways Holdings, Inc. - Common Stock\",NASDAQ Global MarketSM
SHPWW,\"Shapeways Holdings, Inc. - Warrants\",NASDAQ Global MarketSM
SHRY,First Trust Bloomberg Shareholder Yield ETF,NASDAQ Global MarketSM
SHV,iShares Short Treasury Bond ETF,NASDAQ Global MarketSM
SHY,iShares 1-3 Year Treasury Bond ETF,NASDAQ Global MarketSM
SHYF,\"The Shyft Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SIBN,\"SI-BONE, Inc. - Common Stock\",NASDAQ Global MarketSM
SIDU,\"Sidus Space, Inc. - Class A Common Stock\",NASDAQ Capital Market
SIEB,Siebert Financial Corp. - Common Stock,NASDAQ Capital Market
SIFY,\"Sify Technologies Limited - American Depository Shares, each represented by one Equity Share\",NASDAQ Capital Market
SIGA,SIGA Technologies Inc. - Common Stock,NASDAQ Global MarketSM
SIGI,\"Selective Insurance Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SIGIP,\"Selective Insurance Group, Inc. - Depositary Shares, each representing a 1/1,000th interest in a share of 4.60% Non-Cumulative Preferred Stock, Series B\",NASDAQ Global Select MarketSM
SILC,Silicom Ltd - Ordinary Shares,NASDAQ Global Select MarketSM
SILK,\"Silk Road Medical, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SILO,\"Silo Pharma, Inc. - Common Stock\",NASDAQ Capital Market
SIMO,\"Silicon Motion Technology Corporation - American Depositary Shares, each representing four ordinary shares\",NASDAQ Global Select MarketSM
SINT,\"SiNtx Technologies, Inc. - Common Stock\",NASDAQ Capital Market
SIRI,Sirius XM Holdings Inc. - Common Stock,NASDAQ Global Select MarketSM
SISI,\"Shineco, Inc. - Common Stock\",NASDAQ Capital Market
SITM,SiTime Corporation - Common Stock,NASDAQ Global MarketSM
SJ,Scienjoy Holding Corporation - Class A Ordinary Shares,NASDAQ Capital Market
SKGR,SK Growth Opportunities Corporation - Class A Common Stock,NASDAQ Global MarketSM
SKGRU,SK Growth Opportunities Corporation - Unit,NASDAQ Global MarketSM
SKGRW,SK Growth Opportunities Corporation - Warrant,NASDAQ Global MarketSM
SKIN,The Beauty Health Company - Class A Common Stock,NASDAQ Capital Market
SKOR,FlexShares Credit-Scored US Corporate Bond Index Fund,NASDAQ Global MarketSM
SKRE,Tuttle Capital Daily 2X Inverse Regional Banks ETF,NASDAQ Global MarketSM
SKWD,\"Skyward Specialty Insurance Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SKYE,\"Skye Bioscience, Inc. - Common Stock\",NASDAQ Global MarketSM
SKYT,\"SkyWater Technology, Inc. - Common Stock\",NASDAQ Capital Market
SKYU,ProShares Ultra Cloud Computing,NASDAQ Global MarketSM
SKYW,\"SkyWest, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SKYX,SKYX Platforms Corp. - Common Stock,NASDAQ Capital Market
SKYY,First Trust Cloud Computing ETF,NASDAQ Global MarketSM
SLAB,\"Silicon Laboratories, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SLAM,Slam Corp. - Class A Ordinary Share,NASDAQ Capital Market
SLAMU,Slam Corp. - Unit,NASDAQ Capital Market
SLAMW,Slam Corp. - warrant,NASDAQ Capital Market
SLDB,Solid Biosciences Inc. - Common Stock,NASDAQ Global Select MarketSM
SLDP,\"Solid Power, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
SLDPW,\"Solid Power, Inc. - Warrant\",NASDAQ Global Select MarketSM
SLE,\"Super League Enterprise, Inc. - Common Stock\",NASDAQ Capital Market
SLGL,Sol-Gel Technologies Ltd. - Common Stock,NASDAQ Global MarketSM
SLM,SLM Corporation - Common Stock,NASDAQ Global Select MarketSM
SLMBP,\"SLM Corporation - Floating Rate Non-Cumulative Preferred Stock, Series B\",NASDAQ Global Select MarketSM
SLN,Silence Therapeutics Plc - American Depository Share,NASDAQ Global MarketSM
SLNA,Selina Hospitality PLC - Ordinary Shares,NASDAQ Capital Market
SLNAW,Selina Hospitality PLC - Warrant,NASDAQ Capital Market
SLNG,\"Stabilis Solutions, Inc. - Common Stock\",NASDAQ Capital Market
SLNH,\"Soluna Holdings, Inc. - Common Stock\",NASDAQ Capital Market
SLNHP,\"Soluna Holdings, Inc. - 9.0% Series A Cumulative Perpetual Preferred Stock\",NASDAQ Capital Market
SLNO,\"Soleno Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
SLP,\"Simulations Plus, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SLQD,iShares 0-5 Year Investment Grade Corporate Bond ETF,NASDAQ Global MarketSM
SLRC,SLR Investment Corp. - Closed End Fund,NASDAQ Global Select MarketSM
SLRN,\"ACELYRIN, INC. - Common Stock\",NASDAQ Global Select MarketSM
SLRX,\"Salarius Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
SLS,\"SELLAS Life Sciences Group, Inc.  - Common Stock\",NASDAQ Capital Market
SLVO,\"Credit Suisse X-Links Silver Shares Covered Call ETNs due April 21, 2033\",NASDAQ Global MarketSM
SMBC,\"Southern Missouri Bancorp, Inc. - Common Stock\",NASDAQ Global MarketSM
SMCF,Themes US Small Cap Cash Flow Champions ETF,NASDAQ Global MarketSM
SMCI,\"Super Micro Computer, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SMCO,Hilton Small-MidCap Opportunity ETF,NASDAQ Global MarketSM
SMCP,AlphaMark Actively Managed Small Cap ETF,NASDAQ Global MarketSM
SMFL,\"Smart for Life, Inc. - Common Stock\",NASDAQ Capital Market
SMH,VanEck Semiconductor ETF,NASDAQ Global MarketSM
SMID,Smith-Midland Corporation - Common Stock,NASDAQ Capital Market
SMLR,\"Semler Scientific, Inc. - Common Stock\",NASDAQ Capital Market
SMMT,Summit Therapeutics Inc.  - Common Stock,NASDAQ Global MarketSM
SMPL,The Simply Good Foods Company - Common Stock,NASDAQ Capital Market
SMRI,Bushido Capital US Equity ETF,NASDAQ Global MarketSM
SMSI,\"Smith Micro Software, Inc. - Common Stock\",NASDAQ Capital Market
SMTC,Semtech Corporation - Common Stock,NASDAQ Global Select MarketSM
SMTI,Sanara MedTech Inc. - Common Stock,NASDAQ Capital Market
SMX,SMX (Security Matters) Public Limited Company - Class A Ordinary Shares,NASDAQ Capital Market
SMXT,Solarmax Technology Inc. - Common Stock,NASDAQ Global MarketSM
SMXWW,SMX (Security Matters) Public Limited Company - Warrant,NASDAQ Capital Market
SNAL,\"Snail, Inc. - Class A Common Stock\",NASDAQ Capital Market
SNAX,\"Stryve Foods, Inc. - Class A Common Stock\",NASDAQ Capital Market
SNAXW,\"Stryve Foods, Inc. - Warrant\",NASDAQ Capital Market
SNBR,Sleep Number Corporation - Common Stock,NASDAQ Global Select MarketSM
SNCR,\"Synchronoss Technologies, Inc. - Common Stock\",NASDAQ Capital Market
SNCRL,\"Synchronoss Technologies, Inc. - 8.375% Senior Notes due 2026\",NASDAQ Global MarketSM
SNCY,\"Sun Country Airlines Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SND,\"Smart Sand, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SNDL,SNDL Inc. - Common Shares,NASDAQ Capital Market
SNDX,\"Syndax Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SNES,\"SenesTech, Inc. - Common Stock\",NASDAQ Capital Market
SNEX,StoneX Group Inc. - Common Stock,NASDAQ Global Select MarketSM
SNFCA,Security National Financial Corporation - Class A Common Stock,NASDAQ Global MarketSM
SNGX,\"Soligenix, Inc. - Common Stock\",NASDAQ Capital Market
SNOA,\"Sonoma Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
SNPO,Snap One Holdings Corp. - Common Stock,NASDAQ Global Select MarketSM
SNPS,\"Synopsys, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SNPX,\"Synaptogenix, Inc. - Common Stock\",NASDAQ Capital Market
SNSE,\"Sensei Biotherapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
SNSR,Global X Internet of Things ETF,NASDAQ Global MarketSM
SNT,Senstar Technologies Corporation - Common Shares,NASDAQ Global MarketSM
SNTG,Sentage Holdings Inc. - Class A Ordinary Shares,NASDAQ Capital Market
SNTI,\"Senti Biosciences, Inc.  - Common Stock\",NASDAQ Capital Market
SNY,Sanofi - American Depositary Shares,NASDAQ Global Select MarketSM
SOBR,\"SOBR Safe, Inc. - Common Stock\",NASDAQ Capital Market
SOCL,Global X Social Media ETF,NASDAQ Global MarketSM
SOFI,\"SoFi Technologies, Inc.  - Common Stock\",NASDAQ Global Select MarketSM
SOGP,Sound Group Inc. - American Depositary Shares,NASDAQ Capital Market
SOHO,Sotherly Hotels Inc. - Common Stock,NASDAQ Global MarketSM
SOHOB,Sotherly Hotels Inc. - 8.0% Series B Cumulative Redeemable Perpetual Preferred Stock,NASDAQ Global MarketSM
SOHON,Sotherly Hotels Inc. - 8.25% Series D Cumulative Redeemable Perpetual Preferred Stock,NASDAQ Global MarketSM
SOHOO,Sotherly Hotels Inc. - 7.875% Series C Cumulative Redeemable Perpetual Preferred Stock,NASDAQ Global MarketSM
SOHU,Sohu.com Limited  - American Depositary Shares,NASDAQ Global Select MarketSM
SOND,Sonder Holdings Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
SONDW,Sonder Holdings Inc. - Warrants,NASDAQ Global Select MarketSM
SONM,\"Sonim Technologies, Inc. - Common Stock\",NASDAQ Capital Market
SONN,\"Sonnet BioTherapeutics Holdings, Inc. - Common Stock\",NASDAQ Capital Market
SONO,\"Sonos, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SOPA,Society Pass Incorporated - Common Stock,NASDAQ Capital Market
SOPH,SOPHiA GENETICS SA - Ordinary Shares,NASDAQ Global Select MarketSM
SOTK,Sono-Tek Corporation - Common Stock,NASDAQ Capital Market
SOUN,\"SoundHound AI, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
SOUNW,\"SoundHound AI, Inc. - Warrant\",NASDAQ Global MarketSM
SOWG,Sow Good Inc. - Common Stock,NASDAQ Capital Market
SOXQ,Invesco PHLX Semiconductor ETF,NASDAQ Global MarketSM
SOXX,iShares Semiconductor ETF,NASDAQ Global MarketSM
SPAM,Themes Cybersecurity ETF,NASDAQ Global MarketSM
SPAQ,Horizon Kinetics SPAC Active ETF,NASDAQ Global MarketSM
SPBC,Simplify U.S. Equity PLUS GBTC ETF,NASDAQ Global MarketSM
SPC,CrossingBridge Pre-Merger SPAC ETF,NASDAQ Global MarketSM
SPCB,\"SuperCom, Ltd. - Ordinary Shares\",NASDAQ Capital Market
SPCX,AXS SPAC and New Issue ETF,NASDAQ Global MarketSM
SPEC,\"Spectaire Holdings, Inc. - Common Stock\",NASDAQ Global MarketSM
SPECW,\"Spectaire Holdings, Inc. - Warrants\",NASDAQ Global MarketSM
SPFI,\"South Plains Financial, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SPGC,\"Sacks Parente Golf, Inc. - Common Stock\",NASDAQ Capital Market
SPI,\"SPI Energy Co., Ltd. - Ordinary Shares\",NASDAQ Capital Market
SPKL,Spark I Acquisition Corp. - Class A Ordinary Share,NASDAQ Global MarketSM
SPKLU,Spark I Acquisition Corp. - Unit,NASDAQ Global MarketSM
SPKLW,Spark I Acquisition Corp. - Warrant,NASDAQ Global MarketSM
SPNS,Sapiens International Corporation N.V. - Common Shares,NASDAQ Global Select MarketSM
SPOK,\"Spok Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SPPL,SIMPPLE LTD. - Ordinary Shares,NASDAQ Capital Market
SPRB,\"Spruce Biosciences, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SPRC,SciSparc Ltd. - Ordinary Shares,NASDAQ Capital Market
SPRO,\"Spero Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SPRX,Spear Alpha ETF,NASDAQ Global MarketSM
SPRY,\"ARS Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global MarketSM
SPSC,\"SPS Commerce, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SPT,\"Sprout Social, Inc - Class A Common Stock\",NASDAQ Capital Market
SPTN,SpartanNash Company - Common Stock,NASDAQ Global Select MarketSM
SPWH,\"Sportsman's Warehouse Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SPWR,SunPower Corporation - Common Stock,NASDAQ Global Select MarketSM
SQFT,\"Presidio Property Trust, Inc. - Class A Common Stock\",NASDAQ Capital Market
SQFTP,\"Presidio Property Trust, Inc. - 9.375% Series D Cumulative Redeemable Perpetual Preferred Stock, $0.01 par value per share\",NASDAQ Capital Market
SQFTW,\"Presidio Property Trust, Inc. - Series A Common Stock Purchase Warrants\",NASDAQ Capital Market
SQLV,Royce Quant Small-Cap Quality Value ETF,NASDAQ Global MarketSM
SQQQ,ProShares UltraPro Short QQQ,NASDAQ Global MarketSM
SRAD,Sportradar Group AG - Class A Ordinary Shares,NASDAQ Global Select MarketSM
SRBK,\"SR Bancorp, Inc. - Common stock\",NASDAQ Capital Market
SRCE,1st Source Corporation - Common Stock,NASDAQ Global Select MarketSM
SRCL,\"Stericycle, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SRDX,\"Surmodics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SRET,Global X SuperDividend REIT ETF,NASDAQ Global MarketSM
SRM,\"SRM Entertainment, Inc. - Common Stock\",NASDAQ Capital Market
SRPT,\"Sarepta Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SRRK,Scholar Rock Holding Corporation - Common Stock,NASDAQ Global Select MarketSM
SRTS,\"Sensus Healthcare, Inc. - Common Stock\",NASDAQ Capital Market
SRZN,\"Surrozen, Inc. - Common Stock\",NASDAQ Capital Market
SRZNW,\"Surrozen, Inc. - Warrant\",NASDAQ Capital Market
SSBI,Summit State Bank - Common Stock,NASDAQ Global MarketSM
SSBK,\"Southern States Bancshares, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SSIC,Silver Spike Investment Corp. - Common Stock,NASDAQ Global MarketSM
SSKN,\"Strata Skin Sciences, Inc. - Common Stock\",NASDAQ Capital Market
SSNC,\"SS&C Technologies Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SSNT,\"SilverSun Technologies, Inc. - Common Stock\",NASDAQ Capital Market
SSP,E.W. Scripps Company (The) - Class A Common Stock,NASDAQ Global Select MarketSM
SSRM,SSR Mining Inc. - Common Stock,NASDAQ Global Select MarketSM
SSSS,SuRo Capital Corp. - Closed End Fund,NASDAQ Global Select MarketSM
SSSSL,SuRo Capital Corp. - 6.00% Notes due 2026,NASDAQ Global MarketSM
SSTI,\"SoundThinking, Inc. - Common Stock\",NASDAQ Capital Market
SSYS,\"Stratasys, Ltd. - Common Stock\",NASDAQ Global Select MarketSM
STAA,STAAR Surgical Company - Common Stock,NASDAQ Global MarketSM
STAF,\"Staffing 360 Solutions, Inc. - Common Stock\",NASDAQ Capital Market
STBA,\"S&T Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
STBX,Starbox Group Holdings Ltd. - Ordinary Shares,NASDAQ Capital Market
STCN,\"Steel Connect, Inc. - Common Stock\",NASDAQ Capital Market
STEP,StepStone Group Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
STER,Sterling Check Corp. - Common Stock,NASDAQ Global Select MarketSM
STGW,Stagwell Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
STHO,Star Holdings - Shares of Beneficial Interest,NASDAQ Global MarketSM
STI,\"Solidion Technology, Inc. - Common Stock\",NASDAQ Global MarketSM
STIM,\"Neuronetics, Inc. - Common Stock\",NASDAQ Global MarketSM
STKH,Steakholder Foods Ltd. - American Depositary Shares,NASDAQ Capital Market
STKL,\"SunOpta, Inc. - Common Stock\",NASDAQ Global Select MarketSM
STKS,\"The ONE Group Hospitality, Inc. - Common Stock\",NASDAQ Capital Market
STLD,\"Steel Dynamics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
STNE,StoneCo Ltd. - Class A Common Share,NASDAQ Global Select MarketSM
STOK,\"Stoke Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
STRA,\"Strategic Education, Inc. - Common Stock\",NASDAQ Global Select MarketSM
STRL,\"Sterling Infrastructure, Inc. - Common Stock\",NASDAQ Global Select MarketSM
STRM,\"Streamline Health Solutions, Inc. - Common Stock\",NASDAQ Capital Market
STRO,\"Sutro Biopharma, Inc. - Common Stock\",NASDAQ Global MarketSM
STRR,\"Star Equity Holdings, Inc. - Common Stock\",NASDAQ Global MarketSM
STRRP,\"Star Equity Holdings, Inc. - Series A Cumulative Perpetual Preferred Stock\",NASDAQ Global MarketSM
STRS,Stratus Properties Inc. - Common Stock,NASDAQ Global Select MarketSM
STRT,STRATTEC SECURITY CORPORATION - Common Stock,NASDAQ Global MarketSM
STSS,Sharps Technology Inc. - Common Stock,NASDAQ Capital Market
STSSW,Sharps Technology Inc. - Warrant,NASDAQ Capital Market
STTK,\"Shattuck Labs, Inc. - Common Stock\",NASDAQ Global Select MarketSM
STX,Seagate Technology Holdings PLC - Ordinary Shares (Ireland),NASDAQ Global Select MarketSM
SUGP,SU Group Holdings Limited - Ordinary Shares,NASDAQ Capital Market
SUPN,\"Supernus Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global MarketSM
SURG,\"SurgePays, Inc. - Common Stock\",NASDAQ Capital Market
SURGW,\"SurgePays, Inc. - Warrant\",NASDAQ Capital Market
SUSB,iShares ESG Aware 1-5 Year USD Corporate Bond ETF,NASDAQ Global MarketSM
SUSC,iShares ESG Aware USD Corporate Bond ETF,NASDAQ Global MarketSM
SUSL,iShares ESG MSCI USA Leaders ETF,NASDAQ Global MarketSM
SUUN,SolarBank Corporation - Common Stock,NASDAQ Global MarketSM
SVA,\"Sinovac Biotech, Ltd. - Ordinary Shares (Antigua/Barbudo)\",NASDAQ Global Select MarketSM
SVC,Service Properties Trust - Shares of Beneficial Interest,NASDAQ Global Select MarketSM
SVCO,\"Silvaco Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SVII,Spring Valley Acquisition Corp. II - Class A Ordinary Shares,NASDAQ Global MarketSM
SVIIR,Spring Valley Acquisition Corp. II - Rights,NASDAQ Global MarketSM
SVIIU,Spring Valley Acquisition Corp. II - Units,NASDAQ Global MarketSM
SVIIW,Spring Valley Acquisition Corp. II - Warrant,NASDAQ Global MarketSM
SVMH,SRIVARU Holding Limited - Ordinary Shares,NASDAQ Global MarketSM
SVMHW,SRIVARU Holding Limited - Warrant,NASDAQ Capital Market
SVRA,\"Savara, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SVRE,SaverOne 2014 Ltd. - American Depositary Shares,NASDAQ Capital Market
SVREW,SaverOne 2014 Ltd. - Warrant,NASDAQ Capital Market
SWAG,\"Stran & Company, Inc. - Common Stock\",NASDAQ Capital Market
SWAGW,\"Stran & Company, Inc. - Warrant\",NASDAQ Capital Market
SWAV,\"Shockwave Medical, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SWBI,\"Smith & Wesson Brands, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SWIM,\"Latham Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SWIN,Solowin Holdings - Ordinary Share,NASDAQ Capital Market
SWKH,SWK Holdings Corporation - Common Stock,NASDAQ Global MarketSM
SWKHL,SWK Holdings Corporation - 9.00% Senior Notes due 2027,NASDAQ Global MarketSM
SWKS,\"Skyworks Solutions, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SWSS,Clean Energy Special Situations Corp.  - Common stock,NASDAQ Capital Market
SWSSU,Clean Energy Special Situations Corp.  - Unit,NASDAQ Capital Market
SWSSW,Clean Energy Special Situations Corp.  - Warrant,NASDAQ Capital Market
SWTX,\"SpringWorks Therapeutics, Inc. - common stock\",NASDAQ Global Select MarketSM
SWVL,Swvl Holdings Corp - Ordinary Shares,NASDAQ Capital Market
SWVLW,Swvl Holdings Corp - Warrant,NASDAQ Capital Market
SXTC,\"China SXT Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
SXTP,\"60 Degrees Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
SXTPW,\"60 Degrees Pharmaceuticals, Inc. - Warrant\",NASDAQ Capital Market
SY,So-Young International Inc. - American Depository Shares,NASDAQ Global MarketSM
SYBT,\"Stock Yards Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SYBX,\"Synlogic, Inc. - Common Stock\",NASDAQ Capital Market
SYM,Symbotic Inc. - Class A Common Stock,NASDAQ Global MarketSM
SYNA,Synaptics Incorporated - Common Stock,NASDAQ Global Select MarketSM
SYPR,\"Sypris Solutions, Inc. - Common Stock\",NASDAQ Global MarketSM
SYRA,Syra Health Corp. - Class A Common Stock,NASDAQ Capital Market
SYRE,\"Spyre Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SYRS,\"Syros Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
SYT,\"SYLA Technologies Co., Ltd. - American Depositary Shares\",NASDAQ Capital Market
SYTA,\"Siyata Mobile, Inc. - Common Shares\",NASDAQ Capital Market
SYTAW,\"Siyata Mobile, Inc. - Warrant\",NASDAQ Capital Market
TACT,TransAct Technologies Incorporated - Common Stock,NASDAQ Global MarketSM
TAIT,Taitron Components Incorporated - Class A Common Stock,NASDAQ Capital Market
TALK,\"Talkspace, Inc. - Common Stock\",NASDAQ Capital Market
TALKW,\"Talkspace, Inc. - Warrant\",NASDAQ Capital Market
TANH,Tantech Holdings Ltd. - Common Shares,NASDAQ Capital Market
TAOP,Taoping Inc. - Ordinary Shares,NASDAQ Capital Market
TARA,\"Protara Therapeutics, Inc.  - Common Stock\",NASDAQ Global MarketSM
TARK,Tradr 2X Long Innovation ETF,NASDAQ Global MarketSM
TARS,\"Tarsus Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TASK,\"TaskUs, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
TATT,TAT Technologies Ltd. - Ordinary Shares,NASDAQ Global MarketSM
TAYD,\"Taylor Devices, Inc. - Common Stock\",NASDAQ Capital Market
TBBK,\"The Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TBIL,US Treasury 3 Month Bill ETF,NASDAQ Global MarketSM
TBIO,\"Telesis Bio, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TBLA,Taboola.com Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
TBLAW,Taboola.com Ltd. - Warrant,NASDAQ Global Select MarketSM
TBLD,Thornburg Income Builder Opportunities Trust - Closed End Fund,NASDAQ Global Select MarketSM
TBLT,\"ToughBuilt Industries, Inc. - Common Stock\",NASDAQ Capital Market
TBMC,Trailblazer Merger Corporation I - Class A Common Stock,NASDAQ Global MarketSM
TBMCR,Trailblazer Merger Corporation I - Rights,NASDAQ Global MarketSM
TBNK,Territorial Bancorp Inc. - Common Stock,NASDAQ Global Select MarketSM
TBPH,\"Theravance Biopharma, Inc. - Ordinary Shares\",NASDAQ Global MarketSM
TBRG,\"TruBridge, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TC,TuanChe Limited - American Depositary Shares,NASDAQ Capital Market
TCBC,\"TC Bancshares, Inc. - Common Stock\",NASDAQ Capital Market
TCBI,\"Texas Capital Bancshares, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TCBIO,\"Texas Capital Bancshares, Inc. - Depositary Shares 5.75% Fixed Rate Non-Cumulative Perpetual Preferred Stock Series B\",NASDAQ Global Select MarketSM
TCBK,TriCo Bancshares - Common Stock,NASDAQ Global Select MarketSM
TCBP,TC BioPharm (Holdings) plc - American Depositary Shares,NASDAQ Capital Market
TCBPW,TC BioPharm (Holdings) plc - Warrants,NASDAQ Capital Market
TCBS,\"Texas Community Bancshares, Inc. - Common Stock\",NASDAQ Capital Market
TCBX,\"Third Coast Bancshares, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TCHI,iShares MSCI China Multisector Tech ETF,NASDAQ Global MarketSM
TCJH,Top KingWin Ltd - Class A Ordinary Shares,NASDAQ Capital Market
TCMD,\"Tactile Systems Technology, Inc. - Common Stock\",NASDAQ Global MarketSM
TCOM,Trip.com Group Limited - American Depositary Shares,NASDAQ Global Select MarketSM
TCON,\"TRACON Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
TCPC,BlackRock TCP Capital Corp. - Closed End Fund,NASDAQ Global Select MarketSM
TCRT,\"Alaunos Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
TCRX,\"TScan Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
TCTM,TCTM Kids IT Education Inc. - American Depositary Shares,NASDAQ Capital Market
TCX,Tucows Inc. - Common Stock,NASDAQ Capital Market
TDI,Touchstone Dynamic International ETF,NASDAQ Global MarketSM
TDIV,First Trust NASDAQ Technology Dividend Index Fund,NASDAQ Global MarketSM
TDSB,Cabana Target Beta ETF,NASDAQ Global MarketSM
TDSC,Cabana Target Drawdown 10 ETF,NASDAQ Global MarketSM
TDUP,ThredUp Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
TEAM,Atlassian Corporation  - Class A Common Stock,NASDAQ Global Select MarketSM
TECH,Bio-Techne Corp - Common Stock,NASDAQ Global Select MarketSM
TECTP,\"Tectonic Financial, Inc. - 9.00% Fixed-to-Floating Rate Series B Non-Cumulative Perpetual Preferred Stock\",NASDAQ Capital Market
TELA,\"TELA Bio, Inc. - Common stock\",NASDAQ Global MarketSM
TELO,\"Telomir Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
TENB,\"Tenable Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TENK,TenX Keane Acquisition - Ordinary Share,NASDAQ Global MarketSM
TENKR,TenX Keane Acquisition - Right,NASDAQ Global MarketSM
TENKU,TenX Keane Acquisition - Unit,NASDAQ Global MarketSM
TENX,\"Tenax Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
TER,\"Teradyne, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TERN,\"Terns Pharmaceuticals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TETE,Technology & Telecommunication Acquisition Corporation - Class A Ordinary Shares,NASDAQ Global MarketSM
TETEU,Technology & Telecommunication Acquisition Corporation - Unit,NASDAQ Global MarketSM
TETEW,Technology & Telecommunication Acquisition Corporation - Warrant,NASDAQ Global MarketSM
TFFP,\"TFF Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
TFIN,\"Triumph Financial, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TFINP,\"Triumph Financial, Inc. - Depositary Shares, Each Representing a 1/40th Interest in a Share of Series C Fixed-Rate Non-Cumulative Perpetual Preferred Stock\",NASDAQ Global Select MarketSM
TFSL,TFS Financial Corporation - Common Stock,NASDAQ Global Select MarketSM
TGAA,Target Global Acquisition I Corp. - Class A Ordinary Share,NASDAQ Global MarketSM
TGAAU,Target Global Acquisition I Corp. - Unit,NASDAQ Global MarketSM
TGAAW,Target Global Acquisition I Corp. - Warrant,NASDAQ Global MarketSM
TGAN,\"Transphorm, Inc. - Common Stock\",NASDAQ Capital Market
TGL,Treasure Global Inc. - Common Stock,NASDAQ Capital Market
TGTX,\"TG Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
TH,Target Hospitality Corp. - Common Stock,NASDAQ Capital Market
THAR,\"Tharimmune, Inc. - Common Stock\",NASDAQ Capital Market
THCH,TH International Limited - Ordinary shares,NASDAQ Capital Market
THCP,\"Thunder Bridge Capital Partners IV, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
THCPU,\"Thunder Bridge Capital Partners IV, Inc. - Unit\",NASDAQ Global MarketSM
THCPW,\"Thunder Bridge Capital Partners IV, Inc. - Warrant\",NASDAQ Global MarketSM
THFF,First Financial Corporation Indiana - Common Stock,NASDAQ Global Select MarketSM
THMO,\"ThermoGenesis Holdings, Inc. - Common Stock\",NASDAQ Capital Market
THRD,\"Third Harmonic Bio, Inc. - Common Stock\",NASDAQ Global MarketSM
THRM,Gentherm Inc - Common Stock,NASDAQ Global Select MarketSM
THRY,\"Thryv Holdings, Inc. - Common Stock\",NASDAQ Capital Market
THTX,Theratechnologies Inc. - Common Shares,NASDAQ Capital Market
TIGO,Millicom International Cellular S.A. - Common Stock,NASDAQ Global Select MarketSM
TIGR,UP Fintech Holding Limited - American Depositary Shares representing fifteen Class A Ordinary Shares,NASDAQ Global Select MarketSM
TIL,\"Instil Bio, Inc. - Common Stock\",NASDAQ Capital Market
TILE,\"Interface, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TIPT,Tiptree Inc. - Common Stock,NASDAQ Capital Market
TIRX,TIAN RUIXIANG Holdings Ltd - Class A Ordinary Shares,NASDAQ Capital Market
TITN,Titan Machinery Inc. - Common Stock,NASDAQ Global Select MarketSM
TIVC,\"Tivic Health Systems, Inc. - Common stock\",NASDAQ Capital Market
TKLF,\"Yoshitsu Co., Ltd - American Depositary Shares\",NASDAQ Capital Market
TKNO,\"Alpha Teknova, Inc. - Common Stock\",NASDAQ Global MarketSM
TLF,\"Tandy Leather Factory, Inc. - common stock\",NASDAQ Capital Market
TLGY,TLGY Acquisition Corporation - Class A Ordinary Share,NASDAQ Global MarketSM
TLGYU,TLGY Acquisition Corporation - Unit,NASDAQ Global MarketSM
TLGYW,TLGY Acquisition Corporation - Warrant,NASDAQ Global MarketSM
TLIS,Talis Biomedical Corporation - common stock,NASDAQ Capital Market
TLPH,\"Talphera, Inc. - Common Stock\",NASDAQ Global MarketSM
TLRY,\"Tilray Brands, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TLS,Telos Corporation - Common Stock,NASDAQ Global MarketSM
TLSA,Tiziana Life Sciences Ltd - Common Shares,NASDAQ Capital Market
TLSI,\"TriSalus Life Sciences, Inc. - Common Stock\",NASDAQ Global MarketSM
TLSIW,\"TriSalus Life Sciences, Inc. - Warrant\",NASDAQ Global MarketSM
TLT,iShares 20+ Year Treasury Bond ETF,NASDAQ Global MarketSM
TMC,TMC the metals company Inc. - Common Stock,NASDAQ Global Select MarketSM
TMCI,\"Treace Medical Concepts, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TMCWW,TMC the metals company Inc. - Warrant,NASDAQ Global Select MarketSM
TMDX,\"TransMedics Group, Inc. - Common Stock\",NASDAQ Global MarketSM
TMET,iShares Transition-Enabling Metals ETF,NASDAQ Global MarketSM
TMTC,TMT Acquisition Corp - Ordinary Shares,NASDAQ Global MarketSM
TMTCR,TMT Acquisition Corp - Rights,NASDAQ Global MarketSM
TMTCU,TMT Acquisition Corp - Unit,NASDAQ Global MarketSM
TMUS,\"T-Mobile US, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TNDM,\"Tandem Diabetes Care, Inc. - Common Stock\",NASDAQ Global MarketSM
TNGX,\"Tango Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
TNON,\"Tenon Medical, Inc. - Common Stock\",NASDAQ Capital Market
TNONW,\"Tenon Medical, Inc. - Warrant\",NASDAQ Capital Market
TNXP,Tonix Pharmaceuticals Holding Corp. - Common Stock,NASDAQ Capital Market
TNYA,\"Tenaya Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TOI,\"The Oncology Institute, Inc. - Common Stock\",NASDAQ Capital Market
TOIIW,\"The Oncology Institute, Inc. - Warrant\",NASDAQ Capital Market
TOMZ,\"TOMI Environmental Solutions, Inc. - Common Stock\",NASDAQ Capital Market
TOP,TOP Financial Group Limited - Ordinary Shares,NASDAQ Capital Market
TORO,Toro Corp. - Common stock,NASDAQ Capital Market
TOUR,Tuniu Corporation - American Depositary Shares,NASDAQ Global MarketSM
TOWN,Towne Bank - Common Stock,NASDAQ Global Select MarketSM
TPCS,TechPrecision Corporation - Common stock,NASDAQ Capital Market
TPG,TPG Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
TPGXL,TPG Inc. - 6.950% Fixed-Rate Junior Subordinated Notes due 2064,NASDAQ Global MarketSM
TPIC,\"TPI Composites, Inc. - Common Stock\",NASDAQ Global MarketSM
TPST,\"Tempest Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
TQQQ,ProShares UltraPro QQQ,NASDAQ Global MarketSM
TRAW,\"Traws Pharma, Inc. - Common Stock\",NASDAQ Capital Market
TRDA,\"Entrada Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
TREE,\"LendingTree, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TRES,Defiance Treasury Alternative Yield ETF,NASDAQ Global MarketSM
TRIB,Trinity Biotech plc - American Depositary Shares,NASDAQ Global Select MarketSM
TRIN,Trinity Capital Inc. - Common Stock,NASDAQ Global Select MarketSM
TRINL,Trinity Capital Inc. - 7.00% Notes Due 2025,NASDAQ Global MarketSM
TRINZ,Trinity Capital Inc. - 7.875% Notes due 2029,NASDAQ Global Select MarketSM
TRIP,\"TripAdvisor, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TRMB,Trimble Inc. - Common Stock,NASDAQ Global Select MarketSM
TRMD,TORM plc - Class A Common Stock,NASDAQ Global Select MarketSM
TRMK,Trustmark Corporation - Common Stock,NASDAQ Global Select MarketSM
TRML,\"Tourmaline Bio, Inc.  - Common Stock\",NASDAQ Global Select MarketSM
TRNR,Interactive Strength Inc. - Common Stock,NASDAQ Capital Market
TRNS,\"Transcat, Inc. - Common Stock\",NASDAQ Global MarketSM
TRON,Corner Growth Acquisition Corp. 2 - Class A Ordinary Share,NASDAQ Capital Market
TRONU,Corner Growth Acquisition Corp. 2 - Units,NASDAQ Capital Market
TRONW,Corner Growth Acquisition Corp. 2 - Warrants,NASDAQ Capital Market
TROO,\"TROOPS, Inc.  - Ordinary Shares\",NASDAQ Capital Market
TROW,\"T. Rowe Price Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TRS,TriMas Corporation - Common Stock,NASDAQ Global Select MarketSM
TRSG,Tungray Technologies Inc - Class A Ordinary Shares,NASDAQ Capital Market
TRST,TrustCo Bank Corp NY - Common Stock,NASDAQ Global Select MarketSM
TRUE,\"TrueCar, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TRUG,\"TruGolf Holdings, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
TRUP,\"Trupanion, Inc. - Common Stock\",NASDAQ Global MarketSM
TRVG,trivago N.V. - American Depositary Shares,NASDAQ Global Select MarketSM
TRVI,\"Trevi Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
TRVN,\"Trevena, Inc. - Common Stock\",NASDAQ Capital Market
TSAT,Telesat Corporation - Class A Common Shares and Class B Variable Voting Shares,NASDAQ Global Select MarketSM
TSBK,\"Timberland Bancorp, Inc. - Common Stock\",NASDAQ Global MarketSM
TSBX,Turnstone Biologics Corp. - Common Stock,NASDAQ Global MarketSM
TSCO,Tractor Supply Company - Common Stock,NASDAQ Global Select MarketSM
TSDD,GraniteShares 2x Short TSLA Daily ETF,NASDAQ Global MarketSM
TSEM,Tower Semiconductor Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
TSHA,\"Taysha Gene Therapies, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TSL,GraniteShares 1.25x Long TSLA Daily ETF,NASDAQ Global MarketSM
TSLA,\"Tesla, Inc.  - Common Stock\",NASDAQ Global Select MarketSM
TSLL,Direxion Daily TSLA Bull 2X Shares,NASDAQ Global MarketSM
TSLQ,Tradr TSLA Bear Daily ETF,NASDAQ Global MarketSM
TSLR,GraniteShares 2x Long TSLA Daily ETF,NASDAQ Global MarketSM
TSLS,Direxion Daily TSLA Bear 1X Shares,NASDAQ Global MarketSM
TSLT,T-REX 2X Long Tesla Daily Target ETF,NASDAQ Global MarketSM
TSLZ,T-Rex 2X Inverse Tesla Daily Target ETF,NASDAQ Global MarketSM
TSRI,\"TSR, Inc. - Common Stock\",NASDAQ Capital Market
TSVT,\"2seventy bio, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TTD,\"The Trade Desk, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
TTEC,\"TTEC Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TTEK,\"Tetra Tech, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TTGT,\"TechTarget, Inc. - Common Stock\",NASDAQ Global MarketSM
TTMI,\"TTM Technologies, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TTNP,\"Titan Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
TTOO,\"T2 Biosystems, Inc. - Common Stock\",NASDAQ Capital Market
TTSH,\"Tile Shop Holdings, Inc. - Common Stock\",NASDAQ Capital Market
TTWO,\"Take-Two Interactive Software, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TUG,STF Tactical Growth ETF,NASDAQ Global MarketSM
TUGN,STF Tactical Growth & Income ETF,NASDAQ Global MarketSM
TUR,iShares MSCI Turkey ETF,NASDAQ Global MarketSM
TURB,\"Turbo Energy, S.A. - American Depositary Shares\",NASDAQ Capital Market
TURN,180 Degree Capital Corp. - Closed End Fund,NASDAQ Global MarketSM
TUSK,\"Mammoth Energy Services, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TVGN,Tevogen Bio Holdings Inc. - Common Stock,NASDAQ Global MarketSM
TVGNW,Tevogen Bio Holdings Inc. - Warrant,NASDAQ Capital Market
TVTX,\"Travere Therapeutics, Inc. - Common Stock\",NASDAQ Global MarketSM
TW,Tradeweb Markets Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
TWG,Top Wealth Group Holding Limited - Ordinary Shares,NASDAQ Capital Market
TWIN,\"Twin Disc, Incorporated - Common Stock\",NASDAQ Global Select MarketSM
TWKS,\"Thoughtworks Holding, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TWLV,Twelve Seas Investment Company II - Class A Common Stock,NASDAQ Capital Market
TWLVU,Twelve Seas Investment Company II - Unit,NASDAQ Capital Market
TWLVW,Twelve Seas Investment Company II - Warrant,NASDAQ Capital Market
TWOU,\"2U, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TWST,Twist Bioscience Corporation - Common Stock,NASDAQ Global Select MarketSM
TXG,\"10x Genomics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TXMD,\"TherapeuticsMD, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TXN,Texas Instruments Incorporated - Common Stock,NASDAQ Global Select MarketSM
TXRH,\"Texas Roadhouse, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TXSS,Texas Capital Texas Small Cap Equity Index ETF,NASDAQ Global MarketSM
TYGO,\"Tigo Energy, Inc. - Common Stock\",NASDAQ Capital Market
TYRA,\"Tyra Biosciences, Inc. - Common Stock\",NASDAQ Global Select MarketSM
TZOO,Travelzoo - Common Stock,NASDAQ Global Select MarketSM
UAE,iShares MSCI UAE ETF,NASDAQ Global MarketSM
UAL,\"United Airlines Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
UBCP,\"United Bancorp, Inc. - Common Stock\",NASDAQ Capital Market
UBFO,United Security Bancshares - Common Stock,NASDAQ Global Select MarketSM
UBND,VictoryShares Core Plus Intermediate Bond ETF,NASDAQ Global MarketSM
UBSI,\"United Bankshares, Inc. - Common Stock\",NASDAQ Global Select MarketSM
UBX,\"Unity Biotechnology, Inc. - Common Stock\",NASDAQ Global Select MarketSM
UBXG,U-BX Technology Ltd. - Ordinary Shares,NASDAQ Capital Market
UCAR,U Power Limited - Ordinary Shares,NASDAQ Capital Market
UCBI,\"United Community Banks, Inc. - Common Stock\",NASDAQ Global Select MarketSM
UCBIO,\"United Community Banks, Inc. - Depositary Shares each representing 1/1,000th interest in a share of Series I Non-Cumulative Preferred Stock\",NASDAQ Global Select MarketSM
UCL,uCloudlink Group Inc. - American Depositary Shares,NASDAQ Global MarketSM
UCRD,VictoryShares Corporate Bond ETF,NASDAQ Global MarketSM
UCTT,\"Ultra Clean Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
UCYB,ProShares Ultra Nasdaq Cybersecurity,NASDAQ Global MarketSM
UDMY,\"Udemy, Inc. - Common Stock\",NASDAQ Global Select MarketSM
UEIC,Universal Electronics Inc. - Common Stock,NASDAQ Global Select MarketSM
UEVM,VictoryShares Emerging Markets Value Momentum ETF,NASDAQ Global MarketSM
UFCS,\"United Fire Group, Inc - Common Stock\",NASDAQ Global Select MarketSM
UFIV,US Treasury 5 Year Note ETF,NASDAQ Global MarketSM
UFO,Procure Space ETF,NASDAQ Global MarketSM
UFPI,\"UFP Industries, Inc. - Common Stock\",NASDAQ Global Select MarketSM
UFPT,\"UFP Technologies, Inc. - Common Stock\",NASDAQ Capital Market
UG,\"United-Guardian, Inc. - Common Stock\",NASDAQ Global MarketSM
UGRO,\"urban-gro, Inc. - Common Stock\",NASDAQ Capital Market
UHG,\"United Homes Group, Inc - Class A Common Stock\",NASDAQ Global MarketSM
UHGWW,\"United Homes Group, Inc - Warrant\",NASDAQ Capital Market
UITB,VictoryShares Core Intermediate Bond ETF,NASDAQ Global MarketSM
UIVM,VictoryShares International Value Momentum ETF,NASDAQ Global MarketSM
UK,Ucommune International Ltd  - Ordinary Shares,NASDAQ Capital Market
UKOMW,Ucommune International Ltd  - Warrant expiring 11/17/2025,NASDAQ Capital Market
ULBI,Ultralife Corporation - Common Stock,NASDAQ Global MarketSM
ULCC,\"Frontier Group Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ULH,\"Universal Logistics Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ULTA,\"Ulta Beauty, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ULVM,VictoryShares US Value Momentum ETF,NASDAQ Global MarketSM
ULY,Urgent.ly Inc. - Common Stock,NASDAQ Capital Market
UMBF,UMB Financial Corporation - Common Stock,NASDAQ Global Select MarketSM
UMMA,Wahed Dow Jones Islamic World ETF,NASDAQ Global MarketSM
UNB,\"Union Bankshares, Inc. - Common Stock\",NASDAQ Global MarketSM
UNCY,\"Unicycive Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
UNIT,Uniti Group Inc. - Common Stock,NASDAQ Global Select MarketSM
UNIY,WisdomTree Voya Yield Enhanced USD Universal Bond Fund,NASDAQ Global MarketSM
UNTY,\"Unity Bancorp, Inc. - Common Stock\",NASDAQ Global MarketSM
UONE,\"Urban One, Inc.  - Class A Common Stock\",NASDAQ Capital Market
UONEK,\"Urban One, Inc.  - Class D Common Stock\",NASDAQ Capital Market
UPBD,\"Upbound Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
UPC,Universe Pharmaceuticals Inc - Ordinary Shares,NASDAQ Capital Market
UPGR,Xtrackers US Green Infrastructure Select Equity ETF,NASDAQ Global MarketSM
UPLD,\"Upland Software, Inc. - Common Stock\",NASDAQ Global MarketSM
UPST,\"Upstart Holdings, Inc. - Common stock\",NASDAQ Global Select MarketSM
UPWK,Upwork Inc. - Common Stock,NASDAQ Global Select MarketSM
UPXI,\"Upexi, Inc. - Common Stock\",NASDAQ Capital Market
URBN,\"Urban Outfitters, Inc. - Common Stock\",NASDAQ Global Select MarketSM
URGN,UroGen Pharma Ltd. - Ordinary Shares,NASDAQ Global MarketSM
URNJ,Sprott Junior Uranium Miners ETF,NASDAQ Global MarketSM
UROY,Uranium Royalty Corp. - Common Stock,NASDAQ Capital Market
USAP,\"Universal Stainless & Alloy Products, Inc. - Common Stock\",NASDAQ Global Select MarketSM
USAU,U.S. Gold Corp. - Common Stock,NASDAQ Capital Market
USBF,iShares USD Systematic Bond ETF,NASDAQ Global MarketSM
USCB,\"USCB Financial Holdings, Inc.  - Class A Common Stock\",NASDAQ Global MarketSM
USCF,Themes US Cash Flow Champions ETF,NASDAQ Global MarketSM
USCL,iShares Climate Conscious & Transition MSCI USA ETF,NASDAQ Global MarketSM
USDX,SGI Enhanced Core ETF,NASDAQ Global MarketSM
USEA,United Maritime Corporation - Common Stock,NASDAQ Capital Market
USEG,U.S. Energy Corp. - Common Stock,NASDAQ Capital Market
USFI,BrandywineGLOBAL - U.S. Fixed Income ETF,NASDAQ Global MarketSM
USGO,U.S. GoldMining Inc. - Common stock,NASDAQ Capital Market
USGOW,U.S. GoldMining Inc. - Warrant,NASDAQ Capital Market
USIG,iShares Broad USD Investment Grade Corporate Bond ETF,NASDAQ Global MarketSM
USIN,WisdomTree 7-10 Year Laddered Treasury Fund,NASDAQ Global MarketSM
USIO,\"Usio, Inc. - Common Stock\",NASDAQ Global MarketSM
USLM,\"United States Lime & Minerals, Inc. - Common Stock\",NASDAQ Global Select MarketSM
USMC,Principal U.S. Mega-Cap ETF,NASDAQ Global MarketSM
USOI,Credit Suisse X-Links Crude Oil Shares Covered Call ETN,NASDAQ Global MarketSM
USOY,Defiance Oil Enhanced Options Income ETF,NASDAQ Global MarketSM
USRD,Themes US R&D Champions ETF,NASDAQ Global MarketSM
USSH,WisdomTree 1-3 Year Laddered Treasury Fund,NASDAQ Global MarketSM
USTB,VictoryShares Short-Term Bond ETF,NASDAQ Global MarketSM
USVM,VictoryShares US Small Mid Cap Value Momentum ETF,NASDAQ Global MarketSM
USVN,US Treasury 7 Year Note ETF,NASDAQ Global MarketSM
USXF,iShares ESG Advanced MSCI USA ETF,NASDAQ Global MarketSM
UTEN,US Treasury 10 Year Note ETF,NASDAQ Global MarketSM
UTHR,United Therapeutics Corporation - Common Stock,NASDAQ Global Select MarketSM
UTHY,US Treasury 30 Year Bond ETF,NASDAQ Global MarketSM
UTMD,\"Utah Medical Products, Inc. - Common Stock\",NASDAQ Global Select MarketSM
UTRE,US Treasury 3 Year Note ETF,NASDAQ Global MarketSM
UTSI,UTStarcom Holdings Corp - Ordinary Shares,NASDAQ Global Select MarketSM
UTWO,US Treasury 2 Year Note ETF,NASDAQ Global MarketSM
UTWY,US Treasury 20 Year Bond ETF,NASDAQ Global MarketSM
UVSP,Univest Financial Corporation - Common Stock,NASDAQ Global Select MarketSM
UXIN,Uxin Limited - American Depositary Shares,NASDAQ Global Select MarketSM
VABK,Virginia National Bankshares Corporation - Common Stock,NASDAQ Capital Market
VALN,Valneva SE - American Depositary Shares,NASDAQ Global Select MarketSM
VALU,\"Value Line, Inc. - Common Stock\",NASDAQ Capital Market
VANI,\"Vivani Medical, Inc.  - Common Stock\",NASDAQ Capital Market
VBFC,Village Bank and Trust Financial Corp. - Common Stock,NASDAQ Capital Market
VBIV,\"VBI Vaccines, Inc. - Ordinary Shares\",NASDAQ Capital Market
VBNK,VersaBank - Common Shares,NASDAQ Global Select MarketSM
VBTX,\"Veritex Holdings, Inc. - Common Stock\",NASDAQ Global MarketSM
VC,Visteon Corporation - Common Stock,NASDAQ Global Select MarketSM
VCEL,Vericel Corporation - Common Stock,NASDAQ Global MarketSM
VCIG,VCI Global Limited - Ordinary Share,NASDAQ Capital Market
VCIT,Vanguard Intermediate-Term Corporate Bond ETF,NASDAQ Global MarketSM
VCLT,Vanguard Long-Term Corporate Bond ETF,NASDAQ Global MarketSM
VCNX,\"Vaccinex, Inc. - Common Stock\",NASDAQ Capital Market
VCRB,Vanguard Core Bond ETF,NASDAQ Global MarketSM
VCSA,\"Vacasa, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
VCSH,Vanguard Short-Term Corporate Bond ETF,NASDAQ Global MarketSM
VCTR,\"Victory Capital Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
VCYT,\"Veracyte, Inc. - Common Stock\",NASDAQ Global MarketSM
VECO,Veeco Instruments Inc. - Common Stock,NASDAQ Global Select MarketSM
VEEE,Twin Vee PowerCats Co. - Common Stock,NASDAQ Capital Market
VEON,VEON Ltd. - American Depositary Shares,NASDAQ Capital Market
VERA,\"Vera Therapeutics, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
VERB,\"Verb Technology Company, Inc. - Common Stock\",NASDAQ Capital Market
VERI,\"Veritone, Inc. - Common Stock\",NASDAQ Global MarketSM
VERO,Venus Concept Inc.  - Common Stock,NASDAQ Capital Market
VERU,Veru Inc. - Common Stock,NASDAQ Capital Market
VERV,\"Verve Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
VERX,\"Vertex, Inc. - Class A Common Stock\",NASDAQ Global MarketSM
VERY,\"Vericity, Inc. - Common Stock\",NASDAQ Capital Market
VEV,Vicinity Motor Corp. - Common Stock,NASDAQ Capital Market
VFF,\"Village Farms International, Inc. - Common Shares\",NASDAQ Capital Market
VFLO,VictoryShares Free Cash Flow ETF,NASDAQ Global MarketSM
VFS,VinFast Auto Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
VFSWW,VinFast Auto Ltd. - Warrant,NASDAQ Capital Market
VGAS,\"Verde Clean Fuels, Inc. - Class A Common Stock\",NASDAQ Capital Market
VGASW,\"Verde Clean Fuels, Inc. - Warrant\",NASDAQ Capital Market
VGIT,Vanguard Intermediate-Term Treasury ETF,NASDAQ Global MarketSM
VGLT,Vanguard Long-Term Treasury ETF,NASDAQ Global MarketSM
VGSH,Vanguard Short-Term Treasury ETF,NASDAQ Global MarketSM
VGSR,Vert Global Sustainable Real Estate ETF,NASDAQ Global MarketSM
VIA,\"Via Renewables, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
VIASP,\"Via Renewables, Inc. - 8.75% Series A Fixed-to-Floating Rate Cumulative Redeemable Perpetual Preferred Stock\",NASDAQ Global Select MarketSM
VIAV,Viavi Solutions Inc. - Common Stock,NASDAQ Global Select MarketSM
VICR,Vicor Corporation - Common Stock,NASDAQ Global Select MarketSM
VIGI,Vanguard International Dividend Appreciation ETF,NASDAQ Global MarketSM
VIGL,\"Vigil Neuroscience, Inc. - Common Stock\",NASDAQ Global Select MarketSM
VINC,\"Vincerx Pharma, Inc. - Common Stock\",NASDAQ Capital Market
VINO,\"Gaucho Group Holdings, Inc. - Common Stock\",NASDAQ Capital Market
VINP,Vinci Partners Investments Ltd. - Class A Common Shares,NASDAQ Global Select MarketSM
VIOT,\"Viomi Technology Co., Ltd - American Depositary Shares\",NASDAQ Global Select MarketSM
VIR,\"Vir Biotechnology, Inc. - Common Stock\",NASDAQ Global Select MarketSM
VIRC,Virco Manufacturing Corporation - Common Stock,NASDAQ Global MarketSM
VIRI,\"Virios Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
VIRT,\"Virtu Financial, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
VIRX,\"Viracta Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
VISL,\"Vislink Technologies, Inc. - Common Stock\",NASDAQ Capital Market
VITL,\"Vital Farms, Inc. - Common Stock\",NASDAQ Global MarketSM
VIVK,\"Vivakor, Inc. - Common Stock\",NASDAQ Capital Market
VKTX,\"Viking Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
VLCN,\"Volcon, Inc. - Common stock\",NASDAQ Capital Market
VLGEA,\"Village Super Market, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
VLY,Valley National Bancorp - Common Stock,NASDAQ Global Select MarketSM
VLYPO,Valley National Bancorp - 5.5% Fixed to Floating Rate Series B Non-Cumulative Perpetual Preferred Stock,NASDAQ Global Select MarketSM
VLYPP,Valley National Bancorp - 6.25% Fixed-to-Floating Rate Series A Non-Cumulative Perpetual Preferred Stock,NASDAQ Global Select MarketSM
VMAR,Vision Marine Technologies Inc. - Common Shares,NASDAQ Capital Market
VMBS,Vanguard Mortgage-Backed Securities ETF,NASDAQ Global MarketSM
VMCA,Valuence Merger Corp. I - Class A Ordinary Shares,NASDAQ Global MarketSM
VMCAU,Valuence Merger Corp. I - Unit,NASDAQ Global MarketSM
VMCAW,Valuence Merger Corp. I - Warrant,NASDAQ Global MarketSM
VMD,\"Viemed Healthcare, Inc. - Common Shares\",NASDAQ Capital Market
VMEO,\"Vimeo, Inc. - Common Stock\",NASDAQ Global Select MarketSM
VMOT,Alpha Architect Value Momentum Trend ETF,NASDAQ Global MarketSM
VNDA,Vanda Pharmaceuticals Inc. - Common Stock,NASDAQ Global MarketSM
VNET,\"VNET Group, Inc. - American Depositary Shares\",NASDAQ Global Select MarketSM
VNOM,\"Viper Energy, Inc.  - Class A Common Stock\",NASDAQ Global Select MarketSM
VNQI,Vanguard Global ex-U.S. Real Estate ETF,NASDAQ Global MarketSM
VOD,Vodafone Group Plc - American Depositary Shares each representing ten Ordinary Shares,NASDAQ Global Select MarketSM
VONE,Vanguard Russell 1000 ETF,NASDAQ Global MarketSM
VONG,Vanguard Russell 1000 Growth ETF,NASDAQ Global MarketSM
VONV,Vanguard Russell 1000 Value ETF,NASDAQ Global MarketSM
VOR,Vor Biopharma Inc. - Common Stock,NASDAQ Global Select MarketSM
VOXR,Vox Royalty Corp. - common stock,NASDAQ Capital Market
VOXX,VOXX International Corporation - Class A Common Stock,NASDAQ Global Select MarketSM
VPLS,Vanguard Core Plus Bond ETF,NASDAQ Global MarketSM
VRA,\"Vera Bradley, Inc. - Common Stock\",NASDAQ Global Select MarketSM
VRAR,\"The Glimpse Group, Inc. - Common Stock\",NASDAQ Capital Market
VRAX,Virax Biolabs Group Limited - Ordinary Shares,NASDAQ Capital Market
VRCA,Verrica Pharmaceuticals Inc. - Common Stock,NASDAQ Global MarketSM
VRDN,\"Viridian Therapeutics, Inc.  - Common Stock\",NASDAQ Capital Market
VREX,Varex Imaging Corporation - Common Stock,NASDAQ Global Select MarketSM
VRIG,Invesco Variable Rate Investment Grade ETF,NASDAQ Global MarketSM
VRM,\"Vroom, Inc. - Common Stock\",NASDAQ Global Select MarketSM
VRME,\"VerifyMe, Inc. - Common Stock\",NASDAQ Capital Market
VRMEW,\"VerifyMe, Inc. - Warrant\",NASDAQ Capital Market
VRNA,Verona Pharma plc - American Depositary Shares,NASDAQ Global MarketSM
VRNS,\"Varonis Systems, Inc. - Common Stock\",NASDAQ Global Select MarketSM
VRNT,Verint Systems Inc. - Common Stock,NASDAQ Global Select MarketSM
VRPX,\"Virpax Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
VRRM,Verra Mobility Corporation - Class A Common Stock,NASDAQ Capital Market
VRSK,\"Verisk Analytics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
VRSN,\"VeriSign, Inc. - Common Stock\",NASDAQ Global Select MarketSM
VRTX,Vertex Pharmaceuticals Incorporated - Common Stock,NASDAQ Global Select MarketSM
VS,Versus Systems Inc. - Common Shares,NASDAQ Capital Market
VSAC,Vision Sensing Acquisition Corp. - Class A Common Stock,NASDAQ Global MarketSM
VSACU,Vision Sensing Acquisition Corp. - Unit,NASDAQ Global MarketSM
VSACW,Vision Sensing Acquisition Corp. - Warrants,NASDAQ Global MarketSM
VSAT,\"ViaSat, Inc. - Common Stock\",NASDAQ Global Select MarketSM
VSDA,VictoryShares Dividend Accelerator ETF,NASDAQ Global MarketSM
VSEC,VSE Corporation - Common Stock,NASDAQ Global Select MarketSM
VSME,VS Media Holdings Limited - Class A Ordinary Shares,NASDAQ Capital Market
VSMV,VictoryShares US Multi-Factor Minimum Volatility ETF,NASDAQ Global MarketSM
VSSYW,Versus Systems Inc. - Class A Warrants,NASDAQ Capital Market
VSTA,Vasta Platform Limited - Class A Ordinary Shares,NASDAQ Global Select MarketSM
VSTE,Vast Renewables Limited - Ordinary Shares,NASDAQ Global MarketSM
VSTEW,Vast Renewables Limited - Warrants,NASDAQ Capital Market
VSTM,\"Verastem, Inc. - Common Stock\",NASDAQ Capital Market
VTC,Vanguard Total Corporate Bond ETF,NASDAQ Global MarketSM
VTGN,\"Vistagen Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
VTHR,Vanguard Russell 3000 ETF,NASDAQ Global MarketSM
VTIP,Vanguard Short-Term Inflation-Protected Securities Index Fund ETF Shares,NASDAQ Global MarketSM
VTNR,\"Vertex Energy, Inc - Common Stock\",NASDAQ Capital Market
VTRS,Viatris Inc. - Common Stock,NASDAQ Global Select MarketSM
VTRU,Vitru Limited - Common Shares,NASDAQ Global Select MarketSM
VTSI,\"VirTra, Inc. - Common Stock\",NASDAQ Capital Market
VTVT,vTv Therapeutics Inc. - Class A Common Stock,NASDAQ Capital Market
VTWG,Vanguard Russell 2000 Growth ETF,NASDAQ Global MarketSM
VTWO,Vanguard Russell 2000 ETF,NASDAQ Global MarketSM
VTWV,Vanguard Russell 2000 Value ETF,NASDAQ Global MarketSM
VTYX,\"Ventyx Biosciences, Inc. - Common Stock\",NASDAQ Global Select MarketSM
VUZI,Vuzix Corporation  - Common Stock,NASDAQ Capital Market
VVOS,\"Vivos Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
VVPR,VivoPower International PLC - Ordinary Shares,NASDAQ Capital Market
VWE,\"Vintage Wine Estates, Inc. - Common Stock\",NASDAQ Capital Market
VWEWW,\"Vintage Wine Estates, Inc. - Warrants\",NASDAQ Capital Market
VWOB,Vanguard Emerging Markets Government Bond ETF,NASDAQ Global MarketSM
VXRT,\"Vaxart, Inc. - Common Stock\",NASDAQ Capital Market
VXUS,Vanguard Total International Stock ETF,NASDAQ Global MarketSM
VYGR,\"Voyager Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
VYMI,Vanguard International High Dividend Yield ETF,NASDAQ Global MarketSM
VYNE,VYNE Therapeutics Inc. - Common Stock,NASDAQ Capital Market
WABC,Westamerica Bancorporation - Common Stock,NASDAQ Global Select MarketSM
WABF,Western Asset Bond ETF,NASDAQ Global MarketSM
WAFD,\"WaFd, Inc. - Common Stock\",NASDAQ Global Select MarketSM
WAFDP,\"WaFd, Inc. - Depositary Shares\",NASDAQ Global Select MarketSM
WAFU,Wah Fu Education Group Limited - Ordinary Shares,NASDAQ Capital Market
WALD,Waldencast plc - Class A Ordinary Share,NASDAQ Capital Market
WALDW,Waldencast plc - Warrant,NASDAQ Capital Market
WASH,\"Washington Trust Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
WATT,Energous Corporation - Common Stock,NASDAQ Capital Market
WAVD,\"WaveDancer, Inc.  - Common Stock\",NASDAQ Capital Market
WAVE,Eco Wave Power Global AB (publ) - American Depositary Shares,NASDAQ Capital Market
WAVS,Western Acquisition Ventures Corp. - Common Stock,NASDAQ Capital Market
WAVSU,Western Acquisition Ventures Corp. - Unit,NASDAQ Capital Market
WAVSW,Western Acquisition Ventures Corp. - Warrant,NASDAQ Capital Market
WB,Weibo Corporation - American Depositary Shares,NASDAQ Global Select MarketSM
WBA,\"Walgreens Boots Alliance, Inc. - Common Stock\",NASDAQ Global Select MarketSM
WBD,\"Warner Bros. Discovery, Inc. - Series A Common Stock\",NASDAQ Global Select MarketSM
WBND,Western Asset Total Return ETF,NASDAQ Global MarketSM
WBUY,WEBUY GLOBAL LTD. - Ordinary Shares,NASDAQ Capital Market
WCBR,WisdomTree Cybersecurity Fund,NASDAQ Global MarketSM
WCLD,WisdomTree Cloud Computing Fund,NASDAQ Global MarketSM
WDAY,\"Workday, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
WDC,Western Digital Corporation - Common Stock,NASDAQ Global Select MarketSM
WDFC,WD-40 Company - Common Stock,NASDAQ Global Select MarketSM
WEEI,Westwood Salient Enhanced Energy Income ETF,NASDAQ Global MarketSM
WEN,Wendy's Company (The) - Common Stock,NASDAQ Global Select MarketSM
WERN,\"Werner Enterprises, Inc. - Common Stock\",NASDAQ Global Select MarketSM
WEST,Westrock Coffee Company - Common Stock,NASDAQ Global MarketSM
WESTW,Westrock Coffee Company - Warrants,NASDAQ Global MarketSM
WETH,Wetouch Technology Inc. - Common Stock,NASDAQ Capital Market
WEYS,\"Weyco Group, Inc. - Common Stock\",NASDAQ Global Select MarketSM
WFCF,\"Where Food Comes From, Inc. - Common Stock\",NASDAQ Capital Market
WFRD,Weatherford International plc - Ordinary shares,NASDAQ Global Select MarketSM
WGMI,Valkyrie Bitcoin Miners ETF,NASDAQ Global MarketSM
WGS,GeneDx Holdings Corp. - Class A Common Stock,NASDAQ Global Select MarketSM
WGSWW,GeneDx Holdings Corp. - Warrant,NASDAQ Global Select MarketSM
WHF,\"WhiteHorse Finance, Inc. - Closed End Fund\",NASDAQ Global Select MarketSM
WHFCL,\"WhiteHorse Finance, Inc. - 7.875% Notes due 2028\",NASDAQ Global Select MarketSM
WHLM,\"Wilhelmina International, Inc. - Common Stock\",NASDAQ Capital Market
WHLR,\"Wheeler Real Estate Investment Trust, Inc. - Common Stock\",NASDAQ Capital Market
WHLRD,\"Wheeler Real Estate Investment Trust, Inc. - Series D Cumulative Preferred Stock\",NASDAQ Capital Market
WHLRL,\"Wheeler Real Estate Investment Trust, Inc. - 7.00% Senior Subordinated Convertible Notes Due 2031\",NASDAQ Capital Market
WHLRP,\"Wheeler Real Estate Investment Trust, Inc. - Series B Preferred Stock\",NASDAQ Capital Market
WILC,\"G. Willi-Food International,  Ltd. - Ordinary Shares\",NASDAQ Capital Market
WIMI,WiMi Hologram Cloud Inc. - American Depositary Share,NASDAQ Global MarketSM
WINA,Winmark Corporation - Common Stock,NASDAQ Global MarketSM
WINC,Western Asset Short Duration Income ETF,NASDAQ Global MarketSM
WING,Wingstop Inc. - Common Stock,NASDAQ Global Select MarketSM
WINT,\"Windtree Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
WINV,WinVest Acquisition Corp. - Common Stock,NASDAQ Capital Market
WINVR,WinVest Acquisition Corp. - Right,NASDAQ Capital Market
WINVU,WinVest Acquisition Corp. - Unit,NASDAQ Capital Market
WINVW,WinVest Acquisition Corp. - Warrant,NASDAQ Capital Market
WIRE,Encore Wire Corporation - Common Stock,NASDAQ Global Select MarketSM
WISA,\"WiSA Technologies, Inc. - Common Stock\",NASDAQ Capital Market
WISE,Themes Generative Artificial Intelligence ETF,NASDAQ Global MarketSM
WIX,Wix.com Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
WKEY,WISeKey International Holding Ltd - American Depositary Shares,NASDAQ Global MarketSM
WKHS,\"Workhorse Group, Inc. - Common Stock\",NASDAQ Capital Market
WKME,WalkMe Ltd. - Ordinary Shares,NASDAQ Global Select MarketSM
WKSP,\"Worksport, Ltd. - Common Stock\",NASDAQ Capital Market
WKSPW,\"Worksport, Ltd. - Warrant\",NASDAQ Capital Market
WLDN,\"Willdan Group, Inc. - Common Stock\",NASDAQ Global MarketSM
WLDS,Wearable Devices Ltd. - Ordinary Share,NASDAQ Capital Market
WLDSW,Wearable Devices Ltd. - Warrant,NASDAQ Capital Market
WLFC,Willis Lease Finance Corporation - Common Stock,NASDAQ Global MarketSM
WLGS,\"Wang & Lee Group, Inc. - Ordinary Shares\",NASDAQ Capital Market
WMG,Warner Music Group Corp. - Class A Common Stock,NASDAQ Global Select MarketSM
WMPN,William Penn Bancorporation - Common Stock,NASDAQ Capital Market
WNDY,Global X Wind Energy ETF,NASDAQ Global MarketSM
WNEB,\"Western New England Bancorp, Inc. - Common Stock\",NASDAQ Global Select MarketSM
WNW,Meiwu Technology Company Limited - Ordinary Shares,NASDAQ Capital Market
WOOD,iShares Global Timber & Forestry ETF,NASDAQ Global MarketSM
WOOF,\"Petco Health and Wellness Company, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
WORX,SCWorx Corp. - Common Stock,NASDAQ Capital Market
WPRT,Westport Fuel Systems Inc - Common Shares,NASDAQ Global Select MarketSM
WRAP,\"Wrap Technologies, Inc. - Common Stock\",NASDAQ Capital Market
WRLD,World Acceptance Corporation - Common Stock,NASDAQ Global Select MarketSM
WRND,IQ Global Equity R&D Leaders ETF,NASDAQ Global MarketSM
WRNT,Warrantee Inc. - American Depositary Shares,NASDAQ Capital Market
WSBC,\"WesBanco, Inc. - Common Stock\",NASDAQ Global Select MarketSM
WSBCP,\"WesBanco, Inc. - Depositary Shares, Each Representing a 1/40th Interest in a Share of 6.75% Fixed-Rate Reset Non-Cumulative Perpetual Preferred Stock, Series A\",NASDAQ Global Select MarketSM
WSBF,\"Waterstone Financial, Inc. - Common Stock\",NASDAQ Global Select MarketSM
WSC,WillScot Mobile Mini Holdings Corp. - Class A Common Stock,NASDAQ Capital Market
WSFS,WSFS Financial Corporation - Common Stock,NASDAQ Global Select MarketSM
WTBA,West Bancorporation - Common Stock,NASDAQ Global Select MarketSM
WTBN,WisdomTree Bianco Total Return Fund,NASDAQ Global MarketSM
WTFC,Wintrust Financial Corporation - Common Stock,NASDAQ Global Select MarketSM
WTFCM,\"Wintrust Financial Corporation - Fixed-to-Floating Rate Non-Cumulative Perpetual Preferred Stock, Series D\",NASDAQ Global Select MarketSM
WTFCP,\"Wintrust Financial Corporation - Depositary Shares, Each Representing a 1/1,000th Interest in a Share of 6.875% Fixed-Rate Reset Non-Cumulative Perpetual Preferred Stock, Series E\",NASDAQ Global Select MarketSM
WTMA,Welsbach Technology Metals Acquisition Corp. - Common Stock,NASDAQ Global MarketSM
WTMAR,Welsbach Technology Metals Acquisition Corp. - one right to receive 1/10th of a share of common stock,NASDAQ Global MarketSM
WTMAU,Welsbach Technology Metals Acquisition Corp. - Unit,NASDAQ Global MarketSM
WTO,UTime Limited - Ordinary Shares,NASDAQ Capital Market
WTW,Willis Towers Watson Public Limited Company - Ordinary Shares,NASDAQ Global Select MarketSM
WULF,TeraWulf Inc. - Common Stock,NASDAQ Capital Market
WVE,Wave Life Sciences Ltd. - Ordinary Shares,NASDAQ Global MarketSM
WVVI,\"Willamette Valley Vineyards, Inc. - Common Stock\",NASDAQ Capital Market
WVVIP,\"Willamette Valley Vineyards, Inc. - Series A Redeemable Preferred Stock\",NASDAQ Capital Market
WW,\"WW International, Inc.  - Common Stock\",NASDAQ Global Select MarketSM
WWD,\"Woodward, Inc. - Common Stock\",NASDAQ Global Select MarketSM
WYNN,\"Wynn Resorts, Limited - Common Stock\",NASDAQ Global Select MarketSM
XAIR,\"Beyond Air, Inc. - Common Stock\",NASDAQ Capital Market
XBIL,US Treasury 6 Month Bill ETF,NASDAQ Global MarketSM
XBIO,\"Xenetic Biosciences, Inc. - Common Stock\",NASDAQ Capital Market
XBIOW,\"Xenetic Biosciences, Inc. - Warrants\",NASDAQ Capital Market
XBIT,XBiotech Inc. - Common Stock,NASDAQ Global Select MarketSM
XBP,\"XBP Europe Holdings, Inc. - Common Stock\",NASDAQ Global MarketSM
XBPEW,\"XBP Europe Holdings, Inc. - Warrant\",NASDAQ Capital Market
XCUR,\"Exicure, Inc. - Common Stock\",NASDAQ Capital Market
XEL,Xcel Energy Inc. - Common Stock,NASDAQ Global Select MarketSM
XELA,\"Exela Technologies, Inc. - Common Stock\",NASDAQ Capital Market
XELAP,\"Exela Technologies, Inc. - 6.00% Series B Cumulative Convertible Perpetual Preferred Stock\",NASDAQ Capital Market
XELB,\"Xcel Brands, Inc - Common Stock\",NASDAQ Capital Market
XENE,Xenon Pharmaceuticals Inc. - Common Shares,NASDAQ Global MarketSM
XERS,\"Xeris Biopharma Holdings, Inc. - Common Stock\",NASDAQ Global Select MarketSM
XFIN,ExcelFin Acquisition Corp - Class A Common Stock,NASDAQ Global MarketSM
XFINU,ExcelFin Acquisition Corp - Unit,NASDAQ Global MarketSM
XFINW,ExcelFin Acquisition Corp - Warrant,NASDAQ Global MarketSM
XFIX,F/m Opportunistic Income ETF,NASDAQ Global MarketSM
XFOR,\"X4 Pharmaceuticals, Inc. - Common Stock\",NASDAQ Capital Market
XGN,Exagen Inc. - Common Stock,NASDAQ Global MarketSM
XLO,\"Xilio Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
XMTR,\"Xometry, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
XNCR,\"Xencor, Inc. - Common Stock\",NASDAQ Global MarketSM
XNET,Xunlei Limited - American Depositary Shares,NASDAQ Global Select MarketSM
XOMA,XOMA Corporation - Common Stock,NASDAQ Global MarketSM
XOMAO,XOMA Corporation - Depositary Shares Rep Series B 8.375% Cumulative Preferred Stock,NASDAQ Global MarketSM
XOMAP,XOMA Corporation - 8.625% Series A Cumulative Perpetual Preferred Stock,NASDAQ Global MarketSM
XOS,\"Xos, Inc. - Common Stock\",NASDAQ Capital Market
XOSWW,\"Xos, Inc. - Warrants\",NASDAQ Capital Market
XP,XP Inc. - Class A Common Stock,NASDAQ Global Select MarketSM
XPEL,\"XPEL, Inc. - Common Stock\",NASDAQ Capital Market
XPON,Expion360 Inc. - Common Stock,NASDAQ Capital Market
XRAY,DENTSPLY SIRONA Inc. - Common Stock,NASDAQ Global Select MarketSM
XRTX,XORTX Therapeutics Inc. - Common Stock,NASDAQ Capital Market
XRX,Xerox Holdings Corporation - Common Stock,NASDAQ Global Select MarketSM
XT,iShares Exponential Technologies ETF,NASDAQ Global MarketSM
XTIA,\"XTI Aerospace, Inc. Common Stock  - Common Stock\",NASDAQ Capital Market
XTKG,\"X3 Holdings Co., Ltd. - Ordinary Shares\",NASDAQ Capital Market
XTLB,XTL Biopharmaceuticals Ltd. - American Depositary Shares,NASDAQ Capital Market
XWEL,\"XWELL, Inc. - Common Stock\",NASDAQ Capital Market
XXII,\"22nd Century Group, Inc - Common Stock\",NASDAQ Capital Market
XYLO,Xylo Technologies Ltd. - American Depositary Shares,NASDAQ Capital Market
YGMZ,MingZhu Logistics Holdings Limited - Ordinary Shares,NASDAQ Capital Market
YHGJ,Yunhong Green CTI Ltd. - Common Stock,NASDAQ Capital Market
YI,\"111, Inc. - American Depositary Shares\",NASDAQ Global MarketSM
YIBO,Planet Image International Limited - Class A Ordinary Shares,NASDAQ Capital Market
YJ,Yunji Inc. - American Depository Shares,NASDAQ Capital Market
YLDE,ClearBridge Dividend Strategy ESG ETF,NASDAQ Global MarketSM
YMAB,\"Y-mAbs Therapeutics, Inc. - Common Stock\",NASDAQ Global Select MarketSM
YNDX,Yandex N.V. - Class A Ordinary Shares,NASDAQ Global Select MarketSM
YORW,The York Water Company - Common Stock,NASDAQ Global Select MarketSM
YOSH,Yoshiharu Global Co. - Class A Common Stock,NASDAQ Capital Market
YOTA,Yotta Acquisition Corporation - Common Stock,NASDAQ Global MarketSM
YOTAR,Yotta Acquisition Corporation - Right,NASDAQ Global MarketSM
YOTAU,Yotta Acquisition Corporation - Unit,NASDAQ Global MarketSM
YOTAW,Yotta Acquisition Corporation - Warrant,NASDAQ Global MarketSM
YQ,17 Education & Technology Group Inc. - American Depositary Shares,NASDAQ Global Select MarketSM
YS,\"YS Biopharma Co., Ltd. - Ordinary Shares\",NASDAQ Capital Market
YSBPW,\"YS Biopharma Co., Ltd. - Warrants\",NASDAQ Capital Market
YTRA,\"Yatra Online, Inc. - Ordinary Shares\",NASDAQ Capital Market
YY,JOYY Inc. - American Depositary Shares,NASDAQ Global Select MarketSM
YYAI,Connexa Sports Technologies Inc. - Common Stock,NASDAQ Capital Market
YYGH,YY Group Holding Limited - Class A Ordinary Shares,NASDAQ Capital Market
Z,\"Zillow Group, Inc. - Class C Capital Stock\",NASDAQ Global Select MarketSM
ZAPP,Zapp Electric Vehicles Group Limited - Ordinary shares,NASDAQ Global MarketSM
ZAPPW,Zapp Electric Vehicles Group Limited - Warrant,NASDAQ Capital Market
ZBAO,Zhibao Technology Inc. - Class A Ordinary Shares,NASDAQ Capital Market
ZBRA,Zebra Technologies Corporation - Class A Common Stock,NASDAQ Global Select MarketSM
ZCAR,\"Zoomcar Holdings, Inc. - Common Stock\",NASDAQ Global MarketSM
ZCARW,\"Zoomcar Holdings, Inc. - Warrants\",NASDAQ Capital Market
ZCMD,Zhongchao Inc. - Class A Ordinary Shares,NASDAQ Capital Market
ZD,\"Ziff Davis, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ZENV,Zenvia Inc. - Class A Common Stock,NASDAQ Capital Market
ZEO,Zeo Energy Corporation - Class A Common Stock,NASDAQ Capital Market
ZEOWW,Zeo Energy Corporation - Warrants,NASDAQ Capital Market
ZEUS,\"Olympic Steel, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ZG,\"Zillow Group, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
ZI,ZoomInfo Technologies Inc. - Common Stock,NASDAQ Global Select MarketSM
ZIMV,ZimVie Inc. - Common Stock,NASDAQ Global Select MarketSM
ZION,Zions Bancorporation N.A. - Common Stock,NASDAQ Global Select MarketSM
ZIONL,\"Zions Bancorporation N.A. - 6.95% Fixed-to-Floating Rate Subordinated Notes due September 15, 2028\",NASDAQ Global MarketSM
ZIONO,Zions Bancorporation N.A. - Depositary Shares each representing a 1/40th ownership interest in a share of Series G Fixed/Floating-Rate Non-Cumulative Perpetual Preferred Stock,NASDAQ Global MarketSM
ZIONP,Zions Bancorporation N.A. - Depositary Shares each representing a 1/40th ownership interest in a share of Series A Floating-Rate Non-Cumulative Perpetual Preferred Stock,NASDAQ Global MarketSM
ZJYL,JIN MEDICAL INTERNATIONAL LTD. - Ordinary Shares,NASDAQ Capital Market
ZKIN,\"ZK International Group Co., Ltd - Ordinary Share\",NASDAQ Capital Market
ZLAB,Zai Lab Limited - American Depositary Shares,NASDAQ Global MarketSM
ZLS,Zalatoris II Acquisition Corp - Class A Ordinary Shares,NASDAQ Capital Market
ZLSWU,Zalatoris II Acquisition Corp - Unit,NASDAQ Capital Market
ZLSWW,Zalatoris II Acquisition Corp - Warrant,NASDAQ Capital Market
ZM,\"Zoom Video Communications, Inc. - Class A Common Stock\",NASDAQ Global Select MarketSM
ZNTL,\"Zentalis Pharmaceuticals, Inc. - common stock\",NASDAQ Global MarketSM
ZOOZ,ZOOZ Power Ltd. - Ordinary Shares,NASDAQ Capital Market
ZOOZW,ZOOZ Power Ltd. - Warrant,NASDAQ Capital Market
ZPTA,Zapata Computing Holdings Inc.  - Common Stock,NASDAQ Global MarketSM
ZPTAW,Zapata Computing Holdings Inc.  - Warrant,NASDAQ Capital Market
ZS,\"Zscaler, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ZTEK,Zentek Ltd. - common stock,NASDAQ Capital Market
ZUMZ,Zumiez Inc. - Common Stock,NASDAQ Global Select MarketSM
ZURA,Zura Bio Limited - Class A Ordinary shares,NASDAQ Capital Market
ZURAW,Zura Bio Limited - Warrant,NASDAQ Capital Market
ZVRA,\"Zevra Therapeutics, Inc.  - Common Stock\",NASDAQ Global Select MarketSM
ZVSA,\"ZyVersa Therapeutics, Inc. - Common Stock\",NASDAQ Capital Market
ZYME,Zymeworks Inc. - Common Stock,NASDAQ Global Select MarketSM
ZYXI,\"Zynex, Inc. - Common Stock\",NASDAQ Global Select MarketSM
ZZZ,Cyber Hornet S&P 500 and Bitcoin 75/25 Strategy ETF,NASDAQ Global MarketSM
A,\"Agilent Technologies, Inc. Common Stock\",NYSE
AA,Alcoa Corporation Common Stock ,NYSE
AACT,Ares Acquisition Corporation II Class A Ordinary Shares,NYSE
AACT.U,\"Ares Acquisition Corporation II Units, each consisting of one Class A ordinary share and one-half of one redeemable warrant\",NYSE
AACT.W,\"Ares Acquisition Corporation II Redeemable Warrants, each whole warrant exercisable for one Class A ordinary share at an exercise price of $11.50\",NYSE
AAMC,Altisource Asset Management Corp Com,NYSE MKT
AAN,\"Aarons Holdings Company, Inc. Common Stock \",NYSE
AAP,Advance Auto Parts Inc.,NYSE
AAT,\"American Assets Trust, Inc. Common Stock\",NYSE
AB,AllianceBernstein Holding L.P.  Units,NYSE
ABBV,AbbVie Inc. Common Stock,NYSE
ABEV,Ambev S.A. American Depositary Shares (Each representing 1 Common Share),NYSE
ABG,Asbury Automotive Group Inc Common Stock,NYSE
ABM,ABM Industries Incorporated Common Stock,NYSE
ABR,Arbor Realty Trust Common Stock,NYSE
ABR$D,\"Arbor Realty Trust 6.375% Series D Cumulative Redeemable Preferred Stock, Liquidation Preference $25.00 per Share\",NYSE
ABR$E,Arbor Realty Trust 6.25% Series E Cumulative Redeemable Preferred Stock,NYSE
ABR$F,\"Arbor Realty Trust 6.25% Series F Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock, Liquidation Preference $25.00 per share\",NYSE
ABT,Abbott Laboratories Common Stock,NYSE
AC,\"Associated Capital Group, Inc. Common Stock\",NYSE
ACA,\"Arcosa, Inc. Common Stock \",NYSE
ACCO,Acco Brands Corporation Common Stock,NYSE
ACEL,\"Accel Entertainment, Inc. \",NYSE
ACHR,Archer Aviation Inc. Class A Common Stock,NYSE
ACHR.W,\"Archer Aviation Inc. Redeemable Warrants, each whole warrant exercisable for one Class A common stock at an exercise price of $11.50\",NYSE
ACI,\"Albertsons Companies, Inc. Class A Common Stock\",NYSE
ACM,AECOM Common Stock,NYSE
ACN,Accenture plc Class A Ordinary Shares (Ireland),NYSE
ACP,abrdn Income Credit Strategies Fund Common Shares,NYSE
ACP$A,abrdn Income Credit Strategies Fund 5.250% Series A Perpetual Preferred Stock,NYSE
ACR,ACRES Commercial Realty Corp. Common Stock,NYSE
ACR$C,ACRES Commercial Realty Corp. 8.625% Fixed-to-Floating Series C Cumulative Redeemable Preferred Stock ,NYSE
ACR$D,ACRES Commercial Realty Corp. 7.875% Series D Cumulative Redeemable Preferred Stock,NYSE
ACRE,Ares Commercial Real Estate Corporation Common Stock,NYSE
ACU,Acme United Corporation. Common Stock,NYSE MKT
ACV,Virtus Diversified Income & Convertible Fund Common Shares of Beneficial Interest,NYSE
ADC,Agree Realty Corporation Common Stock,NYSE
ADC$A,\"Agree Realty Corporation Depositary Shares, each representing 1/1,000th of a 4.250% Series A Cumulative Redeemable Preferred Stock\",NYSE
ADCT,ADC Therapeutics SA Common Shares,NYSE
ADM,Archer-Daniels-Midland Company Common Stock,NYSE
ADNT,Adient plc Ordinary Shares ,NYSE
ADRT,Ault Disruptive Technologies Corporation Common Stock,NYSE MKT
ADRT.U,\"Ault Disruptive Technologies Corporation Units, each consisting of one share of Common Stock, and three-fourths of one Redeemable Warrant to purchase one share of Common Stock\",NYSE MKT
ADT,ADT Inc. Common Stock,NYSE
ADX,Adams Diversified Equity Fund Inc.,NYSE
AE,\"Adams Resources & Energy, Inc. Common Stock\",NYSE MKT
AEE,Ameren Corporation Common Stock,NYSE
AEF,\"abrdn Emerging Markets Equity Income Fund, Inc. Common Stock\",NYSE MKT
AEFC,Aegon Funding Company LLC 5.10% Subordinated Notes due 2049,NYSE
AEG,Aegon Ltd. New York Registry Shares,NYSE
AEM,Agnico Eagle Mines Limited Common Stock,NYSE
AEO,\"American Eagle Outfitters, Inc. Common Stock\",NYSE
AEON,\"AEON Biopharma, Inc. Class A Common Stock\",NYSE MKT
AER,AerCap Holdings N.V. Ordinary Shares,NYSE
AES,The AES Corporation Common Stock,NYSE
AESI,Atlas Energy Solutions Inc. Common Stock,NYSE
AEVA,\"Aeva Technologies, Inc. Common Stock\",NYSE
AEVA.W,\"Aeva Technologies, Inc. Redeemable Warrants, each whole warrant exercisable for shares of common stock at an exercise price of $11.50 per share\",NYSE
AFB,AllianceBernstein National Municipal Income Fund Inc,NYSE
AFG,\"American Financial Group, Inc. Common Stock\",NYSE
AFGB,\"American Financial Group, Inc. 5.875% Subordinated Debentures due 2059\",NYSE
AFGC,\"American Financial Group, Inc. 5.125% Subordinated Debentures due 2059\",NYSE
AFGD,\"American Financial Group, Inc. 5.625% Subordinated Debentures due 2060\",NYSE
AFGE,\"American Financial Group, Inc. 4.500% Subordinated Debentures due 2060\",NYSE
AFL,AFLAC Incorporated Common Stock,NYSE
AFT,Apollo Senior Floating Rate Fund Inc. Common Stock,NYSE
AG,First Majestic Silver Corp. Ordinary Shares (Canada),NYSE
AGCO,AGCO Corporation Common Stock,NYSE
AGD,abrdn Global Dynamic Dividend Fund Common Shares of Beneficial Interest,NYSE
AGI,Alamos Gold Inc. Class A Common Shares,NYSE
AGL,\"agilon health, inc. Common Stock\",NYSE
AGM,Federal Agricultural Mortgage Corporation Common Stock,NYSE
AGM$C,Federal Agricultural Mortgage Corporation Preferred Series C Fixed to Fltg,NYSE
AGM$D,\"Federal Agricultural Mortgage Corporation 5.700% Non-Cumulative Preferred Stock, Series D\",NYSE
AGM$E,\"Federal Agricultural Mortgage Corporation 5.750% Non-Cumulative Preferred Stock, Series E\",NYSE
AGM$F,\"Federal Agricultural Mortgage Corporation 5.250% Non-Cumulative Preferred Stock, Series F\",NYSE
AGM$G,\"Federal Agricultural Mortgage Corporation 4.875% Non-Cumulative Preferred Stock, Series G\",NYSE
AGM.A,Federal Agricultural Mortgage Corporation Common Stock,NYSE
AGO,Assured Guaranty Ltd. Common Stock,NYSE
AGR,\"Avangrid, Inc. Common Stock\",NYSE
AGRO,Adecoagro S.A. Common Shares,NYSE
AGS,\"PlayAGS, Inc. Common Stock\",NYSE
AGX,\"Argan, Inc. Common Stock\",NYSE
AHH,\"Armada Hoffler Properties, Inc. Common Stock\",NYSE
AHH$A,\"Armada Hoffler Properties, Inc. 6.75% Series A Cumulative Redeemable Perpetual Preferred Stock\",NYSE
AHL$C,Aspen Insurance Holdings Limited 5.95% Fixed-to-Floating Rate Perpetual Non-Cumulative Preference Shares,NYSE
AHL$D,Aspen Insurance Holdings Limited 5.625% Perpetual Non-Cumulative Preference Shares,NYSE
AHL$E,\"Aspen Insurance Holdings Limited Depositary Shares, each representing a 1/1000th interest in a share of 5.625% Perpetual Non-Cumulative Preference Shares\",NYSE
AHR,\"American Healthcare REIT, Inc. Common Stock\",NYSE
AHT,Ashford Hospitality Trust Inc Common Stock,NYSE
AHT$D,Ashford Hospitality Trust Inc 8.45% Series D Cumulative Preferred Stock,NYSE
AHT$F,Ashford Hospitality Trust Inc 7.375% Series F Cumulative Preferred Stock,NYSE
AHT$G,Ashford Hospitality Trust Inc 7.375% Series G Cumulative Preferred Stock,NYSE
AHT$H,Ashford Hospitality Trust Inc 7.50% Series H Cumulative Preferred Stock,NYSE
AHT$I,Ashford Hospitality Trust Inc 7.50% Series I Cumulative Preferred Stock,NYSE
AI,\"C3.ai, Inc. Class A Common Stock\",NYSE
AIF,Apollo Tactical Income Fund Inc. Common Stock,NYSE
AIG,\"American International Group, Inc. New Common Stock\",NYSE
AIM,AIM ImmunoTech Inc. Common Stock,NYSE MKT
AIN,Albany International Corporation Common Stock,NYSE
AINC,Ashford Inc. (Holding Company) Common Stock,NYSE MKT
AIO,Virtus Artificial Intelligence & Technology Opportunities Fund Common Shares of Beneficial Interest,NYSE
AIR,AAR Corp. Common Stock,NYSE
AIRC,Apartment Income REIT Corp. Common Stock,NYSE
AIRI,Air Industries Group Common Stock,NYSE MKT
AIT,\"Applied Industrial Technologies, Inc. Common Stock\",NYSE
AIU,Meta Data Limited ADS,NYSE
AIV,Apartment Investment and Management Company Common Stock,NYSE
AIZ,\"Assurant, Inc. Common Stock\",NYSE
AIZN,\"Assurant, Inc. 5.25% Subordinated Notes due 2061\",NYSE
AJG,Arthur J. Gallagher & Co. Common Stock,NYSE
AJX,Great Ajax Corp. Common Stock,NYSE
AKA,a.k.a. Brands Holding Corp. Common Stock,NYSE
AKO.A,Embotelladora Andina S.A. Common Stock,NYSE
AKO.B,Embotelladora Andina S.A. Common Stock,NYSE
AKR,Acadia Realty Trust Common Stock,NYSE
AL,Air Lease Corporation Class A Common Stock,NYSE
AL$A,\"Air Lease Corporation 6.150% Fixed-to-Floating Rate Non-Cumulative Perpetual Preferred Stock, Series A\",NYSE
ALB,Albemarle Corporation Common Stock,NYSE
ALB$A,Albemarle Corporation Depositary Shares each representing a 1/20th of 7.25% Series A Mandatory Convertible Preferred Stock,NYSE
ALC,Alcon Inc. Ordinary Shares,NYSE
ALE,\"Allete, Inc.\",NYSE
ALEX,\"Alexander & Baldwin, Inc. Common Stock REIT Holding Company\",NYSE
ALG,\"Alamo Group, Inc. Common Stock\",NYSE
ALIT,\"Alight, Inc. Class A Common Stock\",NYSE
ALK,\"Alaska Air Group, Inc. Common Stock\",NYSE
ALL,Allstate Corporation (The) Common Stock,NYSE
ALL$B,Allstate Corporation (The) 5.100% Fixed-to-Floating Rate Subordinated Debentures due 2053,NYSE
ALL$H,\"Allstate Corporation (The) Depositary Shares each representing a 1/1,000th interest in a share of Fixed Rate Noncumulative Perpetual Preferred Stock, Series H\",NYSE
ALL$I,\"Allstate Corporation (The) Depositary Shares each representing a 1/1,000th interest in a share of Fixed Rate Noncumulative Perpetual Preferred Stock, Series I\",NYSE
ALL$J,\"Allstate Corporation (The) Depositary Shares each representing a 1/1,000th interest in a share of Fixed Rate Noncumulative Perpetual Preferred Stock, Series J\",NYSE
ALLE,Allegion plc Ordinary Shares,NYSE
ALLG,Allego N.V. Ordinary Share,NYSE
ALLY,Ally Financial Inc. Common Stock,NYSE
ALSN,\"Allison Transmission Holdings, Inc. Common Stock\",NYSE
ALTG,Alta Equipment Group Inc. Class A Common Stock,NYSE
ALTG$A,Alta Equipment Group Inc. Depositary Shares (each representing 1/1000th in a share of 10% Series A Cumulative Perpetual Preferred Stock),NYSE
ALTM,Arcadium Lithium plc Ordinary Shares,NYSE
ALUR,\"Allurion Technologies, Inc. Common Stock\",NYSE
ALUR.W,\"Allurion Technologies, Inc. Warrants, each whole warrant exercisable to purchase 1.420455 shares of common stock at an exercise price of $8.10 per share of common stock\",NYSE
ALV,\"Autoliv, Inc. Common Stock\",NYSE
ALX,\"Alexander's, Inc. Common Stock\",NYSE
AM,Antero Midstream Corporation Common Stock,NYSE
AMBC,\"Ambac Financial Group, Inc. Common Stock\",NYSE
AMBI,Ambipar Emergency Response Class A Ordinary Shares,NYSE MKT
AMBI.W,\"Ambipar Emergency Response Warrants to purchase Class A ordinary shares, each whole warrant exercisable for one Class A ordinary share at an exercise price of $11.50 per share\",NYSE MKT
AMBO,Ambow Education Holding Ltd. American Depository Shares (each representing twenty (20) Class A Ordinary Shares),NYSE MKT
AMBP,Ardagh Metal Packaging S.A. Ordinary Shares,NYSE
AMBP.W,\"Ardagh Metal Packaging S.A. Warrants, each exercisable for one Share at an exercise price of $11.50 per share\",NYSE
AMC,\"AMC Entertainment Holdings, Inc. Class A Common Stock\",NYSE
AMCR,Amcor plc Ordinary Shares,NYSE
AME,\"AMETEK, Inc.\",NYSE
AMG,\"Affiliated Managers Group, Inc. Common Stock\",NYSE
AMH,American Homes 4 Rent Common Shares of Beneficial Interest,NYSE
AMH$G,American Homes 4 Rent Series G cumulative redeemable perpetual preferred shares of beneficial interest,NYSE
AMH$H,American Homes 4 Rent Series H cumulative redeemable perpetual Preferred Shares of Beneficial Interest,NYSE
AMK,\"AssetMark Financial Holdings, Inc. Common Stock\",NYSE
AMN,AMN Healthcare Services Inc,NYSE
AMP,\"Ameriprise Financial, Inc. Common Stock\",NYSE
AMPS,\"Altus Power, Inc. Class A Common Stock\",NYSE
AMPX,\"Amprius Technologies, Inc. Common Stock\",NYSE
AMPX.W,\"Amprius Technologies, Inc. Warrants to purchase one share of Common Stock at an exercise price of $11.50 per share\",NYSE
AMPY,Amplify Energy Corp. Common Stock,NYSE
AMR,\"Alpha Metallurgical Resources, Inc. Common Stock\",NYSE
AMRC,\"Ameresco, Inc. Class A Common Stock\",NYSE
AMS,American Shared Hospital Services Common Stock,NYSE MKT
AMT,American Tower Corporation (REIT) Common Stock,NYSE
AMTB,Amerant Bancorp Inc. Class A Common Stock,NYSE
AMTD,\"AMTD IDEA Group American Depositary Shares, each representing six (6) Class A Ordinary Shares\",NYSE
AMTR,ETRACS Alerian Midstream Energy Total Return Index ETN,NYSE ARCA
AMUB,\"ETRACS Alerian MLP Index ETN Series B due July 18, 2042\",NYSE ARCA
AMWL,American Well Corporation Class A Common Stock,NYSE
AMX,\"America Movil, S.A.B. de C.V. American Depositary Shares (each representing the right to receive twenty (20) Series B Shares\",NYSE
AN,\"AutoNation, Inc. Common Stock\",NYSE
ANET,\"Arista Networks, Inc. Common Stock\",NYSE
ANF,Abercrombie & Fitch Company Common Stock,NYSE
ANG$A,\"American National Group Inc. Depositary Shares, each representing a 1/1,000th interest in a share of  5.95% Fixed-Rate Reset Non-Cumulative Preferred Stock, Series A\",NYSE
ANG$B,\"American National Group Inc. Depositary Shares, each representing a 1/1,000th interest in a share of 6.625% Fixed-Rate Reset Non-Cumulative Preferred Stock, Series B\",NYSE
ANRO,\"Alto Neuroscience, Inc. Common Stock\",NYSE
ANVS,\"Annovis Bio, Inc. Common Stock\",NYSE
AOD,abrdn Total Dynamic Dividend Fund Common Shares of Beneficial Interest,NYSE
AOMR,\"Angel Oak Mortgage REIT, Inc. Common Stock\",NYSE
AON,Aon plc Class A Ordinary Shares (Ireland),NYSE
AORT,\"Artivion, Inc. Common Stock\",NYSE
AOS,A.O. Smith Corporation Common Stock,NYSE
AP,Ampco-Pittsburgh Corporation Common Stock,NYSE
AP.W,Ampco-Pittsburgh Corporation Series A Warrants to purchase Shares of common stock,NYSE MKT
APAM,Artisan Partners Asset Management Inc. Class A Common Stock,NYSE
APCA,AP Acquisition Corp Class A Ordinary Shares,NYSE
APCA.U,\"AP Acquisition Corp Units, each consisting of one Class A ordinary share and one-half of one redeemable warrant\",NYSE
APCA.W,\"AP Acquisition Corp Redeemable Warrants, each whole warrant exercisable for one Class A ordinary share at an exercise price of $11.50\",NYSE
APD,\"Air Products and Chemicals, Inc. Common Stock\",NYSE
APG,APi Group Corporation Common Stock,NYSE
APH,Amphenol Corporation Common Stock,NYSE
APLE,\"Apple Hospitality REIT, Inc. Common Shares\",NYSE
APO,\"Apollo Global Management, Inc. (New) Common Stock\",NYSE
APO$A,\"Apollo Global Management, Inc. 6.75% Series A Mandatory Convertible Preferred Stock\",NYSE
APOS,\"Apollo Global Management, Inc. 7.625% Fixed-Rate Resettable Junior Subordinated Notes due 2053\",NYSE
APT,\"Alpha Pro Tech, Ltd. Common Stock\",NYSE MKT
APTV,Aptiv PLC Ordinary Shares,NYSE
AQN,Algonquin Power & Utilities Corp. Common Shares,NYSE
AQNB,\"Algonquin Power & Utilities Corp. 6.20% Fixed-to-Floating Subordinated Notes Series 2019-A due July 1, 2079\",NYSE
AQNU,Algonquin Power & Utilities Corp. Corporate Units,NYSE
AR,Antero Resources Corporation Common Stock,NYSE
ARC,\"ARC Document Solutions, Inc. Common Stock\",NYSE
ARCH,\"Arch Resources, Inc. Class A Common Stock\",NYSE
ARCO,Arcos Dorados Holdings Inc. Class A Shares,NYSE
ARDC,\"Ares Dynamic Credit Allocation Fund, Inc. Common Shares\",NYSE
ARE,\"Alexandria Real Estate Equities, Inc. Common Stock\",NYSE
AREN,\"The Arena Group Holdings, Inc. Common Stock\",NYSE MKT
ARES,Ares Management Corporation Class A Common Stock,NYSE
ARGD,\"Argo Group International Holdings, Ltd. 6.5% Senior Notes Due 2042\",NYSE
ARGO$A,\"Argo Group International Holdings, Inc. Depositary Shares, Each Representing a 1/1,000th Interest in a 7.00% Resettable Fixed Rate Preference Share, Series A\",NYSE
ARI,\"Apollo Commercial Real Estate Finance, Inc\",NYSE
ARIS,\"Aris Water Solutions, Inc. Class A Common Stock\",NYSE
ARL,\"American Realty Investors, Inc. Common Stock\",NYSE
ARLO,\"Arlo Technologies, Inc. Common Stock\",NYSE
ARMK,Aramark Common Stock,NYSE
ARMN,Aris Mining Corporation Common Shares,NYSE MKT
ARMP,\"Armata Pharmaceuticals, Inc. Common Stock\",NYSE MKT
AROC,\"Archrock, Inc. Common Stock\",NYSE
ARR,\"ARMOUR Residential REIT, Inc.\",NYSE
ARR$C,\"ARMOUR Residential REIT, Inc. 7% Series C Cumulative Redeemable Preferred Stock (liquidation preference $25.00 per share)\",NYSE
ARW,\"Arrow Electronics, Inc. Common Stock\",NYSE
AS,\"Amer Sports, Inc. Ordinary Shares\",NYSE
ASA,ASA  Gold and Precious Metals Limited,NYSE
ASAI,Sendas Distribuidora S A ADS,NYSE
ASAN,\"Asana, Inc. Class A Common Stock\",NYSE
ASB,Associated Banc-Corp Common Stock,NYSE
ASB$E,\"Associated Banc-Corp Depositary Shares, each representing a 1/40th interest in a share of 5.875% Non-Cumulative Perpetual Preferred Stock, Series E\",NYSE
ASB$F,\"Associated Banc-Corp Depositary Shares, each representing a 1/40th interest in a share of Associated Banc-Corp 5.625% Non-Cumulative Perpetual Preferred Stock, Series F\",NYSE
ASBA,Associated Banc-Corp 6.625% Fixed-Rate Reset Subordinated Notes due 2033,NYSE
ASC,Ardmore Shipping Corporation Common Stock,NYSE
ASG,\"Liberty All-Star Growth Fund, Inc.\",NYSE
ASGI,abrdn Global Infrastructure Income Fund Common Shares of Beneficial Interest,NYSE
ASGN,ASGN Incorporated Common Stock,NYSE
ASH,Ashland Inc. Common Stock,NYSE
ASIX,AdvanSix Inc. Common Stock ,NYSE
ASM,Avino Silver & Gold Mines Ltd. Common Shares (Canada),NYSE MKT
ASPN,\"Aspen Aerogels, Inc. Common Stock\",NYSE
ASR,\"Grupo Aeroportuario del Sureste, S.A. de C.V. Common Stock\",NYSE
ASX,\"ASE Technology Holding Co., Ltd. American Depositary Shares (each representing Two Common Shares) \",NYSE
ASXC,\"Asensus Surgical, Inc. Common Stock\",NYSE MKT
ATCH,\"AtlasClear Holdings, Inc. Common Stock\",NYSE MKT
ATCO$D,Atlas Corp. 7.95% Series D,NYSE
ATCO$H,Atlas Corp. 7.875% Series H,NYSE
ATEK,Athena Technology Acquisition Corp. II Class A Common Stock,NYSE MKT
ATEK.U,\"Athena Technology Acquisition Corp. II Units, each consisting of one share of Class A common stock, and one-half of one Redeemable Warrant\",NYSE MKT
ATEK.W,Athena Technology Acquisition Corp. II Redeemable Warrants,NYSE MKT
ATEN,\"A10 Networks, Inc. Common Stock\",NYSE
ATEST,Tick Pilot Test Control Common Stock,NYSE MKT
ATEST.A,Tick Pilot Test Group 1 Common Stock,NYSE MKT
ATEST.B,Tick pilot Test 2 Common Stock,NYSE MKT
ATEST.C,Tick Pilot Test 3 Common Stock,NYSE MKT
ATGE,Adtalem Global Education Inc. Common Stock,NYSE
ATH$A,\"Athene Holding Ltd. Depositary Shares, Each Representing a 1/1,000th Interest in a 6.35% Fixed-to-Floating Rate Perpetual Non-Cumulative Preference Share, Series A\",NYSE
ATH$B,\"Athene Holding Ltd. Depositary Shares, Each Representing a 1/1,000th Interest in a 5.625% Fixed Rate Perpetual Non- Cumulative Preference Share, Series B, par value $1.00 per share\",NYSE
ATH$C,\"Athene Holding Ltd. Depositary Shares, each representing a 1/1,000th Interest in a Share of 6.375% Fixed-Rate Reset Perpetual Non-Cumulative Preference Shares, Series C\",NYSE
ATH$D,\"Athene Holding Ltd. Depositary Shares, Each Representing a 1/1,000th Interest in a 4.875% Fixed-Rate Perpetual Non-Cumulative Preference Share, Series D\",NYSE
ATH$E,\"Athene Holding Ltd. Depositary Shares, Each Representing a 1/1,000th Interest in a 7.750% Fixed-Rate Reset Perpetual Non-Cumulative Preference Share, Series E\",NYSE
ATHM,\"Autohome Inc. American Depositary Shares, each representing four class A ordinary shares.\",NYSE
ATHS,Athene Holding Ltd. 7.250% Fixed-Rate Reset Junior Subordinated Debentures due 2064,NYSE
ATI,ATI Inc. Common Stock,NYSE
ATIP,\"ATI Physical Therapy, Inc. Class A Common Stock\",NYSE
ATKR,Atkore Inc. Common Stock,NYSE
ATMP,iPath Select MLP ETN,BATS
ATMU,Atmus Filtration Technologies Inc. Common Stock,NYSE
ATNM,\"Actinium Pharmaceuticals, Inc. (Delaware) Common Stock\",NYSE MKT
ATO,Atmos Energy Corporation Common Stock,NYSE
ATR,\"AptarGroup, Inc. Common Stock\",NYSE
ATS,ATS Corporation Common Shares,NYSE
ATUS,\"Altice USA, Inc. Class A Common Stock\",NYSE
AU,AngloGold Ashanti PLC Ordinary Shares,NYSE
AUB,Atlantic Union Bankshares Corporation Common Stock,NYSE
AUB$A,\"Atlantic Union Bankshares Corporation Depositary Shares each representing a 1/400th ownership interest in a share of 6.875% Perpetual Non-Cumulative Preferred Stock, Series A\",NYSE
AULT,\"Ault Alliance, Inc. Common Stock\",NYSE MKT
AULT$D,\"Ault Alliance, Inc. 13.00% Series D Cumulative Redeemable Perpetual Preferred Stock\",NYSE MKT
AUMN,Golden Minerals Company Common Stock,NYSE MKT
AUNA,Auna SA Class A Ordinary Shares,NYSE
AUST,Austin Gold Corp. Common Shares,NYSE MKT
AVA,Avista Corporation Common Stock,NYSE
AVAL,Grupo Aval Acciones y Valores S.A. ADR (Each representing 20 preferred shares),NYSE
AVB,\"AvalonBay Communities, Inc. Common Stock\",NYSE
AVD,American Vanguard Corporation Common Stock ($0.10 Par Value),NYSE
AVK,Advent Convertible and Income Fund,NYSE
AVNS,\"Avanos Medical, Inc. Common Stock\",NYSE
AVNT,Avient Corporation Common Stock,NYSE
AVTR,\"Avantor, Inc. Common Stock\",NYSE
AVY,Avery Dennison Corporation Common Stock,NYSE
AWF,Alliancebernstein Global High Income Fund,NYSE
AWI,Armstrong World Industries Inc Common Stock,NYSE
AWK,\"American Water Works Company, Inc. Common Stock\",NYSE
AWP,abrdn Global Premier Properties Fund Common Shares of Beneficial Interest,NYSE
AWR,American States Water Company Common Stock,NYSE
AWX,Avalon Holdings Corporation Common Stock,NYSE MKT
AX,\"Axos Financial, Inc. Common Stock\",NYSE
AXIL,\"AXIL Brands, Inc. Common Stock\",NYSE MKT
AXL,\"American Axle & Manufacturing Holdings, Inc. Common Stock\",NYSE
AXP,American Express Company Common Stock,NYSE
AXR,AMREP Corporation Common Stock,NYSE
AXS,Axis Capital Holdings Limited Common Stock,NYSE
AXS$E,\"Axis Capital Holdings Limited Depositary Shares, each representing 1/100th interest in a share of a 5.50% Series E Preferred Shares\",NYSE
AXTA,Axalta Coating Systems Ltd. Common Shares,NYSE
AYI,\"Acuity Brands, Inc. \",NYSE
AZEK,The AZEK Company Inc. Class A Common Stock,NYSE
AZO,\"AutoZone, Inc. Common Stock\",NYSE
AZTR,Azitra Inc Common Stock,NYSE MKT
AZUL,Azul S.A. American Depositary Shares (each representing three preferred shares),NYSE
AZZ,AZZ Inc.,NYSE
B,\"Barnes Group, Inc. Common Stock\",NYSE
BA,Boeing Company (The) Common Stock,NYSE
BABA,Alibaba Group Holding Limited American Depositary Shares each representing eight Ordinary share,NYSE
BAC,Bank of America Corporation Common Stock,NYSE
BAC$B,\"Bank of America Corporation Depositary Shares, each representing a 1/1,000th interest in a share of 6.000% Non-Cumulative Preferred Stock, Series GG\",NYSE
BAC$E,Bank of America Corporation Depositary Sh repstg 1/1000th Perp Pfd Ser E,NYSE
BAC$K,\"Bank of America Corporation Depositary Shares, each representing a 1/1,000th interest in a share of 5.875% Non- Cumulative Preferred Stock, Series HH\",NYSE
BAC$L,Bank of America Corporation Non Cumulative Perpetual Conv Pfd Ser L,NYSE
BAC$M,\"Bank of America Corporation Depositary Shares, each representing a 1/1,000th interest in a share of 5.375% Non-Cumulative Preferred Stock, Series KK\",NYSE
BAC$N,\"Bank of America Corporation Depositary shares, each representing 1/1,000th interest in a share of 5.000% Non-Cumulative Preferred Stock, Series LL\",NYSE
BAC$O,\"Bank of America Corporation Depositary shares, each representing 1/1,000th interest in a share of 4.375% Non-Cumulative Preferred Stock, Series NN\",NYSE
BAC$P,\"Bank of America Corporation Depositary Shares, each representing a 1/1,000th interest in a share of 4.125% Non-Cumulative Preferred Stock, Series PP\",NYSE
BAC$Q,\"Bank of America Corporation Depositary shares, each representing 1/1,000th interest in a share of 4.250% Non-Cumulative Preferred Stock, Series QQ\",NYSE
BAC$S,\"Bank of America Corporation Depositary shares, each representing 1/1,000th interest in a share of 4.750% Non-Cumulative Preferred Stock, Series SS\",NYSE
BACA,Berenson Acquisition Corp. I Class A Common Stock,NYSE MKT
BAH,Booz Allen Hamilton Holding Corporation Common Stock,NYSE
BAK,Braskem SA ADR,NYSE
BALL,Ball Corporation Common Stock,NYSE
BALY,Bally's Corporation Common Stock,NYSE
BAM,Brookfield Asset Management Inc Class A Limited Voting Shares,NYSE
BANC,\"Banc of California, Inc. Common Stock\",NYSE
BANC$F,\"Banc of California, Inc. Depositary Shares, each representing a 1/40th interest in a share of 7.75% non-cumulative perpetual preferred stock, Series F\",NYSE
BAP,Credicorp Ltd. Common Stock,NYSE
BAR,GraniteShares Gold Trust Shares of Beneficial Interest,NYSE ARCA
BARK,\"BARK, Inc. Class A Common Stock\",NYSE
BARK.W,\"BARK, Inc. Redeemable Warrants, each whole warrant exercisable for shares of Common Stock at an exercise price of $11.50 per share\",NYSE
BATL,Battalion Oil Corporation Common Stock,NYSE MKT
BAX,Baxter International Inc. Common Stock,NYSE
BB,BlackBerry Limited Common Stock,NYSE
BBAI,\"BigBear.ai, Inc. Common Stock\",NYSE
BBAI.W,\"BigBear.ai, Inc. Redeemable Warrants, each exercisable for one share of Common Stock at an exercise price of $11.50 per share\",NYSE
BBAR,Banco BBVA Argentina S.A. ADS,NYSE
BBD,Banco Bradesco Sa American Depositary Shares,NYSE
BBDC,\"Barings BDC, Inc. Common Stock\",NYSE
BBDO,Banco Bradesco Sa American Depositary Shares (each representing one Common Share),NYSE
BBN,BlackRock Taxable Municipal Bond Trust Common Shares of Beneficial Interest,NYSE
BBU,Brookfield Business Partners L.P. Limited Partnership Units ,NYSE
BBUC,Brookfield Business Corporation Class A Exchangeable Subordinate Voting Shares,NYSE
BBVA,Banco Bilbao Vizcaya Argentaria S.A. Common Stock,NYSE
BBW,\"Build-A-Bear Workshop, Inc. Common Stock\",NYSE
BBWI,\"Bath & Body Works, Inc.\",NYSE
BBY,\"Best Buy Co., Inc. Common Stock\",NYSE
BC,Brunswick Corporation Common Stock,NYSE
BC$A,Brunswick Corporation 6.500% Senior Notes due 2048,NYSE
BC$B,Brunswick Corporation 6.625% Senior Notes due 2049,NYSE
BC$C,Brunswick Corporation 6.375% Notes due 2049,NYSE
BCAT,BlackRock Capital Allocation Term Trust Common Shares of Beneficial Interest,NYSE
BCC,\"Boise Cascade, L.L.C. Common Stock\",NYSE
BCE,\"BCE, Inc. Common Stock\",NYSE
BCH,Banco De Chile ADS,NYSE
BCO,Brinks Company (The) Common Stock,NYSE
BCS,Barclays PLC Common Stock,NYSE
BCSF,\"Bain Capital Specialty Finance, Inc. Common Stock\",NYSE
BCV,\"Bancroft Fund, Ltd.\",NYSE MKT
BCV$A,Bancroft Fund Limited 5.375% Series A Cumulative Preferred Shares,NYSE MKT
BCX,BlackRock Resources Common Shares of Beneficial Interest,NYSE
BDC,Belden Inc Common Stock,NYSE
BDCZ,\"ETRACS MarketVector Business Development Companies Liquid Index ETN due April 26, 2041\",NYSE ARCA
BDJ,Blackrock Enhanced Equity Dividend Trust,NYSE
BDL,\"Flanigan's Enterprises, Inc. Common Stock\",NYSE MKT
BDN,Brandywine Realty Trust Common Stock,NYSE
BDX,\"Becton, Dickinson and Company Common Stock\",NYSE
BE,Bloom Energy Corporation Class A Common Stock,NYSE
BEDU,\"Bright Scholar Education Holdings Limited American Depositary Shares, each  representing four Class A Ordinary Share\",NYSE
BEEP,Mobile Infrastructure Corporation Common Stock,NYSE MKT
BEKE,KE Holdings Inc American Depositary Shares (each representing three Class A Ordinary Shares),NYSE
BEN,\"Franklin Resources, Inc. Common Stock\",NYSE
BEP,Brookfield Renewable Partners L.P. ,NYSE
BEP$A,\"Brookfield Renewable Partners L.P. 5.25% Class A Preferred Limited Partnership Units, Series 17\",NYSE
BEPC,Brookfield Renewable Corporation Class A Subordinate Voting Shares ,NYSE
BEPH,Brookfield BRP Holdings (Canada) Inc. 4.625% Perpetual Subordinated Notes,NYSE
BEPI,Brookfield BRP Holdings (Canada) Inc. 4.875% Perpetual Subordinated Notes,NYSE
BEPJ,Brookfield BRP Holdings (Canada) Inc. 7.250% Perpetual Subordinated Notes,NYSE
BERY,\"Berry Global Group, Inc. Common Stock\",NYSE
BEST,\"BEST Inc. American Depositary Shares, each representing twenty (20) Class A Ordinary Shares\",NYSE
BF.A,Brown Forman Inc Class A Common Stock,NYSE
BF.B,Brown Forman Inc Class B Common Stock,NYSE
BFAC,Battery Future Acquisition Corp. Class A Ordinary Shares,NYSE
BFAC.U,\"Battery Future Acquisition Corp. Units, each consisting of one Class A ordinary share and one-half of one redeemable warrant\",NYSE
BFAC.W,\"Battery Future Acquisition Corp. Warrants, each whole warrant exercisable for one Class A ordinary share at an exercise price of $11.50 per share\",NYSE
BFAM,Bright Horizons Family Solutions Inc. Common Stock,NYSE
BFH,\"Bread Financial Holdings, Inc. Common Stock\",NYSE
BFK,BlackRock Municipal Income Trust,NYSE
BFLY,\"Butterfly Network, Inc. Class A Common Stock\",NYSE
BFLY.W,\"Butterfly Network, Inc. Warrants\",NYSE
BFS,\"Saul Centers, Inc. Common Stock\",NYSE
BFS$D,\"Saul Centers, Inc. Depositary Shares, each representing 1/100th of a share of 6.125% Series D Cumulative Redeemable Preferred Stock\",NYSE
BFS$E,\"Saul Centers, Inc. Depositary shares, each representing a 1/100th fractional interest in a share of 6.000% Series E Cumulative Redeemable Preferred Stock\",NYSE
BFZ,BlackRock California Municipal Income Trust,NYSE
BG,Bunge Limited Common Shares,NYSE
BGB,Blackstone Strategic Credit 2027 Term Fund Common Shares of Beneficial Interest,NYSE
BGH,Barings Global Short Duration High Yield Fund Common Shares of Beneficial Interests,NYSE
BGI,Birks Group Inc. Common Stock,NYSE MKT
BGR,BlackRock Energy and Resources Trust,NYSE
BGS,\"B&G Foods, Inc. Common Stock\",NYSE
BGSF,\"BGSF, Inc. Common Stock\",NYSE
BGT,BlackRock Floating Rate Income Trust,NYSE
BGX,Blackstone Long Short Credit Income Fund Common Shares,NYSE
BGY,Blackrock Enhanced International Dividend Trust,NYSE
BH,Biglari Holdings Inc. Class B Common Stock,NYSE
BH.A,Biglari Holdings Inc. Class A Common Stock,NYSE
BHB,\"Bar Harbor Bankshares, Inc. Common Stock\",NYSE MKT
BHC,Bausch Health Companies Inc. Common Stock,NYSE
BHE,\"Benchmark Electronics, Inc. Common Stock\",NYSE
BHIL,\"Benson Hill, Inc. Common Stock\",NYSE
BHK,Blackrock Core Bond Trust,NYSE
BHLB,\"Berkshire Hills Bancorp, Inc. Common Stock\",NYSE
BHM,\"Bluerock Homes Trust, Inc. Class A Common Stock\",NYSE MKT
BHP,BHP Group Limited American Depositary Shares (Each representing two Ordinary Shares),NYSE
BHR,Braemar Hotels & Resorts Inc. Common Stock,NYSE
BHR$B,\"Braemar Hotels & Resorts Inc. 5.50% Series B Cumulative Convertible Preferred Stock, par value $0.01 per share\",NYSE
BHR$D,\"Braemar Hotels & Resorts Inc. 8.25% Series D Cumulative Preferred Stock,  par value $0.01 per share\",NYSE
BHV,BlackRock Virginia Municipal Bond Trust,NYSE
BHVN,Biohaven Ltd. Common Shares ,NYSE
BIG,\"Big Lots, Inc. Common Stock\",NYSE
BIGZ,BlackRock Innovation and Growth Term Trust Common Shares of Beneficial Interest,NYSE
BILL,\"BILL Holdings, Inc. Common Stock\",NYSE
BIO,\"Bio-Rad Laboratories, Inc. Class A Common Stock\",NYSE
BIO.B,\"Bio-Rad Laboratories, Inc. Class B  Common Stock\",NYSE
BIP,Brookfield Infrastructure Partners LP Limited Partnership Units,NYSE
BIP$A,\"Brookfield Infrastructure Partners LP 5.125% Class A Preferred Limited Partnership Units, Series 13\",NYSE
BIP$B,\"Brookfield Infrastructure Partners LP 5.000% Class A Preferred Limited Partnership Units, Series 14\",NYSE
BIPC,Brookfield Infrastructure Corporation ,NYSE
BIPH,Brookfield Infrastructure Corporation 5.000% Subordinated Notes due 2081,NYSE
BIPI,BIP Bermuda Holdings I Limited 5.125% Perpetual Subordinated Notes,NYSE
BIRK,Birkenstock Holding plc Ordinary Shares,NYSE
BIT,BlackRock Multi-Sector Income Trust Common Shares of Beneficial Interest,NYSE
BITE,Bite Acquisition Corp. Common Stock,NYSE MKT
BITE.U,\"Bite Acquisition Corp. Units, each consisting of one share of common stock and one-half of one redeemable warrant\",NYSE MKT
BITE.W,\"Bite Acquisition Corp. Warrants, each whole warrant exercisable for one share of common stock at an exercise price of $11.50\",NYSE MKT
BJ,\"BJ's Wholesale Club Holdings, Inc. Common Stock\",NYSE
BK,The Bank of New York Mellon Corporation Common Stock,NYSE
BKD,Brookdale Senior Living Inc. Common Stock,NYSE
BKDT,Brookdale Senior Living Inc. 7.00% Tangible Equity Units,NYSE
BKE,\"Buckle, Inc. (The) Common Stock\",NYSE
BKH,Black Hills Corporation Common Stock,NYSE
BKKT,\"Bakkt Holdings, Inc. Class A Common Stock\",NYSE
BKKT.W,\"Bakkt Holdings, Inc. Warrant\",NYSE
BKN,BlackRock Investment Quality Municipal Trust Inc. (The),NYSE
BKSY,BlackSky Technology Inc. Class A Common Stock,NYSE
BKSY.W,\"BlackSky Technology Inc. Redeemable Warrants, each whole Warrant exercisable for one share of Class A Common Stock\",NYSE
BKT,BlackRock Income Trust Inc. (The),NYSE
BKTI,BK Technologies Corporation Common Stock,NYSE MKT
BKU,\"BankUnited, Inc. Common Stock\",NYSE
BLCO,Bausch + Lomb Corporation Common Shares,NYSE
BLD,TopBuild Corp. Common Stock,NYSE
BLDR,\"Builders FirstSource, Inc. Common Stock\",NYSE
BLE,BlackRock Municipal Income Trust II,NYSE
BLK,\"BlackRock, Inc. Common Stock\",NYSE
BLND,\"Blend Labs, Inc. Class A Common Stock\",NYSE
BLUA,BlueRiver Acquisition Corp. Class A Ordinary Shares,NYSE MKT
BLUA.U,\"BlueRiver Acquisition Corp. Units, each consisting of one Class A ordinary share, and one-third of a redeemable Warrant to acquire one Class A ordinary shares\",NYSE MKT
BLUA.W,\"BlueRiver Acquisition Corp. Warrants, each whole warrant exercisable for one Class A Ordinary Share at an exercise price of $11.50\",NYSE MKT
BLW,Blackrock Limited Duration Income Trust,NYSE
BLX,\"Banco Latinoamericano de Comercio Exterior, S.A.\",NYSE
BMA,Banco Macro S.A.  ADR (representing Ten Class B Common Shares),NYSE
BME,Blackrock Health Sciences Trust,NYSE
BMEZ,BlackRock Health Sciences Term Trust Common Shares of Beneficial Interest,NYSE
BMI,\"Badger Meter, Inc. Common Stock\",NYSE
BML$G,\"Bank of America Corporation Depositary Shares (Each representing a 1/1200th interest in a share of Floating Rate Non-Cumulative Preferred Stock , Series 1)\",NYSE
BML$H,\"Bank of America Corporation Depositary Shares (Each representing a 1/1200th interest in a Share of Floating Rate Non-Cumulative Preferred Stock, Series 2)\",NYSE
BML$J,\"Bank of America Corporation Depositary Shares (Each representing a 1/1200th interest in a Share of Floating Rate Non-Cumulative Preferred Stock, Series 4)\",NYSE
BML$L,\"Bank of America Corporation Depositary Shares (Each representing a 1/1200th Interest in a Share of Floating Rate Non-Cumulative Preferred Stock, Series 5)\",NYSE
BMN,BlackRock 2037 Municipal Target Term Trust Common Shares of Beneficial Interest,NYSE
BMO,Bank Of Montreal Common Stock,NYSE
BMTX,\"BM Technologies, Inc. Common Stock\",NYSE MKT
BMTX.W,\"BM Technologies, Inc. Warrants\",NYSE MKT
BMY,Bristol-Myers Squibb Company Common Stock,NYSE
BN,Brookfield Corporation Class A Limited Voting Shares,NYSE
BNED,\"Barnes & Noble Education, Inc Common Stock\",NYSE
BNH,\"Brookfield Finance Inc. 4.625% Subordinated Notes due October 16, 2080\",NYSE
BNJ,Brookfield Finance Inc. 4.50% Perpetual Subordinated Notes,NYSE
BNKD,MicroSectors U.S. Big Banks Index -3X Inverse Leveraged ETNs,NYSE ARCA
BNKU,MicroSectors U.S. Big Banks Index 3X Leveraged ETNs,NYSE ARCA
BNL,\"Broadstone Net Lease, Inc. Common Stock\",NYSE
BNRE,Brookfield Reinsurance Ltd. Class A Exchangeable Limited Voting Shares,NYSE
BNRE.A,Brookfield Reinsurance Ltd. Class A-1 Exchangeable Non-Voting Shares,NYSE
BNS,Bank Nova Scotia Halifax Pfd 3 Ordinary Shares,NYSE
BNY,BlackRock New York Municipal Income Trust,NYSE
BOC,Boston Omaha Corporation Class A Common Stock,NYSE
BODI,\"The Beachbody Company, Inc. Class A Common Stock\",NYSE
BOE,Blackrock Enhanced Global Dividend Trust Common Shares of Beneficial Interest,NYSE
BOH,Bank of Hawaii Corporation Common Stock,NYSE
BOH$A,\"Bank of Hawaii Corporation Depositary Shares Each Representing a 1/40th Interest in a Share of 4.375% Fixed Rate Non-Cumulative Perpetual Preferred Stock, Series A\",NYSE
BOOT,\"Boot Barn Holdings, Inc. Common Stock\",NYSE
BORR,Borr Drilling Limited Common Shares,NYSE
BOWL,Bowlero Corp. Class A Common Stock,NYSE
BOX,\"Box, Inc. Class A Common Stock\",NYSE
BP,BP p.l.c. Common Stock,NYSE
BPT,BP Prudhoe Bay Royalty Trust Common Stock,NYSE
BQ,\"Boqii Holding Limited American Depositary Shares, representing Class A Ordinary Shares\",NYSE MKT
BR,\"Broadridge Financial Solutions, Inc. Common Stock\",NYSE
BRBR,\"BellRing Brands, Inc. Common Stock \",NYSE
BRBS,\"Blue Ridge Bankshares, Inc. Common Stock\",NYSE MKT
BRC,Brady Corporation Common Stock,NYSE
BRCC,BRC Inc. Class A Common Stock,NYSE
BRDG,Bridge Investment Group Holdings Inc. Class A Common Stock,NYSE
BRFS,BRF S.A.,NYSE
BRK.A,Berkshire Hathaway Inc. Common Stock,NYSE
BRK.B,Berkshire Hathaway Inc. New Common Stock,NYSE
BRN,\"Barnwell Industries, Inc. Common Stock\",NYSE MKT
BRO,\"Brown & Brown, Inc. Common Stock\",NYSE
BROS,Dutch Bros Inc. Class A Common Stock,NYSE
BRSP,\"BrightSpire Capital, Inc. Class A Common Stock\",NYSE
BRT,BRT Apartments Corp. (MD) Common Stock,NYSE
BRW,Saba Capital Income & Opportunities Fund SBI,NYSE
BRX,Brixmor Property Group Inc. Common Stock,NYSE
BSAC,Banco Santander - Chile ADS,NYSE
BSBR,\"Banco Santander Brasil SA American Depositary Shares, each representing one unit\",NYSE
BSIG,BrightSphere Investment Group Inc. Common Stock,NYSE
BSL,Blackstone Senior Floating Rate 2027 Term Fund Common Shares of Beneficial Interest,NYSE
BSM,\"Black Stone Minerals, L.P. Common units representing limited partner interests\",NYSE
BST,BlackRock Science and Technology Trust Common Shares of Beneficial Interest,NYSE
BSTZ,BlackRock Science and Technology Term Trust Common Shares of Beneficial Interest,NYSE
BSX,Boston Scientific Corporation Common Stock,NYSE
BTA,BlackRock Long-Term Municipal Advantage Trust Common Shares of Beneficial Interest,NYSE
BTCM,BIT Mining Limited ADS,NYSE
BTE,Baytex Energy Corp Common Shares,NYSE
BTG,B2Gold Corp Common shares (Canada),NYSE MKT
BTI,\"British American Tobacco  Industries, p.l.c. Common Stock ADR\",NYSE
BTO,John Hancock Financial Opportunities Fund Common Stock,NYSE
BTT,BlackRock Municipal 2030 Target Term Trust,NYSE
BTTR,Better Choice Company Inc. Common Stock,NYSE MKT
BTU,Peabody Energy Corporation Common Stock ,NYSE
BTZ,BlackRock Credit Allocation Income Trust,NYSE
BUD,Anheuser-Busch Inbev SA Sponsored ADR (Belgium),NYSE
BUI,\"BlackRock Utility, Infrastructure & Power Opportunities Trust\",NYSE
BUR,Burford Capital Limited Ordinary Shares,NYSE
BURL,\"Burlington Stores, Inc. Common Stock\",NYSE
BURU,\"Nuburu, Inc. Common Stock\",NYSE MKT
BV,\"BrightView Holdings, Inc. Common Stock\",NYSE
BVN,Buenaventura Mining Company Inc.,NYSE
BW,\"Babcock & Wilcox Enterprises, Inc. Common Stock\",NYSE
BW$A,\"Babcock & Wilcox Enterprises, Inc. 7.75% Series A Cumulative Perpetual Preferred Stock\",NYSE
BWA,BorgWarner Inc. Common Stock,NYSE
BWG,BrandywineGLOBAL Global Income Opportunities Fund Inc.,NYSE
BWLP,BW LPG Limited Common Shares,NYSE
BWNB,\"Babcock & Wilcox Enterprises, Inc. 6.50% Senior Notes due 2026\",NYSE
BWSN,\"Babcock & Wilcox Enterprises, Inc. 8.125% Senior Notes due 2026\",NYSE
BWXT,\"BWX Technologies, Inc. Common Stock\",NYSE
BX,Blackstone Inc. Common Stock,NYSE
BXC,Bluelinx Holdings Inc. Common Stock,NYSE
BXMT,\"Blackstone Mortgage Trust, Inc. Common Stock\",NYSE
BXMX,Nuveen S&P 500 Buy-Write Income Fund Common Shares of Beneficial Interest,NYSE
BXP,\"Boston Properties, Inc. Common Stock\",NYSE
BXSL,Blackstone Secured Lending Fund Common Shares of Beneficial Interest,NYSE
BY,\"Byline Bancorp, Inc. Common Stock\",NYSE
BYD,Boyd Gaming Corporation Common Stock,NYSE
BYM,Blackrock Municipal Income Quality Trust Common Shares of Beneficial Interest,NYSE
BYON,\"Beyond, Inc. Common Stock\",NYSE
BZH,\"Beazer Homes USA, Inc. Common Stock\",NYSE
C,\"Citigroup, Inc. Common Stock\",NYSE
C$N,Citigroup Capital XIII 7.875% Fixed rate Floating Rate trust Preferred Securities (TruPS),NYSE
CAAP,Corporacion America Airports SA Common Shares,NYSE
CABO,\"Cable One, Inc. Common Stock\",NYSE
CACI,\"CACI International, Inc. Class A Common Stock\",NYSE
CADE,Cadence Bank Common Stock,NYSE
CADE$A,Cadence Bank 5.50% Series A ,NYSE
CAE,CAE Inc. Ordinary Shares,NYSE
CAF,Morgan Stanley China A Share Fund Inc. Common Stock,NYSE
CAG,\"ConAgra Brands, Inc. Common Stock\",NYSE
CAH,\"Cardinal Health, Inc. Common Stock\",NYSE
CAL,\"Caleres, Inc. Common Stock\",NYSE
CALX,\"Calix, Inc Common Stock\",NYSE
CANF,Can-Fite Biopharma Ltd Sponsored ADR (Israel),NYSE MKT
CANG,\"Cango Inc. American Depositary Shares,  each representing two (2) Class A Ordinary Shares\",NYSE
CAPL,CrossAmerica Partners LP Common Units representing limited partner interests,NYSE
CARR,Carrier Global Corporation Common Stock ,NYSE
CARS,Cars.com Inc. Common Stock ,NYSE
CAT,\"Caterpillar, Inc. Common Stock\",NYSE
CATO,Cato Corporation (The) Class A Common Stock,NYSE
CATX,\"Perspective Therapeutics, Inc. Common Stock\",NYSE MKT
CAVA,\"CAVA Group, Inc. Common Stock\",NYSE
CB,Chubb Limited  Common Stock,NYSE
CBH,Virtus Convertible & Income 2024 Target Term Fund Common Shares of Beneficial Interest,NYSE
CBL,\"CBL & Associates Properties, Inc. Common Stock\",NYSE
CBO,CBO (Listing Market - NYSE - Networks A/E) Common Stock,NYSE
CBOE,\"Cboe Global Markets, Inc. Common Stock\",BATS
CBRE,CBRE Group Inc Common Stock Class A,NYSE
CBT,Cabot Corporation Common Stock,NYSE
CBU,\"Community Bank System, Inc. Common Stock\",NYSE
CBX,CBX (Listing Market NYSE Networks AE) Common Stock,NYSE
CBZ,\"CBIZ, Inc. Common Stock\",NYSE
CC,Chemours Company (The) Common Stock,NYSE
CCEL,\"Cryo-Cell International, Inc. Common Stock\",NYSE MKT
CCI,Crown Castle Inc. Common Stock,NYSE
CCIA,Carlyle Credit Income Fund 8.75% Series A Preferred Shares due 2028,NYSE
CCIF,Carlyle Credit Income Fund Shares of Beneficial Interest,NYSE
CCJ,Cameco Corporation Common Stock,NYSE
CCK,\"Crown Holdings, Inc.\",NYSE
CCL,Carnival Corporation Common Stock,NYSE
CCM,Concord Medical Services Holdings Limited ADS (Each represents three ordinary shares),NYSE
CCO,\"Clear Channel Outdoor Holdings, Inc. Common Stock\",NYSE
CCRD,CoreCard Corporation Common Stock,NYSE
CCS,\"Century Communities, Inc. Common Stock\",NYSE
CCU,\"Compania Cervecerias Unidas, S.A. Common Stock\",NYSE
CCZ,Comcast Holdings ZONES,NYSE
CDE,\"Coeur Mining, Inc. Common Stock\",NYSE
CDLR,Cadeler A/S American Depositary Share (each representing four (4) Ordinary Shares),NYSE
CDP,COPT Defense Properties Common Shares of Beneficial Interest,NYSE
CDR$B,\"Cedar Realty Trust, Inc. 7.25% Series B Cumulative Redeemable Preferred Stock\",NYSE
CDR$C,\"Cedar Realty Trust, Inc. 6.50% Series C Cumulative Redeemable Preferred Stock\",NYSE
CDRE,\"Cadre Holdings, Inc. Common Stock\",NYSE
CE,Celanese Corporation Common Stock,NYSE
CEE,\"The Central and Eastern Europe Fund, Inc. (The) Common Stock\",NYSE
CEI,\"Camber Energy, Inc. Common Stock\",NYSE MKT
CEIX,CONSOL Energy Inc. Common Stock ,NYSE
CELG.R,Bristol-Myers Squibb Company Celegne Contingent Value Rights,NYSE
CEM,ClearBridge MLP and Midstream Fund Inc. Common Stock,NYSE
CEPU,Central Puerto S.A. American Depositary Shares (each represents ten Common Shares),NYSE
CET,Central Securities Corporation Common Stock,NYSE MKT
CEV,Eaton Vance California Municipal Income Trust Shares of Beneficial Interest,NYSE MKT
CF,\"CF Industries Holdings, Inc. Common Stock\",NYSE
CFG,\"Citizens Financial Group, Inc. Common Stock\",NYSE
CFG$D,\"Citizens Financial Group, Inc. Depositary Shares, each representing a 1/40th Interest in a Share of 6.350% Fixed-to-Floating Rate Non-Cumulative Perpetual Preferred Stock, Series D\",NYSE
CFG$E,\"Citizens Financial Group, Inc. Depositary Shares Each Representing 1/40th Interest in a Share of 5.000% Fixed-Rate Non-Cumulative Perpetual Preferred Stock, Series E\",NYSE
CFR,\"Cullen/Frost Bankers, Inc. Common Stock\",NYSE
CFR$B,\"Cullen/Frost Bankers, Inc. Depositary Shares, each representing a 1/40th ownership interest in a share of 4.450% non-cumulative perpetual preferred stock, Series B\",NYSE
CGA,\"China Green Agriculture, Inc. Common Stock\",NYSE
CGAU,Centerra Gold Inc. Common Shares,NYSE
CHAA,Catcha Investment Corp. Class A Ordinary Shares,NYSE MKT
CHCT,Community Healthcare Trust Incorporated Common Stock,NYSE
CHD,\"Church & Dwight Company, Inc. Common Stock\",NYSE
CHE,Chemed Corp,NYSE
CHGG,\"Chegg, Inc. Common Stock\",NYSE
CHH,\"Choice Hotels International, Inc. Common Stock\",NYSE
CHMI,Cherry Hill Mortgage Investment Corporation Common Stock,NYSE
CHMI$A,Cherry Hill Mortgage Investment Corporation 8.20% Series A Cumulative Redeemable Preferred Stock,NYSE
CHMI$B,Cherry Hill Mortgage Investment Corporation 8.250% Series B Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock,NYSE
CHN,\"China Fund, Inc. (The) Common Stock\",NYSE
CHPT,\"ChargePoint Holdings, Inc. Common Stock\",NYSE
CHRO,Chromocell Therapeutics Corporation Common Stock,NYSE MKT
CHT,\"Chunghwa Telecom Co., Ltd.\",NYSE
CHWY,\"Chewy, Inc. Class A Common Stock\",NYSE
CI,The Cigna Group Common Stock,NYSE
CIA,\"Citizens, Inc. Class A Common Stock ($1.00 Par)\",NYSE
CIB,BanColombia S.A. Common Stock,NYSE
CIEN,Ciena Corporation Common Stock,NYSE
CIF,MFS Intermediate High Income Fund Common Stock,NYSE
CIG,Comp En De Mn Cemig ADS American Depositary Shares,NYSE
CIG.C,Comp En De Mn Cemig ADS American Depositary Receipts,NYSE
CII,\"Blackrock Capital and Income Fund, Inc.\",NYSE
CIK,\"Credit Suisse Asset Management Income Fund, Inc. Common Stock\",NYSE MKT
CIM,Chimera Investment Corporation Common Stock,NYSE
CIM$A,Chimera Investment Corporation 8.00% Series A Cumulative Redeemable Preferred Stock,NYSE
CIM$B,Chimera Investment Corporation 8.00% Series B Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock,NYSE
CIM$C,Chimera Investment Corporation 7.75% Series C Fixed-to-Floating Rate  Cumulative Redeemable  Preferred Stock,NYSE
CIM$D,Chimera Investment Corporation 8.00% Series D Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock,NYSE
CINT,CI&T Inc Class A Common Shares,NYSE
CIO,\"City Office REIT, Inc. Common Stock\",NYSE
CIO$A,\"City Office REIT, Inc. 6.625% Series A Cumulative Redeemable Preferred Stock\",NYSE
CION,CION Investment Corporation Common Stock,NYSE
CIVI,\"Civitas Resources, Inc. Common Stock\",NYSE
CIX,CompX International Inc. Common Stock,NYSE MKT
CKX,\"CKX Lands, Inc. Common Stock\",NYSE MKT
CL,Colgate-Palmolive Company Common Stock,NYSE
CLB,Core Laboratories Inc. Common Stock,NYSE
CLBR,Colombier Acquisition Corp. II Class A Ordinary Shares,NYSE
CLBR.U,\"Colombier Acquisition Corp. II Units, each consisting of one Class A ordinary share and one-third of one redeemable warrant\",NYSE
CLBR.W,\"Colombier Acquisition Corp. II Warrants, each whole warrant exercisable for one Class A Ordinary Share at an exercise price of $11.50 per share\",NYSE
CLCO,Cool Company Ltd. Common Shares,NYSE
CLDI,\"Calidi Biotherapeutics, Inc. Common Stock\",NYSE MKT
CLDI.W,\"Calidi Biotherapeutics, Inc. Redeemable Warrants, each whole warrant exercisable for one share of Class A common stock at an exercise price of $11.50 per share\",NYSE MKT
CLDT,Chatham Lodging Trust (REIT) Common Shares of Beneficial Interest,NYSE
CLDT$A,Chatham Lodging Trust (REIT) 6.625% Series A Cumulative Redeemable Preferred Shares of Beneficial Interest,NYSE
CLF,Cleveland-Cliffs Inc. Common Stock,NYSE
CLH,\"Clean Harbors, Inc. Common Stock\",NYSE
CLM,\"Cornerstone Strategic Value Fund, Inc. New Common Stock\",NYSE MKT
CLPR,Clipper Realty Inc. Common Stock,NYSE
CLS,\"Celestica, Inc. Common Stock\",NYSE
CLVT,Clarivate Plc Ordinary Shares,NYSE
CLVT$A,Clarivate Plc 5.25% Series A Mandatory Convertible Preferred Shares,NYSE
CLW,Clearwater Paper Corporation Common Stock,NYSE
CLX,Clorox Company (The) Common Stock,NYSE
CM,Canadian Imperial Bank of Commerce Common Stock,NYSE
CMA,Comerica Incorporated Common Stock,NYSE
CMC,Commercial Metals Company Common Stock,NYSE
CMCL,Caledonia Mining Corporation Plc Common Shares,NYSE MKT
CMCM,\"Cheetah Mobile Inc. American Depositary Shares, each representing fifty (50) Class A Ordinary Shares\",NYSE
CMG,\"Chipotle Mexican Grill, Inc. Common Stock\",NYSE
CMI,Cummins Inc. Common Stock,NYSE
CMP,Compass Minerals Intl Inc Common Stock,NYSE
CMRE,Costamare Inc. Common Stock $0.0001 par value,NYSE
CMRE$B,Costamare Inc. Perpetual Preferred Stock Series B (Marshall Islands),NYSE
CMRE$C,Costamare Inc. Perpetual Preferred Series C (Marshall Islands),NYSE
CMRE$D,Costamare Inc. 8.75% Series D Cumulative Redeemable Perpetual Preferred Stock,NYSE
CMRE$E,\"Costamare Inc. 8.875% Series E Cumulative Redeemable Perpetual Preferred Stock, par value $0.0001\",NYSE
CMS,CMS Energy Corporation Common Stock,NYSE
CMS$B,CMS Energy Corporation Preferred Stock,NYSE
CMS$C,\"CMS Energy Corporation Depositary Shares, each representing a 1/1,000th interest in a share of 4.200% Cumulative Redeemable Perpetual Preferred Stock, Series C\",NYSE
CMSA,CMS Energy Corporation 5.625% Junior Subordinated Notes due 2078,NYSE
CMSC,CMS Energy Corporation 5.875% Junior Subordinated Notes due 2078,NYSE
CMSD,CMS Energy Corporation 5.875% Junior Subordinated Notes due 2079,NYSE
CMT,Core Molding Technologies Inc Common Stock,NYSE MKT
CMTG,\"Claros Mortgage Trust, Inc. Common Stock\",NYSE
CMU,MFS Municipal Income Trust Common Stock,NYSE
CNA,CNA Financial Corporation Common Stock,NYSE
CNC,Centene Corporation Common Stock,NYSE
CNDA,Concord Acquisition Corp II Class A Common Stock,NYSE
CNDA.U,Concord Acquisition Corp II Units,NYSE
CNDA.W,\"Concord Acquisition Corp II Warrants, each whole warrant exercisable for one share of Class A Common Stock at an exercise price of $11.50\",NYSE
CNF,\"CNFinance Holdings Limited American Depositary Shares, each representing  twenty (20) Ordinary Shares\",NYSE
CNH,CNH Industrial N.V. Common Shares,NYSE
CNI,Canadian National Railway Company Common Stock,NYSE
CNK,\"Cinemark Holdings Inc Cinemark Holdings, Inc. Common Stock\",NYSE
CNM,\"Core & Main, Inc. Class A Common Stock\",NYSE
CNMD,CONMED Corporation Common Stock,NYSE
CNNE,\"Cannae Holdings, Inc. Common Stock\",NYSE
CNO,\"CNO Financial Group, Inc. Common Stock\",NYSE
CNO$A,\"CNO Financial Group, Inc. 5.125% Subordinated Debentures due 2060\",NYSE
CNP,\"CenterPoint Energy, Inc (Holding Co) Common Stock\",NYSE
CNQ,Canadian Natural Resources Limited Common Stock,NYSE
CNS,Cohen & Steers Inc Common Stock,NYSE
CNX,CNX Resources Corporation Common Stock,NYSE
CODI,D/B/A Compass Diversified Holdings Shares of Beneficial Interest,NYSE
CODI$A,Compass Diversified Holdings 7.250% Series A Preferred Shares representing beneficial interest in Compass Diversified Holdings,NYSE
CODI$B,Compass Diversified Holdings 7.875% Series B Fixed-to-Floating Rate Cumulative Preferred Shares representing beneficial interests in Compass Diversified Holdings,NYSE
CODI$C,Compass Diversified Holdings 7.875% Series C Cumulative Preferred Shares,NYSE
COE,\"51Talk Online Education Group American depositary shares, each representing 60 Class A ordinary shares\",NYSE MKT
COF,Capital One Financial Corporation Common Stock,NYSE
COF$I,\"Capital One Financial Corporation Depositary shares each representing a 1/40th interest in a share of Fixed Rate Non-Cumulative Perpetual Preferred Stock, Series I of the Issuer\",NYSE
COF$J,\"Capital One Financial Corporation Depositary Shares, Each Representing a 1/40th Interest in a Share of Fixed Rate Non- Cumulative Perpetual Preferred Stock, Series J\",NYSE
COF$K,\"Capital One Financial Corporation Depositary Shares, Each Representing a 1/40th Ownership Interest in a Share of Fixed Rate Non-Cumulative Perpetual Preferred Stock, Series K\",NYSE
COF$L,\"Capital One Financial Corporation Depositary Shares, Each Representing a 1/40th Interest in a Share of Fixed Rate Non-Cumulative Perpetual Preferred Stock, Series L\",NYSE
COF$N,\"Capital One Financial Corporation Depositary Shares, Each Representing a 1/40th Ownership Interest in a Share of Fixed Rate Non-Cumulative Perpetual Preferred Stock, Series N\",NYSE
COHN,Cohen & Company Inc.,NYSE MKT
COHR,Coherent Corp. Common Stock,NYSE
COLD,\"Americold Realty Trust, Inc. Common Stock\",NYSE
COMP,\"Compass, Inc. Class A Common Stock\",NYSE
COOK,\"Traeger, Inc. Common Stock\",NYSE
COP,ConocoPhillips Common Stock,NYSE
COR,\"Cencora, Inc. Common Stock\",NYSE
COTY,Coty Inc. Class A Common Stock,NYSE
COUR,\"Coursera, Inc. Common Stock\",NYSE
CP,Canadian Pacific Kansas City Limited Common Shares,NYSE
CPA,\"Copa Holdings, S.A. Class A Common Stock\",NYSE
CPAC,Cementos Pacasmayo S.A.A. American Depositary Shares (Each representing five Common Shares),NYSE
CPAY,\"Corpay, Inc. Common Stock\",NYSE
CPB,Campbell Soup Company Common Stock,NYSE
CPF,Central Pacific Financial Corp New,NYSE
CPHI,\"China Pharma Holdings, Inc. Common Stock\",NYSE MKT
CPK,Chesapeake Utilities Corporation Common Stock,NYSE
CPNG,\"Coupang, Inc. Class A Common Stock\",NYSE
CPRI,Capri Holdings Limited Ordinary Shares,NYSE
CPS,Cooper-Standard Holdings Inc. Common Stock,NYSE
CPT,Camden Property Trust Common Stock,NYSE
CQP,\"Cheniere Energy Partners, LP Common Units\",NYSE
CR,Crane Company Common Stock,NYSE
CRBG,Corebridge Financial Inc. Common Stock,NYSE
CRC,California Resources Corporation Common Stock,NYSE
CRD.A,Crawford & Company Common Stock,NYSE
CRD.B,Crawford & Company Common Stock,NYSE
CRF,\"Cornerstone Total Return Fund, Inc. (The) Common Stock\",NYSE MKT
CRGY,Crescent Energy Company Class A Common Stock,NYSE
CRH,CRH PLC Ordinary Shares,NYSE
CRI,\"Carter's, Inc. Common Stock\",NYSE
CRK,\"Comstock Resources, Inc. Common Stock\",NYSE
CRL,\"Charles River Laboratories International, Inc. Common Stock\",NYSE
CRM,\"Salesforce, Inc. Common Stock\",NYSE
CRS,Carpenter Technology Corporation Common Stock,NYSE
CRT,Cross Timbers Royalty Trust Common Stock,NYSE
CSAN,Cosan S.A. ADS,NYSE
CSL,Carlisle Companies Incorporated Common Stock,NYSE
CSR,D/B/A Centerspace Common Stock,NYSE
CSR$C,D/B/A Centerspace 6.625% Series C ,NYSE
CSTM,Constellium SE Ordinary Shares (France),NYSE
CSV,\"Carriage Services, Inc. Common Stock\",NYSE
CTA$A,\"EIDP, Inc. Preferred Stock $3.50 Series\",NYSE
CTA$B,\"EIDP, Inc. Preferred Stock $4.50 Series\",NYSE
CTBB,Qwest Corporation 6.5% Notes due 2056,NYSE
CTDD,Qwest Corporation 6.75% Notes due 2057,NYSE
CTEST,NYSE Test One Common Stock,NYSE
CTEST.E,NYSE Test One Common Stock,NYSE
CTEST.G,NYSE Test One Common Stock,NYSE
CTEST.L,NYSE Test One Common Stock,NYSE
CTEST.O,NYSE Test One Common Stock,NYSE
CTEST.S,NYSE Test Six Common Stock,NYSE
CTEST.V,NYSE Test One Common Stock,NYSE
CTGO,\"Contango ORE, Inc. Common Stock\",NYSE MKT
CTLT,\"Catalent, Inc. Common Stock\",NYSE
CTM,\"Castellum, Inc. Common Stock\",NYSE MKT
CTO,\"CTO Realty Growth, Inc. Common Stock\",NYSE
CTO$A,\"CTO Realty Growth, Inc. 6.375% Series A Cumulative Redeemable Preferred Stock\",NYSE
CTOS,\"Custom Truck One Source, Inc. Common Stock\",NYSE
CTR,ClearBridge MLP and Midstream Total Return Fund Inc. Common Stock,NYSE
CTRA,Coterra Energy Inc. Common Stock,NYSE
CTRE,\"CareTrust REIT, Inc. Common Stock\",NYSE
CTRI,\"Centuri Holdings, Inc. Common Stock\",NYSE
CTS,CTS Corporation Common Stock,NYSE
CTV,Innovid Corp. Common Stock,NYSE
CTV.W,\"Innovid Corp. Warrants, each whole warrant exercisable for one share of Common Stock at an exercise price of $11.50 per share\",NYSE
CTVA,\"Corteva, Inc. Common Stock \",NYSE
CUBB,\"Customers Bancorp, Inc 5.375% Subordinated Notes Due 2034\",NYSE
CUBE,CubeSmart Common Shares,NYSE
CUBI,\"Customers Bancorp, Inc Common Stock\",NYSE
CUBI$E,\"Customers Bancorp, Inc Fixed-to-Floating Rate Non-Cumulative Perpetual Preferred Stock, Series E\",NYSE
CUBI$F,\"Customers Bancorp, Inc Fixed-to-Floating Rate Non-Cumulative Perpetual Preferred Stock, Series F\",NYSE
CUK,Carnival Plc ADS ADS,NYSE
CULP,\"Culp, Inc. Common Stock\",NYSE
CURV,Torrid Holdings Inc. Common Stock,NYSE
CUZ,Cousins Properties Incorporated Common Stock,NYSE
CVE,Cenovus Energy Inc Common Stock,NYSE
CVE.W,Cenovus Energy Inc Warrants (each warrant entitles the holder to purchase one common share at an exercise price of C$6.54 per share),NYSE
CVEO,Civeo Corporation (Canada) Common Shares,NYSE
CVI,CVR Energy Inc. Common Stock,NYSE
CVM,Cel-Sci Corporation Common Stock,NYSE MKT
CVNA,Carvana Co. Class A Common Stock,NYSE
CVR,Chicago Rivet & Machine Co. Common Stock,NYSE MKT
CVS,CVS Health Corporation Common Stock,NYSE
CVU,\"CPI Aerostructures, Inc. Common Stock\",NYSE MKT
CVX,Chevron Corporation Common Stock,NYSE
CW,Curtiss-Wright Corporation Common Stock,NYSE
CWAN,\"Clearwater Analytics Holdings, Inc. Class A Common Stock\",NYSE
CWEN,\"Clearway Energy, Inc. Class C Common Stock\",NYSE
CWEN.A,\"Clearway Energy, Inc. Class A Common Stock\",NYSE
CWH,\"Camping World Holdings, Inc. Class A Common Stock\",NYSE
CWK,Cushman & Wakefield plc Ordinary Shares,NYSE
CWT,California Water Service Group Common Stock,NYSE
CX,\"Cemex, S.A.B. de C.V. Sponsored ADR\",NYSE
CXE,MFS High Income Municipal Trust Common Stock,NYSE
CXH,MFS Investment Grade Municipal Trust Common Stock,NYSE
CXM,\"Sprinklr, Inc. Class A Common Stock\",NYSE
CXT,\"Crane NXT, Co. Common Stock\",NYSE
CXW,\"CoreCivic, Inc. Common Stock\",NYSE
CYBN,Cybin Inc. Common Shares,NYSE MKT
CYD,China Yuchai International Limited Common Stock,NYSE
CYH,\"Community Health Systems, Inc. Common Stock\",NYSE
D,\"Dominion Energy, Inc. Common Stock\",NYSE
DAC,Danaos Corporation Common Stock,NYSE
DAL,\"Delta Air Lines, Inc. Common Stock\",NYSE
DAN,Dana Incorporated Common Stock ,NYSE
DAO,\"Youdao, Inc. American Depositary Shares, each representing one Class A Ordinary Share\",NYSE
DAR,Darling Ingredients Inc. Common Stock,NYSE
DAVA,Endava plc American Depositary Shares (each representing one Class A Ordinary Share),NYSE
DAY,\"Dayforce, Inc. Common Stock\",NYSE
DB,Deutsche Bank AG Common Stock,NYSE
DBD,Diebold Nixdorf Incorporated Common stock,NYSE
DBI,Designer Brands Inc. Class A Common Stock,NYSE
DBL,DoubleLine Opportunistic Credit Fund Common Shares of Beneficial Interest,NYSE
DBRG,\"DigitalBridge Group, Inc.\",NYSE
DBRG$H,\"DigitalBridge Group, Inc. 7.125% Series H \",NYSE
DBRG$I,\"DigitalBridge Group, Inc. 7.15% Series I \",NYSE
DBRG$J,\"DigitalBridge Group, Inc. 7.125% Series J \",NYSE
DC,Dakota Gold Corp. Common Stock,NYSE MKT
DC.W,\"Dakota Gold Corp. Warrants, each warrant exercisable for one Common Share at an exercise price of $2.08\",NYSE MKT
DCF,\"BNY Mellon Alcentra Global Credit Income 2024 Target Term Fund, Inc. Common Stock\",NYSE
DCI,\"Donaldson Company, Inc. Common Stock\",NYSE
DCO,Ducommun Incorporated Common Stock,NYSE
DD,\"DuPont de Nemours, Inc. Common Stock\",NYSE
DDC,DDC Enterprise Limited Class A Ordinary Shares,NYSE MKT
DDD,3D Systems Corporation Common Stock,NYSE
DDL,Dingdong (Cayman) Limited American Depositary Shares (each two representing three Ordinary Shares),NYSE
DDS,\"Dillard's, Inc. Common Stock\",NYSE
DDT,Dillard's Capital Trust I,NYSE
DE,Deere & Company Common Stock,NYSE
DEA,\"Easterly Government Properties, Inc. Common Stock\",NYSE
DEC,Diversified Energy Company plc Ordinary Shares,NYSE
DECK,Deckers Outdoor Corporation Common Stock,NYSE
DEI,\"Douglas Emmett, Inc. Common Stock\",NYSE
DELL,Dell Technologies Inc. Class C Common Stock ,NYSE
DEO,Diageo plc Common Stock,NYSE
DESP,\"Despegar.com, Corp. Ordinary Shares\",NYSE
DFH,\"Dream Finders Homes, Inc. Class A Common Stock\",NYSE
DFIN,\"Donnelley Financial Solutions, Inc. Common Stock \",NYSE
DFP,Flaherty & Crumrine Dynamic Preferred and Income Fund Inc. Common Stock,NYSE
DFS,Discover Financial Services Common Stock,NYSE
DG,Dollar General Corporation Common Stock,NYSE
DGP,\"DB Gold Double Long ETN due February 15, 2038\",NYSE ARCA
DGX,Quest Diagnostics Incorporated Common Stock,NYSE
DGZ,\"DB Gold Short ETN due February 15, 2038\",NYSE ARCA
DHF,BNY Mellon High Yield Strategies Fund Common Stock,NYSE
DHI,\"D.R. Horton, Inc. Common Stock\",NYSE
DHR,Danaher Corporation Common Stock,NYSE
DHT,\"DHT Holdings, Inc.\",NYSE
DHX,\"DHI Group, Inc. Common Stock\",NYSE
DHY,Credit Suisse High Yield Bond Fund Common Stock,NYSE MKT
DIAX,Nuveen Dow 30SM Dynamic Overwrite Fund Common Shares of Beneficial Interest,NYSE
DIN,\"Dine Brands Global, Inc. Common Stock\",NYSE
DINO,HF Sinclair Corporation Common Stock,NYSE
DIS,Walt Disney Company (The) Common Stock,NYSE
DIT,AMCON Distributing Company Common Stock,NYSE MKT
DJP,iPath Bloomberg Commodity Index Total Return ETN,NYSE ARCA
DK,\"Delek US Holdings, Inc. Common Stock\",NYSE
DKL,\"Delek Logistics Partners, L.P. Common Units representing Limited Partner Interests\",NYSE
DKS,Dick's Sporting Goods Inc Common Stock,NYSE
DLA,\"Delta Apparel, Inc. Common Stock\",NYSE MKT
DLB,Dolby Laboratories Common Stock,NYSE
DLNG,Dynagas LNG Partners LP Common Units,NYSE
DLNG$A,Dynagas LNG Partners LP 9.00% Series A Cumulative Redeemable Preferred Units,NYSE
DLNG$B,\"Dynagas LNG Partners LP 8.75% Series B Fixed to Floating Rate Cumulative Redeemable Perpetual Preferred Units, liquidation preference $25.00 per Uni\",NYSE
DLR,\"Digital Realty Trust, Inc. Common Stock\",NYSE
DLR$J,\"Digital Realty Trust, Inc. 5.250% Series J Cumulative Redeemable Preferred Stock\",NYSE
DLR$K,\"Digital Realty Trust, Inc. 5.850% Series K Cumulative Redeemable Preferred Stock, par value $0.01 per share\",NYSE
DLR$L,\"Digital Realty Trust, Inc. 5.200% Series L Cumulative Redeemable Preferred Stock\",NYSE
DLX,Deluxe Corporation Common Stock,NYSE
DLY,DoubleLine Yield Opportunities Fund Common Shares of Beneficial Interest,NYSE
DM,\"Desktop Metal, Inc. Class A Common Stock\",NYSE
DMA,Destra Multi-Alternative Fund Common Stock,NYSE
DMB,\"BNY Mellon Municipal Bond Infrastructure Fund, Inc. Common Stock\",NYSE
DMF,BNY Mellon Municipal Income Inc. Common Stock,NYSE MKT
DMO,Western Asset Mortgage Opportunity Fund Inc. Common Stock,NYSE
DMYY,\"dMY Squared Technology Group, Inc. Class A Common Stock\",NYSE MKT
DMYY.U,\"dMY Squared Technology Group, Inc. Units, each consisting of one share of Class A common stock and one-half of one redeemable warrant\",NYSE MKT
DMYY.W,\"dMY Squared Technology Group, Inc. Redeemable Warrants, each whole warrant exercisable for one share of Class A common stock at an exercise price of $11.50 per share\",NYSE MKT
DNA,\"Ginkgo Bioworks Holdings, Inc. Class A Common Stock\",NYSE
DNA.W,\"Ginkgo Bioworks Holdings, Inc. Warrant\",NYSE
DNB,\"Dun & Bradstreet Holdings, Inc. Common Stock\",NYSE
DNMR,\"Danimer Scientific, Inc. Common Stock\",NYSE
DNN,Denison Mines Corp Ordinary Shares (Canada),NYSE MKT
DNOW,DNOW Inc. Common Stock,NYSE
DNP,\"DNP Select Income Fund, Inc. Common Stock\",NYSE
DO,\"Diamond Offshore Drilling, Inc. Common Stock\",NYSE
DOC,\"Healthpeak Properties, Inc. Common Stock\",NYSE
DOCN,\"DigitalOcean Holdings, Inc. Common Stock\",NYSE
DOCS,\"Doximity, Inc. Class A Common Stock\",NYSE
DOLE,Dole plc Ordinary Shares,NYSE
DOMA,\"Doma Holdings, Inc. Common Stock\",NYSE
DOUG,Douglas Elliman Inc. Common Stock,NYSE
DOV,Dover Corporation Common Stock,NYSE
DOW,Dow Inc. Common Stock ,NYSE
DPG,Duff & Phelps Utility and Infrastructure Fund Inc.,NYSE
DPSI,\"DecisionPoint Systems, Inc. Common Stock\",NYSE MKT
DPZ,Domino's Pizza Inc Common Stock,NYSE
DQ,\"DAQO New Energy Corp. American Depositary Shares, each representing five ordinary shares\",NYSE
DRD,DRDGOLD Limited American Depositary Shares,NYSE
DRH,Diamondrock Hospitality Company Common Stock,NYSE
DRH$A,Diamondrock Hospitality Company 8.250% Series A Cumulative Redeemable Preferred Stock,NYSE
DRI,\"Darden Restaurants, Inc. Common Stock\",NYSE
DRQ,\"Dril-Quip, Inc. Common Stock\",NYSE
DSL,DoubleLine Income Solutions Fund Common Shares of Beneficial Interests,NYSE
DSM,\"BNY Mellon Strategic Municipal Bond Fund, Inc. Common Stock\",NYSE
DSS,\"DSS, Inc. Common Stock\",NYSE MKT
DSU,\"Blackrock Debt Strategies Fund, Inc. Common Stock\",NYSE
DSX,Diana Shipping inc. common stock,NYSE
DSX$B,Diana Shipping Inc. Perpetual Preferred Shares Series B (Marshall Islands),NYSE
DSX.W,Diana Shipping inc. Warrants,NYSE
DT,\"Dynatrace, Inc. Common Stock\",NYSE
DTB,DTE Energy Company 2020 Series G 4.375% Junior Subordinated Debentures due 2080,NYSE
DTC,\"Solo Brands, Inc. Class A Common Stock\",NYSE
DTE,DTE Energy Company Common Stock,NYSE
DTF,DTF Tax-Free Income 2028 Term Fund Inc. Common Stock,NYSE
DTG,DTE Energy Company 2021 Series E 4.375% Junior Subordinated Debentures,NYSE
DTM,\"DT Midstream, Inc. Common Stock \",NYSE
DTW,DTE Energy Company 2017 Series E 5.25% Junior Subordinated Debentures due 2077,NYSE
DUK,Duke Energy Corporation (Holding Company) Common Stock,NYSE
DUK$A,\"Duke Energy Corporation Depositary Shares, each representing a 1/1,000th interest in a share of 5.75% Series A Cumulative Redeemable Perpetual Preferred Stock\",NYSE
DUKB,Duke Energy Corporation 5.625% Junior Subordinated Debentures due 2078,NYSE
DV,\"DoubleVerify Holdings, Inc. Common Stock\",NYSE
DVA,DaVita Inc. Common Stock,NYSE
DVN,Devon Energy Corporation Common Stock,NYSE
DX,\"Dynex Capital, Inc. Common Stock\",NYSE
DX$C,\"Dynex Capital, Inc. 6.900% Series C Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock\",NYSE
DXC,DXC Technology Company Common Stock ,NYSE
DXF,Dunxin Financial Holdings Limited American Depositary Shares,NYSE MKT
DXYZ,Destiny Tech100 Inc. Common Stock,NYSE
DY,\"Dycom Industries, Inc. Common Stock\",NYSE
DZZ,\"DB Gold Double Short ETN due February 15, 2038\",NYSE ARCA
E,ENI S.p.A. Common Stock,NYSE
EAD,Allspring Income Opportunities Fund Common Shares,NYSE MKT
EAF,GrafTech International Ltd. Common Stock,NYSE
EAI,\"Entergy Arkansas, LLC First Mortgage Bonds, 4.875% Series Due September 1, 2066\",NYSE
EARN,Ellington Credit Company Common Shares of Beneficial Interest,NYSE
EAT,\"Brinker International, Inc. Common Stock\",NYSE
EB,\"Eventbrite, Inc. Class A Common Stock\",NYSE
EBF,\"Ennis, Inc. Common Stock\",NYSE
EBR,Centrais Electricas Brasileiras S A American Depositary Shares (Each representing one Common Share),NYSE
EBR.B,Centrais Electricas Brasileiras S.A.- Eletrobr?!s American Depositary Shares (Each representing one Preferred Share),NYSE
EBS,\"Emergent Biosolutions, Inc. Common Stock\",NYSE
EC,Ecopetrol S.A. American Depositary Shares,NYSE
ECAT,BlackRock ESG Capital Allocation Term Trust Common Shares of Beneficial Interest,NYSE
ECC,Eagle Point Credit Company Inc. Common Stock,NYSE
ECC$D,Eagle Point Credit Company Inc. 6.75% Series D Preferred Stock,NYSE
ECCC,Eagle Point Credit Company Inc. 6.50% Series C Term Preferred Stock due 2031,NYSE
ECCF,Eagle Point Credit Company Inc. 8.00% Series F Term Preferred Stock due 2029,NYSE
ECCV,Eagle Point Credit Company Inc. 5.375% Notes due 2029,NYSE
ECCW,Eagle Point Credit Company Inc. 6.75% Notes due 2031,NYSE
ECCX,Eagle Point Credit Company Inc. 6.6875% Notes due 2028,NYSE
ECF,Ellsworth Growth and Income Fund Ltd.,NYSE MKT
ECF$A,Ellsworth Growth and Income Fund Ltd. 5.25% Series A Cumulative Preferred Shares (Liquidation Preference $25.00 per share),NYSE MKT
ECL,Ecolab Inc. Common Stock,NYSE
ECO,Okeanis Eco Tankers Corp. Common Stock,NYSE
ECVT,Ecovyst Inc. Common Stock,NYSE
ED,\"Consolidated Edison, Inc. Common Stock\",NYSE
EDD,\"Morgan Stanley Emerging Markets Domestic Debt Fund, Inc. Common Stock\",NYSE
EDF,Virtus Stone Harbor Emerging Markets Income Fund Common Shares of Beneficial Interest,NYSE
EDN,Empresa Distribuidora Y Comercializadora Norte S.A. (Edenor) American Depositary Shares,NYSE
EDR,\"Endeavor Group Holdings, Inc. Class A Common Stock\",NYSE
EDU,\"New Oriental Education & Technology Group, Inc. Sponsored ADR representing 10 Ordinary Share (Cayman Islands)\",NYSE
EE,\"Excelerate Energy, Inc. Class A Common Stock\",NYSE
EEA,\"The European Equity Fund, Inc. Common Stock\",NYSE
EEX,\"Emerald Holding, Inc. Common Stock\",NYSE
EFC,Ellington Financial Inc. Common Stock ,NYSE
EFC$A,Ellington Financial Inc. 6.750% Series A Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock,NYSE
EFC$B,Ellington Financial Inc. 6.250% Series B Fixed-Rate Reset Cumulative Redeemable Preferred Stock,NYSE
EFC$C,Ellington Financial Inc. 8.625% Series C Fixed-Rate Reset Cumulative Redeemable Preferred Stock,NYSE
EFC$D,Ellington Financial Inc. 7.00% Series D Cumulative Perpetual Redeemable Preferred Stock,NYSE
EFC$E,Ellington Financial Inc. 8.250% Series E Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock,NYSE
EFR,Eaton Vance Senior Floating-Rate Fund Common Shares of Beneficial Interest,NYSE
EFSH,1847 Holdings LLC Common Shares,NYSE MKT
EFT,Eaton Vance Floating Rate Income Trust Common Shares of Beneficial Interest,NYSE
EFX,\"Equifax, Inc. Common Stock\",NYSE
EFXT,Enerflex Ltd Common Shares,NYSE
EG,\"Everest Group, Ltd. Common Stock\",NYSE
EGF,\"Blackrock Enhanced Government Fund, Inc. Common Stock\",NYSE
EGO,Eldorado Gold Corporation Ordinary Shares,NYSE
EGP,\"EastGroup Properties, Inc. Common Stock\",NYSE
EGY,\"VAALCO Energy, Inc.  Common Stock\",NYSE
EHAB,\"Enhabit, Inc. Common Stock\",NYSE
EHC,Encompass Health Corporation Common Stock,NYSE
EHI,Western Asset Global High Income Fund Inc Common Stock,NYSE
EIC,Eagle Point Income Company Inc. Common Stock,NYSE
EICA,Eagle Point Income Company Inc. 5.00% Series A Term Preferred Stock due 2026,NYSE
EICB,Eagle Point Income Company Inc. 7.75% Series B Term Preferred Stock Due 2028,NYSE
EICC,Eagle Point Income Company Inc. 8.00% Series C Term Preferred Stock due 2029,NYSE
EIG,Employers Holdings Inc Common Stock,NYSE
EIM,\"Eaton Vance Municipal Bond Fund Common Shares of Beneficial Interest, $.01 par value\",NYSE MKT
EIX,Edison International Common Stock,NYSE
EL,\"Estee Lauder Companies, Inc. (The) Common Stock\",NYSE
ELA,Envela Corporation Common Stock,NYSE MKT
ELAN,Elanco Animal Health Incorporated Common Stock,NYSE
ELC,\"Entergy Louisiana, Inc. Collateral Trust Mortgage Bonds, 4.875 % Series due September 1, 2066\",NYSE
ELF,\"e.l.f. Beauty, Inc. Common Stock\",NYSE
ELLO,Ellomay Capital Ltd Ordinary Shares (Israel),NYSE MKT
ELMD,\"Electromed, Inc. Common Stock\",NYSE MKT
ELME,Elme Communities Common Stock,NYSE
ELP,Companhia Paranaense de Energia (COPEL) American Depositary Shares (each representing one Unit consisting one Common Share and four non-voting Class B Preferred Shares),NYSE
ELPC,Companhia Paranaense de Energia (COPEL) American Depositary Shares (each representing four (4) Common Shares),NYSE
ELS,\"Equity Lifestyle Properties, Inc. Common Stock\",NYSE
ELV,\"Elevance Health, Inc. Common Stock\",NYSE
EMD,Western Asset Emerging Markets Debt Fund Inc Common Stock,NYSE
EME,\"EMCOR Group, Inc. Common Stock\",NYSE
EMF,Templeton Emerging Markets Fund Common Stock,NYSE
EMN,Eastman Chemical Company Common Stock,NYSE
EMO,ClearBridge Energy Midstream Opportunity Fund Inc. Common Stock,NYSE
EMP,\"Entergy Mississippi, LLC First Mortgage Bonds, 4.90% Series Due October 1, 2066\",NYSE
EMR,Emerson Electric Company Common Stock,NYSE
EMX,EMX Royalty Corporation Common Shares (Canada),NYSE MKT
ENB,Enbridge Inc Common Stock,NYSE
ENFN,\"Enfusion, Inc. Class A Common Stock\",NYSE
ENIC,Enel Chile S.A. American Depositary Shares (Each representing 50 shares of Common Stock),NYSE
ENJ,\"Entergy New Orleans, LLC First Mortgage Bonds, 5.0% Series due December 1, 2052\",NYSE
ENLC,\"EnLink Midstream, LLC Common Units representing Limited Partner Interests\",NYSE
ENO,\"Entergy New Orleans, LLC First Mortgage Bonds, 5.50% Series due April 1, 2066\",NYSE
ENOV,Enovis Corporation Common Stock,NYSE
ENR,\"Energizer Holdings, Inc. Common Stock\",NYSE
ENS,EnerSys Common Stock,NYSE
ENSV,Enservco Corporation Common Stock,NYSE MKT
ENV,\"Envestnet, Inc Common Stock\",NYSE
ENVA,\"Enova International, Inc. Common Stock\",NYSE
ENX,\"Eaton Vance New York Municipal Bond Fund Common Shares of Beneficial Interest, $.01 par value\",NYSE MKT
ENZ,\"Enzo Biochem, Inc. Common Stock ($0.01 Par Value)\",NYSE
EOD,Allspring Global Dividend Opportunity Fund Common Shares of Beneficial Interest,NYSE
EOG,\"EOG Resources, Inc. Common Stock\",NYSE
EOI,Eaton Vance Enhance Equity Income Fund Eaton Vance Enhanced Equity Income Fund Shares of Beneficial Interest,NYSE
EOS,Eaton Vance Enhance Equity Income Fund II Common Stock,NYSE
EOT,Eaton Vance Municipal Income Trust EATON VANCE NATIONAL MUNICIPAL OPPORTUNITIES TRUST,NYSE
EP,Empire Petroleum Corporation Common Stock,NYSE MKT
EP$C,El Paso Corporation Preferred Stock,NYSE
EPAC,Enerpac Tool Group Corp. Common Stock,NYSE
EPAM,\"EPAM Systems, Inc. Common Stock\",NYSE
EPC,Edgewell Personal Care Company Common Stock,NYSE
EPD,Enterprise Products Partners L.P. Common Stock,NYSE
EPM,\"Evolution Petroleum Corporation, Inc. Common Stock\",NYSE MKT
EPR,EPR Properties Common Stock,NYSE
EPR$C,EPR Properties 5.75% Series C Cumulative Convertible Preferred Shares,NYSE
EPR$E,EPR Properties Series E Cumulative Conv Pfd Shs Ser E,NYSE
EPR$G,EPR Properties 5.750% Series G Cumulative Redeemable Preferred Shares,NYSE
EPRT,\"Essential Properties Realty Trust, Inc. Common Stock\",NYSE
EQBK,\"Equity Bancshares, Inc. Class A Common Stock\",NYSE
EQC,Equity Commonwealth Common Shares of Beneficial Interest,NYSE
EQC$D,Equity Commonwealth 6.50% Pfd Conv Shs Ser D,NYSE
EQH,\"Equitable Holdings, Inc. Common Stock\",NYSE
EQH$A,\"Equitable Holdings, Inc. Depositary Shares\",NYSE
EQH$C,\"Equitable Holdings, Inc. Depositary Shares, each representing a 1/1,000th interest in a share of Fixed Rate Noncumulative Perpetual Preferred Stock, Series C\",NYSE
EQNR,Equinor ASA,NYSE
EQR,Equity Residential Common Shares of Beneficial Interest,NYSE
EQS,\"Equus Total Return, Inc. Common Stock\",NYSE
EQT,EQT Corporation Common Stock,NYSE
EQX,Equinox Gold Corp. Common Shares,NYSE MKT
ERC,Allspring Multi-Sector Income Fund Common Stock,NYSE MKT
ERF,Enerplus Corporation Common Stock,NYSE
ERH,Allspring Utilities and High Income Fund Common Shares,NYSE MKT
ERJ,Embraer S.A. Common Stock,NYSE
ERO,Ero Copper Corp. Common Shares,NYSE
ES,Eversource Energy (D/B/A) Common Stock,NYSE
ESAB,ESAB Corporation Common Stock,NYSE
ESBA,\"Empire State Realty OP, L.P. Series ES Operating Partnership Units Representing Limited Partnership Interests\",NYSE ARCA
ESE,ESCO Technologies Inc. Common Stock,NYSE
ESI,Element Solutions Inc. Common Stock,NYSE
ESNT,Essent Group Ltd. Common Shares,NYSE
ESP,Espey Mfg. & Electronics Corp. Common Stock,NYSE MKT
ESRT,\"Empire State Realty Trust, Inc. Class A Common Stock\",NYSE
ESS,\"Essex Property Trust, Inc. Common Stock\",NYSE
ESTC,Elastic N.V. Ordinary Shares,NYSE
ET,Energy Transfer LP Common Units ,NYSE
ET$I,Energy Transfer L.P. Series I Fixed Rate Perpetual Preferred Units,NYSE
ETB,Eaton Vance Tax-Managed Buy-Write Income Fund Eaton Vance Tax-Managed Buy-Write Income Fund Common Shares of Beneficial Interest,NYSE
ETD,Ethan Allen Interiors Inc. Common Stock,NYSE
ETG,Eaton Vance Tax-Advantaged Global Dividend Income Fund Common Shares of Beneficial Interest,NYSE
ETI$,\"Entergy Texas Inc 5.375% Series A Preferred Stock, Cumulative, No Par Value\",NYSE
ETJ,Eaton Vance Risk-Managed Diversified Equity Income Fund Common Shares of Beneficial Interest,NYSE
ETN,\"Eaton Corporation, PLC Ordinary Shares\",NYSE
ETO,Eaton Vance Tax-Advantage Global Dividend Opp Common Stock,NYSE
ETR,Entergy Corporation Common Stock,NYSE
ETRN,Equitrans Midstream Corporation Common Stock ,NYSE
ETV,Eaton Vance Corporation Eaton Vance Tax-Managed Buy-Write Opportunities Fund Common Shares of Beneficial Interest,NYSE
ETW,Eaton Vance Corporation Eaton Vance Tax-Managed Global Buy-Write Opportunites Fund Common Shares of Beneficial Interest,NYSE
ETWO,\"E2open Parent Holdings, Inc.Class A Common Stock\",NYSE
ETWO.W,\"E2open Parent Holdings, Inc. Warrants\",NYSE
ETX,Eaton Vance Municipal Income 2028 Term Trust Common Shares of Beneficial Interest,NYSE
ETY,\"Eaton Vance Tax-Managed Diversified Equity Income Fund Common Shares of Beneficial Interest,\",NYSE
EURN,Euronav NV Ordinary Shares,NYSE
EVA,Enviva Inc. Common Stock,NYSE
EVBN,\"Evans Bancorp, Inc. Common Stock\",NYSE MKT
EVC,Entravision Communications Corporation Common Stock,NYSE
EVE,EVe Mobility Acquisition Corp Class A Ordinary Shares,NYSE MKT
EVE.U,\"EVe Mobility Acquisition Corp Units, each consisting of one Class A ordinary share and one-half of one redeemable warrant(\",NYSE MKT
EVE.W,\"EVe Mobility Acquisition Corp Redeemable warrants, each whole warrant exercisable for one Class A ordinary share at an exercise price of $11.50\",NYSE MKT
EVEX,\"Eve Holding, Inc. Common Stock\",NYSE
EVEX.W,\"Eve Holding, Inc. Warrants, each exercisable for one share of Common Stock at an exercise price of $11.50 per share\",NYSE
EVF,Eaton Vance Senior Income Trust Common Stock,NYSE
EVG,Eaton Vance Short Diversified Income Fund Eaton Vance Short Duration Diversified Income Fund Common Shares of Beneficial Interest,NYSE
EVH,\"Evolent Health, Inc Class A Common Stock\",NYSE
EVI,\"EVI Industries, Inc.  Common Stock\",NYSE MKT
EVM,\"Eaton Vance California Municipal Bond Fund Common Shares of Beneficial Interest, $.01 par value\",NYSE MKT
EVN,Eaton Vance Municipal Income Trust Common Stock,NYSE
EVR,Evercore Inc. Class A Common Stock,NYSE
EVRI,Everi Holdings Inc. Common Stock,NYSE
EVT,Eaton Vance Tax Advantaged Dividend Income Fund Common Shares of Beneficial Interest,NYSE
EVTC,\"Evertec, Inc. Common Stock\",NYSE
EVTL,Vertical Aerospace Ltd. Ordinary Shares,NYSE
EVTL.W,\"Vertical Aerospace Ltd. Warrants, each whole warrant exercisable for one ordinary share at an exercise price of $11.50 per share\",NYSE
EVV,Eaton Vance Limited Duration Income Fund Common Shares of Beneficial Interest,NYSE MKT
EW,Edwards Lifesciences Corporation Common Stock,NYSE
EXG,Eaton Vance Tax-Managed Global Diversified Equity Income Fund Common Shares of Beneficial Interest,NYSE
EXK,Endeavour Silver Corporation Ordinary Shares (Canada),NYSE
EXP,Eagle Materials Inc Common Stock,NYSE
EXPD,\"Expeditors International of Washington, Inc. Common Stock\",NYSE
EXR,Extra Space Storage Inc Common Stock,NYSE
EXTO,\"Almacenes Exito S.A. American Depositary Share, each representing eight (8) Common Shares\",NYSE
F,Ford Motor Company Common Stock,NYSE
F$B,\"Ford Motor Company 6.20% Notes due June 1, 2059\",NYSE
F$C,\"Ford Motor Company 6% Notes due December 1, 2059\",NYSE
F$D,\"Ford Motor Company 6.500% Notes due August 15, 2062\",NYSE
FAF,First American Corporation (New) Common Stock,NYSE
FAM,First Trust/abrdn Global Opportunity Income Fund Common Shares of Beneficial Interest,NYSE
FAX,\"abrdn Asia-Pacific Income Fund, Inc. Common Stock\",NYSE MKT
FBGX,UBS AG FI Enhanced Large Cap Growth ETN,NYSE ARCA
FBIN,\"Fortune Brands Innovations, Inc. Common Stock\",NYSE
FBK,FB Financial Corporation Common Stock,NYSE
FBP,First BanCorp. New Common Stock,NYSE
FBRT,\"Franklin BSP Realty Trust, Inc. Common Stock\",NYSE
FBRT$E,\"Franklin BSP Realty Trust, Inc. 7.50% Series E Cumulative Redeemable Preferred Stock\",NYSE
FC,Franklin Covey Company Common Stock,NYSE
FCF,First Commonwealth Financial Corporation Common Stock,NYSE
FCN,\"FTI Consulting, Inc. Common Stock\",NYSE
FCO,\"abrdn Global Income Fund, Inc. Common Stock\",NYSE MKT
FCPT,\"Four Corners Property Trust, Inc. Common Stock\",NYSE
FCRX,\"Crescent Capital BDC, Inc. 5.00% Notes due 2026\",NYSE
FCT,First Trust Senior Floating Rate Income Fund II Common Shares of Beneficial Interest,NYSE
FCX,\"Freeport-McMoRan, Inc. Common Stock\",NYSE
FDP,\"Fresh Del Monte Produce, Inc. Common Stock\",NYSE
FDS,FactSet Research Systems Inc. Common Stock,NYSE
FDX,FedEx Corporation Common Stock,NYSE
FE,FirstEnergy Corp. Common Stock,NYSE
FEDU,\"Four Seasons Education (Cayman) Inc. American Depositary Shares, each ADS representing 10 ordinary shares\",NYSE
FENG,\"Phoenix New Media Limited American Depositary Shares, each representing 48 Class A ordinary shares.\",NYSE
FERG,Ferguson plc Ordinary Shares,NYSE
FET,\"Forum Energy Technologies, Inc. Common Stock\",NYSE
FF,FutureFuel Corp.  Common shares,NYSE
FFA,First Trust Enhanced Equity Income Fund,NYSE
FFC,Flaherty & Crumrine Preferred and Income Securities Fund Incorporated,NYSE
FFWM,First Foundation Inc. Common Stock,NYSE
FG,\"F&G Annuities & Life, Inc. Common Stock\",NYSE
FGB,First Trust Specialty Finance and Financial Opportunities Fund,NYSE
FGN,\"F&G Annuities & Life, Inc. 7.950% Senior Notes due 2053\",NYSE
FHI,\"Federated Hermes, Inc. Common Stock\",NYSE
FHN,First Horizon Corporation Common Stock,NYSE
FHN$B,\"First Horizon Corporation Depositary Shares, each representing a 1/400th interest in a share of Non-Cumulative Perpetual Preferred Stock, Series B\",NYSE
FHN$C,\"First Horizon Corporation Depositary Shares, each representing a 1/400th interest in a share of Non-Cumulative Perpetual Preferred Stock, Series C\",NYSE
FHN$E,\"First Horizon Corporation Depositary Shares, each representing a 1/4,000th interest in a share of Non-Cumulative Perpetual Preferred Stock, Series E\",NYSE
FHN$F,\"First Horizon Corporation Depositary Shares, each representing 1/4000th Interest in a Share of Non-Cumulative Perpetual Preferred Stock, Series F\",NYSE
FI,\"Fiserv, Inc. Common Stock\",NYSE
FICO,Fair Isaac Corproation Common Stock,NYSE
FIGS,\"FIGS, Inc. Class A Common Stock\",NYSE
FIHL,Fidelis Insurance Holdings Limited Common Shares,NYSE
FINS,Angel Oak Financial Strategies Income Term Trust Common Shares of Beneficial Interest,NYSE
FINV,FinVolution Group American Depositary Shares,NYSE
FIS,\"Fidelity National Information Services, Inc. Common Stock\",NYSE
FISK,\"Empire State Realty OP, L.P. Series 250 Operating Partnership Units Representing Limited Partnership Interests\",NYSE ARCA
FIX,\"Comfort Systems USA, Inc. Common Stock\",NYSE
FL,\"Foot Locker, Inc.\",NYSE
FLC,Flaherty & Crumrine Total Return Fund Inc Common Stock,NYSE
FLNG,FLEX LNG Ltd. Ordinary Shares,NYSE
FLO,\"Flowers Foods, Inc. Common Stock\",NYSE
FLR,Fluor Corporation Common Stock,NYSE
FLS,Flowserve Corporation Common Stock,NYSE
FLUT,Flutter Entertainment plc Ordinary Shares,NYSE
FLYX,\"flyExclusive, Inc. Class A Common Stock\",NYSE MKT
FLYX.W,\"flyExclusive, Inc. Redeemable warrants, each whole warrant exercisable for one Class A common stock at an exercise price of $11.50 per share\",NYSE MKT
FMC,FMC Corporation Common Stock,NYSE
FMN,Federated Hermes Premier Municipal Income Fund,NYSE
FMS,Fresenius Medical Care AG American Depositary Shares (Each representing 1/2 of an Ordinary Share),NYSE
FMX,Fomento Economico Mexicano S.A.B. de C.V. Common Stock,NYSE
FMY,First Trust Motgage Income Fund Common Shares of Beneficial Interest,NYSE
FN,Fabrinet Ordinary Shares,NYSE
FNA,\"Paragon 28, Inc. Common Stock\",NYSE
FNB,F.N.B. Corporation Common Stock,NYSE
FND,\"Floor & Decor Holdings, Inc. Common Stock\",NYSE
FNF,\"FNF Group of Fidelity National Financial, Inc. Common Stock\",NYSE
FNGD,\"MicroSectors FANG  Index -3X Inverse Leveraged ETNs due January 8, 2038\",NYSE ARCA
FNGO,\"MicroSectors FANG  Index 2X Leveraged ETNs due January 8, 2038\",NYSE ARCA
FNGU,\"MicroSectors FANG  Index 3X Leveraged ETNs due January 8, 2038\",NYSE ARCA
FNV,Franco-Nevada Corporation,NYSE
FOA,Finance of America Companies Inc. Class A Common Stock,NYSE
FOA.W,\"Finance of America Companies Inc. Warrants, each whole warrant exercisable for one Class A Common Stock at an exercise price of $11.50\",NYSE
FOF,\"Cohen & Steers Closed-End Opportunity Fund, Inc. Common Stock\",NYSE
FOR,Forestar Group Inc Common Stock ,NYSE
FOUR,\"Shift4 Payments, Inc. Class A Common Stock\",NYSE
FOXO,FOXO Technologies Inc. Class A Common Stock,NYSE MKT
FPF,First Trust Intermediate Duration Preferred & Income Fund Common Shares of Beneficial Interest,NYSE
FPH,\"Five Point Holdings, LLC Class A Common Shares\",NYSE
FPI,Farmland Partners Inc. Common Stock,NYSE
FR,\"First Industrial Realty Trust, Inc. Common Stock\",NYSE
FRA,Blackrock Floating Rate Income Strategies Fund Inc  Common Stock,NYSE
FRD,Friedman Industries Inc. Common Stock,NYSE MKT
FREY,\"FREYR Battery, Inc. Common Stock\",NYSE
FREY.W,FREYR Battery Warrants each whole warrant exercisable to purchase one Common Stock at an exercise price of $11.50 per share,NYSE
FRGE,\"Forge Global Holdings, Inc. Common Stock\",NYSE
FRO,Frontline Plc Ordinary Shares,NYSE
FRT,Federal Realty Investment Trust Common Stock,NYSE
FRT$C,\"Federal Realty Investment Trust Depositary Shares, each representing a 1/1000th interest in a 5.000% Series C Cumulative Redeemable Preferred Share\",NYSE
FSCO,FS Credit Opportunities Corp. Common Stock,NYSE
FSD,First Trust High Income Long Short Fund Common Shares of Beneficial Interest,NYSE
FSI,Flexible Solutions International Inc. Common Stock (CDA),NYSE MKT
FSK,FS KKR Capital Corp. Common Stock,NYSE
FSLY,\"Fastly, Inc. Class A Common Stock\",NYSE
FSM,Fortuna Silver Mines Inc Ordinary Shares (Canada),NYSE
FSP,Franklin Street Properties Corp. Common Stock,NYSE MKT
FSS,Federal Signal Corporation Common Stock,NYSE
FT,Franklin Universal Trust Common Stock,NYSE
FTF,Franklin Limited Duration Income Trust Common Shares of Beneficial Interest,NYSE MKT
FTHY,First Trust High Yield Opportunities 2027 Term Fund Common Stock,NYSE
FTI,TechnipFMC plc Ordinary Share,NYSE
FTK,\"Flotek Industries, Inc. Common Stock\",NYSE
FTS,Fortis Inc. Common Shares,NYSE
FTV,Fortive Corporation Common Stock ,NYSE
FUBO,fuboTV Inc. Common Stock,NYSE
FUL,H. B. Fuller Company Common Stock,NYSE
FUN,\"Cedar Fair, L.P. Common Stock\",NYSE
FURY,Fury Gold Mines Limited Common Shares,NYSE MKT
FVRR,\"Fiverr International Ltd. Ordinary Shares, no par value\",NYSE
G,Genpact Limited Common Stock,NYSE
GAB,\"Gabelli Equity Trust, Inc. (The) Common Stock\",NYSE
GAB$G,\"Gabelli Equity Trust, Inc. (The) Series G Cumulative Preferred Stock\",NYSE
GAB$H,\"Gabelli Equity Trust, Inc. (The) Pfd Ser H\",NYSE
GAB$K,\"Gabelli Equity Trust, Inc. (The) 5.00% Series K Cumulative Preferred Stock\",NYSE
GAM,\"General American Investors, Inc. Common Stock\",NYSE
GAM$B,\"General American Investors Company, Inc. Cumulative Preferred Stock\",NYSE
GATO,\"Gatos Silver, Inc. Common Stock\",NYSE
GATX,GATX Corporation Common Stock,NYSE
GAU,Galiano Gold Inc.,NYSE MKT
GB,Global Blue Group Holding AG Ordinary Shares,NYSE
GB.W,Global Blue Group Holding AG Warrants exercisable for one Ordinary Share of Global Blue Group Holding AG at a price of $11.50 per share,NYSE
GBAB,Guggenheim Taxable Municipal Bond & Investment Grade Debt Trust Common Shares of Beneficial Interest,NYSE
GBCI,\"Glacier Bancorp, Inc. Common Stock\",NYSE
GBLI,\"Global Indemnity Group, LLC Class A Common Stock (DE)\",NYSE
GBR,\"New Concept Energy, Inc Common Stock\",NYSE MKT
GBTG,\"Global Business Travel Group, Inc. Class A Common Stock\",NYSE
GBX,\"Greenbrier Companies, Inc. (The) Common Stock\",NYSE
GCI,\"Gannett Co., Inc. Common Stock\",NYSE
GCO,Genesco Inc. Common Stock,NYSE
GCTS,\"GCT Semiconductor Holding, Inc. Common Stock\",NYSE
GCTS.W,\"GCT Semiconductor Holding, Inc. Warrants, each whole warrant exercisable for one share of Common Stock at an exercise price of $11.50\",NYSE
GCV,\"Gabelli Convertible and Income Securities Fund, Inc. (The) Common Stock\",NYSE
GD,General Dynamics Corporation Common Stock,NYSE
GDDY,GoDaddy Inc. Class A Common Stock,NYSE
GDL,\"GDL Fund, The Common Shares of Beneficial Interest\",NYSE
GDL$C,The GDL Fund Series C Cumulative Puttable and Callable Preferred Shares,NYSE
GDO,Western Asset Global Corporate Defined Opportunity Fund Inc.,NYSE
GDOT,\"Green Dot Corporation Class A Common Stock, $0.001 par value\",NYSE
GDV,Gabelli Dividend & Income Trust Common Shares of Beneficial Interest,NYSE
GDV$H,The Gabelli Dividend & Income Trust 5.375% Series H Cumulative Preferred Shares,NYSE
GDV$K,The Gabelli Dividend & Income Trust 4.250% Series K Cumulative Preferred Shares,NYSE
GE,GE Aerospace Common Stock,NYSE
GEF,Greif Inc. Class A Common Stock,NYSE
GEF.B,\"Greif, Inc. Corporation Class B Common Stock\",NYSE
GEL,\"Genesis Energy, L.P. Common Units\",NYSE
GENC,\"Gencor Industries, Inc. Common Stock\",NYSE MKT
GENI,Genius Sports Limited Ordinary Shares,NYSE
GEO,Geo Group Inc (The) REIT,NYSE
GES,\"Guess?, Inc. Common Stock\",NYSE
GETR,\"Getaround, Inc. Common Stock\",NYSE
GETY,\"Getty Images Holdings, Inc. Class A Common Stock\",NYSE
GEV,GE Vernova Inc. Common Stock,NYSE
GF,\"New Germany Fund, Inc. (The) Common Stock\",NYSE
GFF,Griffon Corporation Common Stock,NYSE
GFI,Gold Fields Limited American Depositary Shares,NYSE
GFL,\"GFL Environmental Inc. Subordinate voting shares, no par value\",NYSE
GFR,Greenfire Resources Ltd. Common Shares,NYSE
GGB,Gerdau S.A. Common Stock,NYSE
GGG,Graco Inc. Common Stock,NYSE
GGN,\"GAMCO Global Gold, Natural Resources & Income Trust\",NYSE MKT
GGN$B,\"GAMCO Global Gold, Natural Reources & Income Trust 5.00% Series B Cumulative 25.00 Liquidation Preference\",NYSE MKT
GGT,\"Gabelli Multi-Media Trust, Inc. (The) Common Stock\",NYSE
GGT$E,Gabelli Multi-Media Trust Inc. (The) 5.125% Series E Cumulative Preferred Stock,NYSE
GGT$G,Gabelli Multi-Media Trust Inc. (The) 5.125% Series G Cumulative Preferred Shares,NYSE
GGZ,Gabelli Global Small and Mid Cap Value Trust (The) Common Shares of Beneficial Interest,NYSE
GHC,Graham Holdings Company Common Stock,NYSE
GHG,\"GreenTree Hospitality Group Ltd. American depositary shares, each representing one Class A ordinary share\",NYSE
GHI,Greystone Housing Impact Investors LP Beneficial Unit Certificates representing assignments of limited partnership interests,NYSE
GHLD,Guild Holdings Company Class A Common Stock,NYSE
GHM,Graham Corporation Common Stock,NYSE
GHY,\"PGIM Global High Yield Fund, Inc.\",NYSE
GIB,CGI Inc. Common Stock,NYSE
GIC,Global Industrial Company Common Stock,NYSE
GIL,\"Gildan Activewear, Inc. Class A Sub. Vot. Common Stock\",NYSE
GIS,\"General Mills, Inc. Common Stock\",NYSE
GJH,Synthetic Fixed-Income Securities Inc 6.375% (STRATS) Cl A-1,NYSE
GJO,\"Synthetic Fixed-Income Securities, Inc. on behalf of STRATS(SM) Trust for Wal-Mart Stores, Inc. Securities, Series 2004-5\",NYSE
GJP,\"Synthetic Fixed-Income Securities, Inc. on behalf of STRATS (SM) Trust for Dominion Resources, Inc. Securities, Series 2005-6, Floating Rate Structured Repackaged Asset-Backed Trust Securities (STRATS) Certificates\",NYSE
GJR,\"Synthetic Fixed-Income Securities, Inc. STRATS Trust for Procter&Gamble Securities, Series 2006-1\",NYSE
GJS,\"Goldman Sachs Group Securities STRATS Trust for Goldman Sachs Group Securities, Series 2006-2\",NYSE
GJT,\"Synthetic Fixed-Income Securities, Inc. Floating Rate Structured Repackaged Asset-Backed Trust Securities Certificates, Series 2006-3\",NYSE
GKOS,Glaukos Corporation Common Stock,NYSE
GL,Globe Life Inc. Common Stock,NYSE
GL$D,Globe Life Inc. 4.25% Junior Subordinated Debentures due 2061,NYSE
GLDG,GoldMining Inc. Common Shares,NYSE MKT
GLO,Clough Global Opportunities Fund Common Stock,NYSE MKT
GLOB,Globant S.A. Common Shares,NYSE
GLOG$A,GasLog LP. 8.75% Series A Cumulative Redeemable Perpetual Preference Shares,NYSE
GLOP$A,GasLog Partners LP 8.625% Series A Cumulative Redeemable Perpetual Fixed to Floating Rate Preference Units,NYSE
GLOP$B,GasLog Partners LP 8.200% Series B Cumulative Redeemable Perpetual Fixed to Floating Rate Preference Units,NYSE
GLOP$C,GasLog Partners LP 8.500% Series C Cumulative Redeemable Perpetual Fixed to Floating Rate Preference Units,NYSE
GLP,Global Partners LP Common Units representing Limited Partner Interests,NYSE
GLP$B,Global Partners LP 9.50% Series B Fixed Rate Cumulative Redeemable Perpetual Preferred Units representing limited partner interests,NYSE
GLQ,Clough Global Equity Fund Clough Global Equity Fund Common Shares of Beneficial Interest,NYSE MKT
GLT,Glatfelter Corporation Common Stock,NYSE
GLU,Gabelli Global Utility Common Shares of Beneficial Ownership,NYSE MKT
GLU$A,The Gabelli Global Utility and Income Trust Series A Cumulative Puttable and Callable Preferred Shares,NYSE MKT
GLU$B,The Gabelli Global Utility and Income Trust Series B Cumulative Puttable and Callable Preferred Shares,NYSE MKT
GLV,Clough Global Dividend and Income Fund Common Shares of beneficial interest,NYSE MKT
GLW,Corning Incorporated Common Stock,NYSE
GM,General Motors Company Common Stock,NYSE
GME,GameStop Corporation Common Stock,NYSE
GMED,\"Globus Medical, Inc. Class A Common Stock\",NYSE
GMRE,Global Medical REIT Inc. Common Stock,NYSE
GMRE$A,Global Medical REIT Inc. Series A Cumulative Redeemable Preferred Stock,NYSE
GMS,GMS Inc. Common Stock,NYSE
GNE,Genie Energy Ltd. Class B Common Stock Stock,NYSE
GNK,Genco Shipping & Trading Limited Ordinary Shares New (Marshall Islands),NYSE
GNL,\"Global Net Lease, Inc. Common Stock\",NYSE
GNL$A,\"Global Net Lease, Inc. 7.25% Series A Cumulative Redeemable Preferred Stock, $0.01 par value per share\",NYSE
GNL$B,\"Global Net Lease, Inc. 6.875% Series B Cumulative Redeemable Perpetual Preferred Stock\",NYSE
GNL$D,\"Global Net Lease, Inc. 7.50% Series D Cumulative Redeemable Perpetual Preferred Stock\",NYSE
GNL$E,\"Global Net Lease, Inc. 7.375% Series E Cumulative Redeemable Perpetual Preferred Stock\",NYSE
GNRC,Generac Holdlings Inc. Common Stock,NYSE
GNS,Genius Group Limited Ordinary Shares,NYSE MKT
GNT,\"GAMCO Natural Resources, Gold & Income Trust\",NYSE
GNT$A,\"GAMCO Natural Resources, Gold & Income Tust  5.20% Series A Cumulative Preferred Shares (Liquidation Preference $25.00 per share)\",NYSE
GNTY,\"Guaranty Bancshares, Inc. Common Stock\",NYSE
GNW,Genworth Financial Inc Common Stock,NYSE
GOF,Guggenheim Strategic Opportunities Fund Common Shares of Beneficial Interest,NYSE
GOLD,Barrick Gold Corporation Common Stock (BC),NYSE
GOLF,Acushnet Holdings Corp. Common Stock,NYSE
GOOS,Canada Goose Holdings Inc. Subordinate Voting Shares,NYSE
GORO,Gold Resource Corporation Common Stock,NYSE MKT
GOTU,Gaotu Techedu Inc. American Depositary Shares,NYSE
GPC,Genuine Parts Company Common Stock,NYSE
GPI,\"Group 1 Automotive, Inc. Common Stock\",NYSE
GPJA,\"Georgia Power Company Series 2017A 5.00% Junior Subordinated Notes due October 1, 2077\",NYSE
GPK,Graphic Packaging Holding Company,NYSE
GPMT,Granite Point Mortgage Trust Inc. Common Stock,NYSE
GPMT$A,Granite Point Mortgage Trust Inc. 7.00% Series A Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock,NYSE
GPN,Global Payments Inc. Common Stock,NYSE
GPOR,Gulfport Energy Corporation Common Shares,NYSE
GPRK,Geopark Ltd Common Shares,NYSE
GPS,\"Gap, Inc. (The) Common Stock\",NYSE
GRBK,\"Green Brick Partners, Inc. Common Stock\",NYSE
GRBK$A,\"Green Brick Partners, Inc. Depositary Shares (each representing a 1/1000th fractional interest in a share of 5.75% Series A Cumulative Perpetual Preferred Stock)\",NYSE
GRC,Gorman-Rupp Company (The) Common Stock,NYSE
GRF,\"Eagle Capital Growth Fund, Inc. Common Stock\",NYSE MKT
GRFX,\"Graphex Group Limited American Depositary Shares, each American Depositary Share representing 20 Ordinary Shares\",NYSE MKT
GRMN,Garmin Ltd. Common Stock (Switzerland),NYSE
GRN,iPath Series B Carbon Exchange-Traded Notes,NYSE ARCA
GRND,Grindr Inc. Common Stock,NYSE
GRND.W,\"Grindr Inc. Warrants, each exercisable for one share of Common Stock at an exercise price of $11.50 per share\",NYSE
GRNT,\"Granite Ridge Resources, Inc. Common Stock\",NYSE
GROV,\"Grove Collaborative Holdings, Inc. Class A Common Stock\",NYSE
GROY,Gold Royalty Corp. Common Shares,NYSE MKT
GRP.U,\"Granite Real Estate Inc. Stapled Units, each consisting of one unit of Granite Real Estate Trust and one common share of Granite REIT Inc.\",NYSE
GRX,The Gabelli Healthcare & Wellness Trust Common Shares of Beneficial Interest,NYSE
GS,\"Goldman Sachs Group, Inc. (The) Common Stock\",NYSE
GS$A,\"Goldman Sachs Group, Inc. (The) Depositary Shares each representing 1/1000th Interest in a Share of Floating Rate Non-Cumulative Preferred Stock Series A\",NYSE
GS$C,\"Goldman Sachs Group, Inc. (The) Depositary Share repstg 1/1000th Preferred Series C\",NYSE
GS$D,\"Goldman Sachs Group, Inc. (The) Dep Shs repstg 1/1000 Pfd Ser D Fltg\",NYSE
GSAT,\"Globalstar, Inc. Common Stock\",NYSE MKT
GSBD,\"Goldman Sachs BDC, Inc. Common Stock\",NYSE
GSK,GSK plc American Depositary Shares (Each representing two Ordinary Shares),NYSE
GSL,Global Ship Lease Inc New Class A Common Shares,NYSE
GSL$B,\"Global Ship Lease, Inc. Depository Shares Representing 1/100th Perpetual Preferred Series B% (Marshall Island)\",NYSE
GTE,Gran Tierra Energy Inc. Common Stock,NYSE MKT
GTES,Gates Industrial Corporation plc Ordinary Shares,NYSE
GTLS,\"Chart Industries, Inc. Common Stock\",NYSE
GTLS$B,\"Chart Industries, Inc. Depositary Shares, each Representing a 1/20th Interest in a Share of 6.75% Series B Mandatory Convertible Preferred Stock\",NYSE
GTN,\"Gray Television, Inc. Common Stock\",NYSE
GTN.A,\"Gray Television, Inc. CLass A Common Stock\",NYSE
GTY,Getty Realty Corporation Common Stock,NYSE
GUG,Guggenheim Active Allocation Fund Common Shares of Beneficial Interest,NYSE
GUT,Gabelli Utility Trust (The) Common Stock,NYSE
GUT$C,Gabelli Utility Trust (The) 5.375% Series C Cumulative Preferred Shares,NYSE
GVA,Granite Construction Incorporated Common Stock,NYSE
GWH,\"ESS Tech, Inc. Common Stock\",NYSE
GWH.W,\"ESS Tech, Inc. Warrant\",NYSE
GWRE,\"Guidewire Software, Inc. Common Stock\",NYSE
GWW,\"W.W. Grainger, Inc. Common Stock\",NYSE
GXO,\"GXO Logistics, Inc. Common Stock \",NYSE
H,Hyatt Hotels Corporation Class A Common Stock,NYSE
HAE,Haemonetics Corporation Common Stock,NYSE
HAFN,Hafnia Limited Common Shares,NYSE
HAL,Halliburton Company Common Stock,NYSE
HASI,\"Hannon Armstrong Sustainable Infrastructure Capital, Inc. Common Stock\",NYSE
HAYW,\"Hayward Holdings, Inc. Common Stock\",NYSE
HBB,Hamilton Beach Brands Holding Company Class A Common Stock ,NYSE
HBI,Hanesbrands Inc. Common Stock,NYSE
HBM,Hudbay Minerals Inc. Ordinary Shares (Canada),NYSE
HCA,\"HCA Healthcare, Inc. Common Stock\",NYSE
HCC,\"Warrior Met Coal, Inc. Common Stock\",NYSE
HCI,\"HCI Group, Inc. Common Stock\",NYSE
HCXY,\"Hercules Capital, Inc. 6.25% Notes due 2033\",NYSE
HD,\"Home Depot, Inc. (The) Common Stock\",NYSE
HDB,HDFC Bank Limited Common Stock,NYSE
HE,\"Hawaiian Electric Industries, Inc. Common Stock\",NYSE
HEI,Heico Corporation Common Stock,NYSE
HEI.A,Heico Corporation Common Stock,NYSE
HEQ,John Hancock Hedged Equity & Income Fund Common Shares of Beneficial Interest,NYSE
HES,Hess Corporation Common Stock,NYSE
HESM,Hess Midstream LP Class A Share,NYSE
HFRO,Highland Opportunities and Income Fund Common Shares of Beneficial Interest,NYSE
HFRO$A,Highland Opportunities and Income Fund 5.375% Series A Cumulative Preferred Shares,NYSE
HG,\"Hamilton Insurance Group, Ltd. Class B Common Shares\",NYSE
HGLB,Highland Global Allocation Fund Common Stock,NYSE
HGTY,\"Hagerty, Inc. Class A Common Stock\",NYSE
HGTY.W,\"Hagerty, Inc. Warrants, each whole warrant entitles the holder thereof to purchase one share of Class A common stock at a price of $11.50 per share\",NYSE
HGV,Hilton Grand Vacations Inc. Common Stock ,NYSE
HHH,Howard Hughes Holdings Inc. Common Stock,NYSE
HI,Hillenbrand Inc Common Stock,NYSE
HIE,Miller/Howard High Income Equity Fund Common Shares of Beneficial Interest,NYSE
HIG,\"Hartford Financial Services Group, Inc. (The) Common Stock\",NYSE
HIG$G,\"Hartford Financial Services Group, Inc. (The) Depositary Shares each representing a 1/1,000th interest in a share of 6.000% Non-Cumulative Preferred Stock, Series G, $0.01 par value\",NYSE
HII,\"Huntington Ingalls Industries, Inc. Common Stock\",NYSE
HIMS,\"Hims & Hers Health, Inc. Class A Common Stock\",NYSE
HIO,\"Western Asset High Income Opportunity Fund, Inc. Common Stock\",NYSE
HIPO,Hippo Holdings Inc. Common Stock,NYSE
HIPO.W,Hippo Holdings Inc. Warrants,NYSE
HIW,\"Highwoods Properties, Inc. Common Stock\",NYSE
HIX,Western Asset High Income Fund II Inc. Common Stock,NYSE
HKD,AMTD Digital Inc. American Depositary Shares (every five of which represent two Class A Ordinary Shares),NYSE
HL,Hecla Mining Company Common Stock,NYSE
HL$B,Hecla Mining Company Preferred Stock,NYSE
HLF,Herbalife Ltd. Common Shares,NYSE
HLI,\"Houlihan Lokey, Inc. Class A Common Stock\",NYSE
HLIO,\"Helios Technologies, Inc. Common Stock\",NYSE
HLLY,Holley Inc. Common Stock,NYSE
HLLY.W,Holley Inc. Warrants,NYSE
HLN,Haleon plc American Depositary Shares (Each representing two Ordinary Shares),NYSE
HLT,Hilton Worldwide Holdings Inc. Common Stock ,NYSE
HLX,\"Helix Energy Solutions Group, Inc. Common Stock\",NYSE
HMC,\"Honda Motor Company, Ltd. Common Stock\",NYSE
HMN,Horace Mann Educators Corporation Common Stock,NYSE
HMY,Harmony Gold Mining Company Limited,NYSE
HNI,HNI Corporation Common Stock,NYSE
HNRA,HNR Acquisition Corp Class A Common Stock,NYSE MKT
HNRA.W,\"HNR Acquisition Corp Warrants, each whole warrant exercisable for three quarters of one share of Common Stock at an exercise price of $11.50 per whole share\",NYSE MKT
HNW,\"Pioneer Diversified High Income Fund, Inc.\",NYSE MKT
HOG,\"Harley-Davidson, Inc. Common Stock\",NYSE
HOMB,\"Home BancShares, Inc. Common Stock\",NYSE
HOUS,\"Anywhere Real Estate Inc. Common Stock,\",NYSE
HOV,\"Hovnanian Enterprises, Inc. Class A Common Stock\",NYSE
HP,\"Helmerich & Payne, Inc. Common Stock\",NYSE
HPE,Hewlett Packard Enterprise Company Common Stock,NYSE
HPF,John Hancock Pfd Income Fund II Pfd Income Fund II,NYSE
HPI,John Hancock Preferred Income Fund Common Shares of Beneficial Interest,NYSE
HPP,\"Hudson Pacific Properties, Inc. Common Stock\",NYSE
HPP$C,\"Hudson Pacific Properties, Inc. 4.750% Series C Cumulative Redeemable Preferred Stock\",NYSE
HPQ,HP Inc. Common Stock,NYSE
HPS,John Hancock Preferred Income Fund III Preferred Income Fund III,NYSE
HQH,abrdn Healthcare Investors Shares of Beneficial Interest,NYSE
HQL,abrdn Life Sciences Investors Shares of Beneficial Interest,NYSE
HR,Healthcare Realty Trust Incorporated Common Stock,NYSE
HRB,\"H&R Block, Inc. Common Stock\",NYSE
HRI,Herc Holdings Inc. Common Stock ,NYSE
HRL,Hormel Foods Corporation Common Stock,NYSE
HRT,HireRight Holdings Corporation Common Stock,NYSE
HRTG,\"Heritage Insurance Holdings, Inc. Common Stock\",NYSE
HSBC,\"HSBC Holdings, plc. Common Stock\",NYSE
HSHP,Himalaya Shipping Ltd. Common Shares,NYSE
HSY,The Hershey Company Common Stock,NYSE
HTD,John Hancock Tax Advantaged Dividend Income Fund Common Shares of Beneficial Interest,NYSE
HTFB,Horizon Technology Finance Corporation 4.875% Notes due 2026,NYSE
HTFC,Horizon Technology Finance Corporation 6.25% Notes due 2027,NYSE
HTGC,\"Hercules Capital, Inc. Common Stock\",NYSE
HTH,Hilltop Holdings Inc.,NYSE
HUBB,Hubbell Inc Common Stock,NYSE
HUBS,\"HubSpot, Inc. Common Stock\",NYSE
HUM,Humana Inc. Common Stock,NYSE
HUN,Huntsman Corporation Common Stock,NYSE
HUSA,Houston American Energy Corporation Common Stock,NYSE MKT
HUYA,\"HUYA Inc. American depositary shares, each  representing one Class A ordinary share\",NYSE
HVT,\"Haverty Furniture Companies, Inc. Common Stock\",NYSE
HVT.A,\"Haverty Furniture Companies, Inc. Common Stock\",NYSE
HWM,Howmet Aerospace Inc. Common Stock,NYSE
HWM$,Howmet Aerospace Inc. $3.75 Preferred Stock,NYSE MKT
HXL,Hexcel Corporation Common Stock,NYSE
HY,\"Hyster-Yale Materials Handling, Inc. Class A Common Stock\",NYSE
HYAC,Haymaker Acquisition Corp. 4 Class A Ordinary Shares,NYSE
HYAC.U,\"Haymaker Acquisition Corp. 4 Units, each consisting one Class A Ordinary Share and one-half of one redeemable Warrant\",NYSE
HYAC.W,\"Haymaker Acquisition Corp. 4 Warrants, each whole warrant exercisable for one Class A ordinary share at an exercise price of $11.50 per share\",NYSE
HYB,\"New America High Income Fund, Inc. (The) Common Stock\",NYSE
HYI,Western Asset High Yield Defined Opportunity Fund Inc. Common Stock,NYSE
HYLN,Hyliion Holdings Corp. Class A Common Stock,NYSE
HYT,\"Blackrock Corporate High Yield Fund, Inc. Common Stock\",NYSE
HZO,\"MarineMax, Inc.  (FL) Common Stock\",NYSE
IAE,Voya Asia Pacific High Dividend Equity Income Fund ING Asia Pacific High Dividend Equity Income Fund Common Shares of Beneficial Interest,NYSE
IAF,\"abrdn Australia Equity Fund, Inc. Common Stock\",NYSE MKT
IAG,Iamgold Corporation Ordinary Shares,NYSE
IAUX,i-80 Gold Corp. Common Shares,NYSE MKT
IBIO,\"iBio, Inc. Common Stock\",NYSE MKT
IBM,International Business Machines Corporation Common Stock,NYSE
IBN,ICICI Bank Limited Common Stock,NYSE
IBP,\"Installed Building Products, Inc. Common Stock\",NYSE
IBTA,\"Ibotta, Inc. Class A Common Stock\",NYSE
ICD,\"Independence Contract Drilling, Inc. Common Stock\",NYSE
ICE,Intercontinental Exchange Inc. Common Stock,NYSE
ICL,ICL Group Ltd. Ordinary Shares,NYSE
ICR$A,\"InPoint Commercial Real Estate Income, Inc. 6.75% Series A Cumulative Redeemable Preferred Stock\",NYSE
IDA,\"IDACORP, Inc. Common Stock\",NYSE
IDE,\"Voya Infrastructure, Industrials and Materials Fund Common Shares of Beneficial Interest\",NYSE
IDR,\"Idaho Strategic Resources, Inc. Common Stock\",NYSE MKT
IDT,IDT Corporation Class B Common Stock,NYSE
IE,Ivanhoe Electric Inc. Common Stock,NYSE MKT
IEX,IDEX Corporation Common Stock,NYSE
IFF,\"International Flavors & Fragrances, Inc. Common Stock\",NYSE
IFIN,InFinT Acquisition Corporation Class A Ordinary Shares,NYSE
IFIN.U,\"InFinT Acquisition Corporation Units,each consisting of one Class A ordinary share and one-half of one redeemable warrant\",NYSE
IFN,\"India Fund, Inc. (The) Common Stock\",NYSE
IFS,Intercorp Financial Services Inc. Common Shares,NYSE
IGA,Voya Global Advantage and Premium Opportunity Fund Common Shares of Beneficial Interest,NYSE
IGC,\"IGC Pharma, Inc. Common Stock\",NYSE MKT
IGD,Voya Global Equity Dividend and Premium Opportunity Fund,NYSE
IGI,Western Asset Investment Grade Defined Opportunity Trust Inc. Common Stock,NYSE
IGR,CBRE Global Real Estate Income Fund Common Shares of Beneficial Interest,NYSE
IGT,International Game Technology Ordinary Shares,NYSE
IGZ,IGZ (Listing Market NYSE Arca Network B F) Common Stock,NYSE ARCA
IH,\"iHuman Inc. American depositary shares, each representing five Class A ordinary shares\",NYSE
IHD,Voya Emerging Markets High Income Dividend Equity Fund Common Shares,NYSE
IHG,Intercontinental Hotels Group American Depositary Shares (Each representing one Ordinary Share),NYSE
IHS,IHS Holding Limited Ordinary Shares,NYSE
IHT,InnSuites Hospitality Trust Shares of Beneficial Interest,NYSE MKT
IHTA,\"Invesco High Income 2024 Target Term Fund Common Shares of Beneficial Interest, No par value per share\",NYSE
IIF,\"Morgan Stanley India Investment Fund, Inc. Common Stock\",NYSE
IIIN,\"Insteel Industries, Inc. Common Stock\",NYSE
IIM,Invesco Value Municipal Income Trust Common Stock,NYSE
IIPR,\"Innovative Industrial Properties, Inc. Common Stock\",NYSE
IIPR$A,\"Innovative Industrial Properties, Inc. 9.00% Series A Cumulative Redeemable Preferred Stock\",NYSE
IMAX,Imax Corporation Common Stock,NYSE
IMO,Imperial Oil Limited Common Stock,NYSE MKT
INDO,Indonesia Energy Corporation Limited Ordinary Shares,NYSE MKT
INFA,Informatica Inc. Class A Common Stock,NYSE
INFU,\"InfuSystems Holdings, Inc. Common Stock\",NYSE MKT
INFY,Infosys Limited American Depositary Shares,NYSE
ING,\"ING Group, N.V. Common Stock\",NYSE
INGR,Ingredion Incorporated Common Stock,NYSE
INLX,\"Intellinetics, Inc. Common Stock\",NYSE MKT
INN,\"Summit Hotel Properties, Inc. Common Stock\",NYSE
INN$E,\"Summit Hotel Properties, Inc. 6.250% Series E Cumulative Redeemable Preferred Stock\",NYSE
INN$F,\"Summit Hotel Properties, Inc. 5.875% Series F Cumulative Redeemable Preferred Stock, $0.01 par value per share\",NYSE
INSI,Insight Select Income Fund,NYSE
INSP,\"Inspire Medical Systems, Inc. Common Stock\",NYSE
INST,\"Instructure Holdings, Inc. Common Stock\",NYSE
INSW,\"International Seaways, Inc. Common Stock \",NYSE
INTT,inTest Corporation Common Stock,NYSE MKT
INUV,\"Inuvo, Inc.\",NYSE MKT
INVH,Invitation Homes Inc. Common Stock,NYSE
IONQ,\"IonQ, Inc. Common Stock\",NYSE
IONQ.W,\"IonQ, Inc. Redeemable warrants, each whole warrant exercisable for one share of Common Stock, each at an exercise price of $11.50 per share\",NYSE
IOR,\"Income Opportunity Realty Investors, Inc. Common Stock\",NYSE MKT
IOT,Samsara Inc. Class A Common Stock,NYSE
IP,International Paper Company Common Stock,NYSE
IPG,\"Interpublic Group of Companies, Inc. (The) Common Stock\",NYSE
IPI,\"Intrepid Potash, Inc Common Stock\",NYSE
IQI,Invesco Quality Municipal Income Trust Common Stock,NYSE
IQV,\"IQVIA Holdings, Inc. Common Stock\",NYSE
IR,Ingersoll Rand Inc. Common Stock,NYSE
IRM,Iron Mountain Incorporated (Delaware)Common Stock REIT,NYSE
IRS,IRSA Inversiones Y Representaciones S.A. Global Depositary Shares (Each representing ten shares of Common Stock),NYSE
IRS.W,IRSA Inversiones Y Representaciones S.A. Warrants to purchase Common Shares,NYSE
IRT,\"Independence Realty Trust, Inc. Common Stock\",NYSE
ISD,\"PGIM High Yield Bond Fund, Inc.\",NYSE
ISDR,Issuer Direct Corporation Common Stock,NYSE MKT
IT,\"Gartner, Inc. Common Stock\",NYSE
ITGR,Integer Holdings Corporation Common Stock,NYSE
ITP,\"IT Tech Packaging, Inc. Common Stock\",NYSE MKT
ITRG,Integra Resources Corp. Common Shares,NYSE MKT
ITT,ITT Inc. Common Stock ,NYSE
ITUB,Itau Unibanco Banco Holding SA American Depositary Shares (Each repstg 500 Preferred shares),NYSE
ITW,Illinois Tool Works Inc. Common Stock,NYSE
IVR,INVESCO MORTGAGE CAPITAL INC Common Stock,NYSE
IVR$B,Invesco Mortgage Capital Inc. Preferred Series B Cum Fxd to Fltg,NYSE
IVR$C,\"INVESCO MORTGAGE CAPITAL INC 7.5% Fixed-to-Floating Series C Cumulative Redeemable Preferred Stock, Liquation Preference $25.00 per Share\",NYSE
IVT,InvenTrust Properties Corp. Common Stock,NYSE
IVZ,Invesco Ltd Common Stock,NYSE
IX,Orix Corp Ads Common Stock,NYSE
J,Jacobs Solutions Inc. Common Stock,NYSE
JBGS,JBG SMITH Properties Common Shares ,NYSE
JBI,\"Janus International Group, Inc. Common Stock\",NYSE
JBK,Lehman ABS 3.50 3.50% Adjustable Corp Backed Tr Certs GS Cap I,NYSE
JBL,Jabil Inc. Common Stock,NYSE
JBT,John Bean Technologies Corporation Common Stock,NYSE
JCE,Nuveen Core Equity Alpha Fund Common Shares of Beneficial Interest,NYSE
JCI,Johnson Controls International plc Ordinary Share,NYSE
JEF,Jefferies Financial Group Inc. Common Stock,NYSE
JELD,\"JELD-WEN Holding, Inc. Common Stock\",NYSE
JEQ,\"abrdn Japan Equity Fund, Inc. Common Stock\",NYSE
JFR,Nuveen Floating Rate Income Fund Common Stock,NYSE
JGH,Nuveen Global High Income Fund Common Shares of Beneficial Interest,NYSE
JHG,Janus Henderson Group plc Ordinary Shares,NYSE
JHI,John Hancock Investors Trust Common Stock,NYSE
JHS,John Hancock Income Securities Trust Common Stock,NYSE
JHX,James Hardie Industries plc American Depositary Shares (Ireland),NYSE
JILL,\"J. Jill, Inc. Common Stock\",NYSE
JKS,JinkoSolar Holding Company Limited American Depositary Shares (each representing 4 Common Shares),NYSE
JLL,Jones Lang LaSalle Incorporated Common Stock,NYSE
JLS,Nuveen Mortgage and Income Fund,NYSE
JMIA,\"Jumia Technologies AG American Depositary Shares, each representing two Ordinary Shares\",NYSE
JMM,Nuveen Multi-Market Income Fund (MA),NYSE
JNJ,Johnson & Johnson Common Stock,NYSE
JNPR,\"Juniper Networks, Inc. Common Stock\",NYSE
JOB,GEE Group Inc. Common Stock,NYSE MKT
JOBY,\"Joby Aviation, Inc. Common Stock\",NYSE
JOBY.W,\"Joby Aviation, Inc. Warrants\",NYSE
JOE,St. Joe Company (The) Common Stock,NYSE
JOF,Japan Smaller Capitalization Fund Inc Common Stock,NYSE
JPC,Nuveen Preferred & Income Opportunities Fund,NYSE
JPI,Nuveen Preferred and Income Term Fund Common Shares of Beneficial Interest,NYSE
JPM,JP Morgan Chase & Co. Common Stock,NYSE
JPM$C,\"J P Morgan Chase & Co Depositary Shares, each representing a 1/400th interest in a share of 6.00% Non-Cumulative  Preferred Stock, Series EE\",NYSE
JPM$D,\"J P Morgan Chase & Co Depositary Shares, each representing a 1/400th  interest in a share of 5.75% Non-Cumulative  Preferred Stock, Series DD\",NYSE
JPM$J,\"J P Morgan Chase & Co Depositary Shares, each representing a 1/400th interest in a share of JPMorgan Chase & Co. 4.75% Non-Cumulative Preferred Stock, Series GG\",NYSE
JPM$K,\"J P Morgan Chase & Co Depositary Shares, each representing a 1/400th interest in a share of 4.55% Non-Cumulative Preferred Stock, Series JJ\",NYSE
JPM$L,\"J P Morgan Chase & Co Depositary Shares, each representing a 1/400th interest in a share of 4.625% Non-Cumulative Preferred Stock, Series LL\",NYSE
JPM$M,\"J P Morgan Chase & Co Depositary Shares, each representing a 1/400th interest in a share of 4.20% Non-Cumulative Preferred Stock, Series MM\",NYSE
JQC,Nuveen Credit Strategies Income Fund Shares of Beneficial Interest,NYSE
JRI,Nuveen Real Asset Income and Growth Fund Common Shares of Beneficial Interest,NYSE
JRS,Nuveen Real Estate Income Fund Common Shares of Beneficial Interest,NYSE
JWN,\"Nordstrom, Inc. Common Stock\",NYSE
JWSM,Jaws Mustang Acquisition Corp. Class A Ordinary Shares,NYSE MKT
JWSM.U,\"Jaws Mustang Acquisition Corp. Units, each consisting of one Class A ordinary share, and one-fourth of one redeemable warrant\",NYSE MKT
JWSM.W,\"Jaws Mustang Acquisition Corp. Redeemable Warrants, each whole warrant exercisable for one Class A ordinary share at an exercise price of $11.50\",NYSE MKT
JXN,Jackson Financial Inc. Class A Common Stock ,NYSE
JXN$A,\"Jackson Financial Inc. Depositary Shares, each representing a 1/1,000th interest in a share of Fixed-Rate Reset Noncumulative Perpetual Preferred Stock, Series A\",NYSE
K,Kellanova Common Stock,NYSE
KAI,Kadant Inc Common Stock,NYSE
KAR,\"OPENLANE, Inc. Common Stock\",NYSE
KB,KB Financial Group Inc,NYSE
KBDC,\"Kayne Anderson BDC, Inc. Common Stock\",NYSE
KBH,KB Home Common Stock,NYSE
KBR,\"KBR, Inc. Common Stock\",NYSE
KCGI,Kensington Capital Acquisition Corp. V Class A Ordinary Shares,NYSE
KCGI.U,\"Kensington Capital Acquisition Corp. V Units, each consisting of one Class A ordinary share and three-fourths of one redeemable warrant\",NYSE
KCGI.W,\"Kensington Capital Acquisition Corp. V Redeemable warrants, each whole warrant exercisable for one Class A ordinary share at an exercise price of $11.50\",NYSE
KD,\"Kyndryl Holdings, Inc. Common Stock\",NYSE
KEN,Kenon Holdings Ltd. Ordinary Shares,NYSE
KEP,Korea Electric Power Corporation Common Stock,NYSE
KEX,Kirby Corporation Common Stock,NYSE
KEY,KeyCorp Common Stock,NYSE
KEY$I,\"KeyCorp Depositary Shares Each Representing a 1/40th Ownership Interest in a Share of Fixed-to-Floating Rate Perpetual Non-Cumulative Preferred Stock, Series E\",NYSE
KEY$J,\"KeyCorp Depositary Shares each representing a 1/40th ownership interest in a share of Fixed Rate Perpetual Non-Cumulative Preferred Stock, Series F\",NYSE
KEY$K,\"KeyCorp Depositary Shares, each representing a 1/40th ownership interest in a share of Fixed Rate Perpetual Non-Cumulative Preferred Stock, Series G\",NYSE
KEY$L,\"KeyCorp Depositary Shares each representing a 1/40th ownership interest in a share of Fixed Rate Perpetual Non-Cumulative Preferred Stock, Series H\",NYSE
KEYS,Keysight Technologies Inc. Common Stock,NYSE
KF,\"Korea Fund, Inc. (The) New Common Stock\",NYSE
KFRC,\"Kforce, Inc. Common Stock\",NYSE
KFS,\"Kingsway Financial Services, Inc. Common Stock (DE)\",NYSE
KFY,Korn Ferry Common Stock,NYSE
KGC,Kinross Gold Corporation Common Stock,NYSE
KGS,\"Kodiak Gas Services, Inc. Common Stock\",NYSE
KIM,Kimco Realty Corporation (HC) Common Stock,NYSE
KIM$L,\"Kimco Realty Corporation Class L Depositary Shares, each of which represents a one-one thousandth fractional interest in a share of 5.125% Class L Cumulative Redeemable Preferred Stock, liquidation preference $25,000.00 per share\",NYSE
KIM$M,\"Kimco Realty Corporation Class M Depositary Shares, each of which represents a one-one thousandth fractional interest in a share of 5.25% Class M Cumulative Redeemable Preferred Stock, liquidation preference $25,000.00 per share\",NYSE
KIM$N,\"Kimco Realty Corporation Depositary Shares, each representing 1/1,000th interest in a share of 7.25% Class N Cumulative Convertible Perpetual Preferred Stock\",NYSE
KIND,\"Nextdoor Holdings, Inc. Class A Common Stock\",NYSE
KIO,KKR Income Opportunities Fund Common Shares,NYSE
KKR,KKR & Co. Inc. Common Stock,NYSE
KKRS,KKR Group Finance Co. IX LLC 4.625% Subordinated Notes due 2061,NYSE
KLG,WK Kellogg Co Common Stock,NYSE
KMB,Kimberly-Clark Corporation Common Stock,NYSE
KMI,\"Kinder Morgan, Inc. Common Stock\",NYSE
KMPB,Kemper Corporation 5.875% Fixed-Rate Reset Junior Subordinated Debentures due 2062,NYSE
KMPR,Kemper Corporation,NYSE
KMT,Kennametal Inc. Common Stock,NYSE
KMX,CarMax Inc,NYSE
KN,Knowles Corporation Common Stock,NYSE
KNF,Knife Riv Holding Co. Common Stock,NYSE
KNOP,KNOT Offshore Partners LP Common Units representing Limited Partner Interests,NYSE
KNSL,\"Kinsale Capital Group, Inc. Common Stock\",NYSE
KNTK,Kinetik Holdings Inc. Class A Common Stock,NYSE
KNW,\"Know Labs, Inc. Common Stock\",NYSE MKT
KNX,Knight-Swift Transportation Holdings Inc.,NYSE
KO,Coca-Cola Company (The) Common Stock,NYSE
KODK,Eastman Kodak Company Common New,NYSE
KOF,\"Coca Cola Femsa S.A.B. de C.V.  American Depositary Shares, each representing 10 Units (each Unit consists of 3 Series B Shares and 5 Series L Shares)\",NYSE
KOP,Koppers Holdings Inc. Common Stock,NYSE
KORE,\"KORE Group Holdings, Inc. Common Stock\",NYSE
KOS,Kosmos Energy Ltd. Common Shares (DE),NYSE
KR,Kroger Company (The) Common Stock,NYSE
KRC,Kilroy Realty Corporation Common Stock,NYSE
KREF,KKR Real Estate Finance Trust Inc. Common Stock,NYSE
KREF$A,KKR Real Estate Finance Trust Inc. 6.50% Series A Cumulative Redeemable Preferred Stock,NYSE
KRG,Kite Realty Group Trust Common Stock,NYSE
KRO,Kronos Worldwide Inc Common Stock,NYSE
KRP,Kimbell Royalty Partners Common Units Representing Limited Partner Interests,NYSE
KSM,DWS Strategic Municipal Income Trust,NYSE
KSS,Kohl's Corporation Common Stock,NYSE
KT,KT Corporation Common Stock,NYSE
KTB,\"Kontoor Brands, Inc. Common Stock \",NYSE
KTF,DWS Municipal Income Trust,NYSE
KTH,Structures Products Cp 8% CorTS Issued by Peco Energy Cap Tr II Preferred Stock,NYSE
KTN,Structured Products Corp 8.205% CorTS 8.205% Corporate Backed Trust Securities (CorTS),NYSE
KUKE,\"Kuke Music Holding Limited American Depositary Shares, each representing one Ordinary Share\",NYSE
KULR,\"KULR Technology Group, Inc. Common Stock\",NYSE MKT
KVUE,Kenvue Inc. Common Stock,NYSE
KVYO,\"Klaviyo, Inc. Series A Common Stock\",NYSE
KW,Kennedy-Wilson Holdings Inc. Common Stock,NYSE
KWR,Quaker Houghton Common Stock,NYSE
KYN,\"Kayne Anderson Energy Infrastructure Fund, Inc.\",NYSE
L,Loews Corporation Common Stock,NYSE
LAAC,Lithium Americas (Argentina) Corp. Common Shares,NYSE
LAC,Lithium Americas Corp. Common Shares,NYSE
LAD,\"Lithia Motors, Inc. Common Stock\",NYSE
LADR,Ladder Capital Corp Class A Common Stock,NYSE
LANV,Lanvin Group Holdings Limited Ordinary Shares,NYSE
LANV.W,\"Lanvin Group Holdings Limited Redeemable Warrants, each whole warrant exercisable for one Ordinary Share at an exercise price of $11.50\",NYSE
LAW,\"CS Disco, Inc. Common Stock\",NYSE
LAZ,\"Lazard, Inc. Common Stock\",NYSE
LBRT,Liberty Energy Inc. Class A common stock,NYSE
LC,LendingClub Corporation Common Stock,NYSE
LCII,LCI Industries,NYSE
LCTX,\"Lineage Cell Therapeutics, Inc. Common Stock\",NYSE MKT
LCW,Learn CW Investment Corporation Class A Ordinary Shares,NYSE
LCW.U,\"Learn CW Investment Corporation Units, each consisting of one Class A ordinary share and one-half of one redeemable warrant\",NYSE
LCW.W,\"Learn CW Investment Corporation Redeemable warrants, each whole warrant exercisable for one Class A ordinary share at an exercise price of $11.50 per share\",NYSE
LDI,\"loanDepot, Inc. Class A Common Stock\",NYSE
LDOS,\"Leidos Holdings, Inc. Common Stock\",NYSE
LDP,\"Cohen & Steers Limited Duration Preferred and Income Fund, Inc.\",NYSE
LEA,Lear Corporation Common Stock,NYSE
LEG,\"Leggett & Platt, Incorporated Common Stock\",NYSE
LEGT,Legato Merger Corp. III Ordinary Shares,NYSE MKT
LEGT.U,\"Legato Merger Corp. III Units, each consisting of one Ordinary Share and one-half of one warrant\",NYSE MKT
LEGT.W,\"Legato Merger Corp. III Redeemable Warrants, each whole warrant exercisable for one ordinary share at an exercise price of $11.50 per share\",NYSE MKT
LEN,Lennar Corporation Class A Common Stock,NYSE
LEN.B,Lennar Corporation Class B,NYSE
LEO,\"BNY Mellon Strategic Municipals, Inc. Common Stock\",NYSE
LEU,Centrus Energy Corp. Class A Common Stock,NYSE MKT
LEV,The Lion Electric Company Common Shares,NYSE
LEV.A,The Lion Electric Company Warrants to purchase common shares,NYSE
LEV.W,\"The Lion Electric Company Redeemable Warrants, each whole warrant entitles the holder to purchase one Common Share at a price of $11.50 per share\",NYSE
LEVI,Levi Strauss & Co Class A Common Stock,NYSE
LFT,\"Lument Finance Trust, Inc. Common Stock\",NYSE
LFT$A,\"Lument Finance Trust, Inc. 7.875% Series A Cumulative Redeemable Preferred Stock\",NYSE
LGF.A,Lions Gate Entertainment Corporation Class A Voting Shares,NYSE
LGF.B,Lions Gate Entertainment Corporation Class B Non-Voting Shares,NYSE
LGI,Lazard Global Total Return and Income Fund Common Stock,NYSE
LGL,\"LGL Group, Inc. (The) Common Stock\",NYSE MKT
LGL.W,\"LGL Group, Inc. (The) Warrants\",NYSE MKT
LH,Labcorp Holdings Inc. Common Stock,NYSE
LHX,\"L3Harris Technologies, Inc. Common Stock\",NYSE
LICY,Li-Cycle Holdings Corp. Common Shares,NYSE
LII,\"Lennox International, Inc. Common Stock\",NYSE
LITB,\"LightInTheBox Holding Co., Ltd. American Depositary Shares, each representing 2 ordinary shares\",NYSE
LL,\"LL Flooring Holdings, Inc. Common Stock\",NYSE
LLAP,Terran Orbital Corporation Common Stock,NYSE
LLAP.W,\"Terran Orbital Corporation Redeemable Warrants, each whole warrant exercisable for one share of Common Stock at an exercise price of $11.50\",NYSE
LLY,Eli Lilly and Company Common Stock,NYSE
LMND,\"Lemonade, Inc. Common Stock\",NYSE
LMND.W,\"Lemonade, Inc. Warrants\",NYSE MKT
LMT,Lockheed Martin Corporation Common Stock,NYSE
LNC,Lincoln National Corporation Common Stock,NYSE
LNC$D,\"Lincoln National Corporation Depositary Shares Each Representing a 1/1,000th Interest in a Share of 9.000% Non-Cumulative Preferred Stock, Series D\",NYSE
LND,Brasilagro Brazilian Agric Real Estate Co Sponsored ADR (Brazil),NYSE
LNG,\"Cheniere Energy, Inc. Common Stock\",NYSE
LNN,Lindsay Corporation Common Stock,NYSE
LOAR,Loar Holdings Inc. Common Stock,NYSE
LOB,\"Live Oak Bancshares, Inc. Common Stock\",NYSE
LOCL,Local Bounti Corporation Common Stock,NYSE
LODE,Comstock Inc. Common Stock,NYSE MKT
LOMA,Loma Negra Compania Industrial Argentina Sociedad Anonima ADS,NYSE
LOW,\"Lowe's Companies, Inc. Common Stock\",NYSE
LPA,Logistic Properties of the Americas Ordinary Shares,NYSE MKT
LPG,Dorian LPG Ltd. Common Stock,NYSE
LPL,\"LG Display Co, Ltd AMERICAN DEPOSITORY SHARES\",NYSE
LPTV,\"Loop Media, Inc. Common Stock\",NYSE MKT
LPX,Louisiana-Pacific Corporation Common Stock,NYSE
LRN,\"Stride, Inc. Common Stock\",NYSE
LSF,\"Laird Superfood, Inc. Common Stock\",NYSE MKT
LSPD,Lightspeed Commerce Inc. Subordinate Voting Shares,NYSE
LTC,\"LTC Properties, Inc. Common Stock\",NYSE
LTH,\"Life Time Group Holdings, Inc. Common Stock\",NYSE
LU,\"Lufax Holding Ltd American Depositary Shares, each representing two (2) Ordinary Shares\",NYSE
LUMN,\"Lumen Technologies, Inc. Common Stock\",NYSE
LUV,Southwest Airlines Company Common Stock,NYSE
LVS,Las Vegas Sands Corp. Common Stock,NYSE
LVWR,\"LiveWire Group, Inc. Common Stock\",NYSE
LVWR.W,\"LiveWire Group, Inc. Warrants, each exercisable for one share of Common Stock at an exercise price of $11.50 per share\",NYSE
LW,\"Lamb Weston Holdings, Inc. Common Stock \",NYSE
LXFR,Luxfer Holdings PLC Ordinary Shares,NYSE
LXP,LXP Industrial Trust Common Stock (Maryland REIT),NYSE
LXP$C,LXP Industrial Trust 6.5% Series C Cumulative Convertible Preferred Stock,NYSE
LXU,\"LSB Industries, Inc. Common Stock\",NYSE
LYB,LyondellBasell Industries NV Ordinary Shares Class A (Netherlands),NYSE
LYG,Lloyds Banking Group Plc American Depositary Shares,NYSE
LYV,\"Live Nation Entertainment, Inc. Common Stock\",NYSE
LZB,La-Z-Boy Incorporated Common Stock,NYSE
LZM,Lifezone Metals Limited Ordinary Shares,NYSE
LZM.W,Lifezone Metals Limited Warrants,NYSE
M,Macy's Inc Common Stock,NYSE
MA,Mastercard Incorporated Common Stock,NYSE
MAA,\"Mid-America Apartment Communities, Inc. Common Stock\",NYSE
MAA$I,\"Mid-America Apartment Communities, Inc. 8.50% Series I Cumulative Redeemable Preferred Stock\",NYSE
MAC,Macerich Company (The) Common Stock,NYSE
MAG,MAG Silver Corporation Ordinary Shares,NYSE MKT
MAIA,\"MAIA Biotechnology, Inc. Common Stock\",NYSE MKT
MAIN,Main Street Capital Corporation Common Stock,NYSE
MAN,ManpowerGroup Common Stock,NYSE
MANU,Manchester United Ltd. Class A Ordinary Shares,NYSE
MAS,Masco Corporation Common Stock,NYSE
MATV,\"Mativ Holdings, Inc. Common Stock\",NYSE
MATX,\"Matson, Inc. Common Stock\",NYSE
MAV,\"Pioneer Municipal High Income Advantage Fund, Inc.\",NYSE
MAX,\"MediaAlpha, Inc. Class A Common Stock\",NYSE
MBC,\"MasterBrand, Inc. Common Stock\",NYSE
MBI,MBIA Inc. Common Stock,NYSE
MC,Moelis & Company Class A Common Stock,NYSE
MCB,Metropolitan Bank Holding Corp. Common Stock,NYSE
MCD,McDonald's Corporation Common Stock,NYSE
MCI,Barings Corporate Investors Common Stock,NYSE
MCK,McKesson Corporation Common Stock,NYSE
MCN,Madison Covered Call & Equity Strategy Fund Common Stock,NYSE
MCO,Moody's Corporation Common Stock,NYSE
MCR,MFS Charter Income Trust Common Stock,NYSE
MCS,Marcus Corporation (The) Common Stock,NYSE
MCW,\"Mister Car Wash, Inc. Common Stock\",NYSE
MCY,Mercury General Corporation Common Stock,NYSE
MD,\"Pediatrix Medical Group, Inc. Common Stock\",NYSE
MDT,Medtronic plc. Ordinary Shares,NYSE
MDU,\"MDU Resources Group, Inc. Common Stock (Holding Company)\",NYSE
MDV,\"Modiv Industrial, Inc. Class C Common Stock\",NYSE
MDV$A,\"Modiv Industrial, Inc. 7.375% Series A Cumulative Redeemable Perpetual Preferred Stock\",NYSE
MEC,\"Mayville Engineering Company, Inc. Common Stock\",NYSE
MED,MEDIFAST INC Common Stock,NYSE
MEG,\"Montrose Environmental Group, Inc. Common Stock\",NYSE
MEGI,MainStay CBRE Global Infrastructure Megatrends Term Fund Common Shares,NYSE
MEI,\"Methode Electronics, Inc. Common Stock\",NYSE
MER$K,\"Bank of America Corporation Income Capital Obligation Notes initially due December 15, 2066\",NYSE
MET,\"MetLife, Inc. Common Stock\",NYSE
MET$A,\"MetLife, Inc. Preferred Series A Floating Rate\",NYSE
MET$E,\"MetLife, Inc. Depositary shares, each representing a 1/1000th interest in a share of the Issuera??s 5.625% Non-Cumulative Preferred Stock, Series E.\",NYSE
MET$F,\"MetLife, Inc. Depositary Shares, each representing a 1/1,000th interest in a share of 4.75% Non-Cumulative Preferred Stock, Series F\",NYSE
MFA,\"MFA Financial, Inc.\",NYSE
MFA$B,\"MFA Financial, Inc. Preferred Series B\",NYSE
MFA$C,\"MFA Financial, Inc. 6.50% Series C Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock\",NYSE
MFAN,\"MFA Financial, Inc. 8.875% Senior Notes due 2029\",NYSE
MFAO,\"MFA Financial, Inc. 9.000% Senior Notes due 2029\",NYSE
MFC,Manulife Financial Corporation Common Stock,NYSE
MFD,Macquarie First Trust Global Common Stock,NYSE
MFG,\"Mizuho Financial Group, Inc. Sponosred ADR (Japan)\",NYSE
MFM,MFS Municipal Income Trust Common Stock,NYSE
MG,Mistras Group Inc Common Stock,NYSE
MGA,\"Magna International, Inc. Common Stock\",NYSE
MGF,MFS Government Markets Income Trust Common Stock,NYSE
MGLD,\"The Marygold Companies, Inc. Common Stock\",NYSE MKT
MGM,MGM Resorts International Common Stock,NYSE
MGR,\"Affiliated Managers Group, Inc. 5.875% Junior Subordinated Notes due 2059\",NYSE
MGRB,\"Affiliated Managers Group, Inc. 4.750% Junior Subordinated Notes due 2060\",NYSE
MGRD,\"Affiliated Managers Group, Inc. 4.200% Junior Subordinated Notes due 2061\",NYSE
MGRE,\"Affiliated Managers Group, Inc. 6.750% Junior Subordinated Notes due 2064\",NYSE
MGY,Magnolia Oil & Gas Corporation Class A Common Stock,NYSE
MHD,\"Blackrock MuniHoldings Fund, Inc. Common Stock\",NYSE
MHF,\"Western Asset Municipal High Income Fund, Inc. Common Stock\",NYSE
MHH,\"Mastech Digital, Inc Common Stock\",NYSE MKT
MHI,\"Pioneer Municipal High Income Fund, Inc.\",NYSE
MHK,\"Mohawk Industries, Inc. Common Stock\",NYSE
MHLA,\"Maiden Holdings, Ltd. 6.625% Notes due 2046\",NYSE
MHN,\"Blackrock MuniHoldings New York Quality Fund, Inc. Common Stock\",NYSE
MHNC,\"Maiden Holdings North America, Ltd. 7.75% Notes due 2043\",NYSE
MHO,\"M/I Homes, Inc. Common Stock\",NYSE
MI,NFT Limited Class A Ordinary Share,NYSE MKT
MIN,MFS Intermediate Income Trust Common Stock,NYSE
MIO,\"Pioneer Municipal High Income Opportunities Fund, Inc. Common Stock\",NYSE
MIR,\"Mirion Technologies, Inc. Class A Common Stock\",NYSE
MITN,\"AG Mortgage Investment Trust, Inc. 9.500% Senior Notes due 2029\",NYSE
MITP,\"AG Mortgage Investment Trust, Inc. 9.500% Senior Notes due 2029\",NYSE
MITQ,\"Moving iMage Technologies, Inc. Common Stock\",NYSE MKT
MITT,\"AG Mortgage Investment Trust, Inc. Common Stock\",NYSE
MITT$A,\"AG Mortgage Investment Trust, Inc. 8.25% Preferred Series A\",NYSE
MITT$B,\"AG Mortgage Investment Trust, Inc. Preferred Series B\",NYSE
MITT$C,\"AG Mortgage Investment Trust, Inc. 8.00% Series C Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock, $0.01 par value per share\",NYSE
MIY,\"Blackrock MuniYield Michigan Quality Fund, Inc. Common Stock\",NYSE
MKC,\"McCormick & Company, Incorporated Common Stock\",NYSE
MKC.V,\"McCormick & Company, Incorporated Common Stock\",NYSE
MKFG,Markforged Holding Corporation Common Stock,NYSE
MKFG.W,Markforged Holding Corporation Warrants,NYSE
MKL,Markel Group Inc. Common Stock,NYSE
ML,MoneyLion Inc. Class A Common Stock,NYSE
ML.W,\"MoneyLion Inc. Warrants, each whole warrant exercisable for one share of Class A common stock at an exercise price of $11.50 per share\",NYSE
MLI,\"Mueller Industries, Inc. Common Stock\",NYSE
MLM,\"Martin Marietta Materials, Inc. Common Stock\",NYSE
MLNK,\"MeridianLink, Inc. Common Stock\",NYSE
MLP,\"Maui Land & Pineapple Company, Inc. Common Stock\",NYSE
MLPB,\"ETRACS Alerian MLP Infrastructure Index ETN Series B due April 2, 2040\",NYSE ARCA
MLR,\"Miller Industries, Inc. Common Stock\",NYSE
MLSS,\"Milestone Scientific, Inc. Common Stock\",NYSE MKT
MMA,Alta Global Group Limited Ordinary Shares,NYSE MKT
MMC,\"Marsh & McLennan Companies, Inc. Common Stock\",NYSE
MMD,MainStay MacKay DefinedTerm Municipal Opportunities Fund,NYSE
MMI,\"Marcus & Millichap, Inc. Common Stock\",NYSE
MMM,3M Company Common Stock,NYSE
MMS,\"Maximus, Inc. Common Stock\",NYSE
MMT,MFS Multimarket Income Trust Common Stock,NYSE
MMU,\"Western Asset Managed Municipals Fund, Inc. Common Stock\",NYSE
MNR,Mach Natural Resources LP Common Units representing Limited Partner Interests,NYSE
MNSO,\"MINISO Group Holding Limited American Depositary Shares, each representing four Ordinary Shares\",NYSE
MNTN,Everest Consolidator Acquisition Corporation Class A Common Stock,NYSE
MNTN.U,\"Everest Consolidator Acquisition Corporation Units, each consisting of one share of Class A common stock and one-half of one redeemable warrant\",NYSE
MNTN.W,\"Everest Consolidator Acquisition Corporation Warrants, each whole warrant exercisable for one share of Class A Common Stock, at an exercise price of $11.50\",NYSE
MO,\"Altria Group, Inc.\",NYSE
MOD,Modine Manufacturing Company Common Stock,NYSE
MODG,Topgolf Callaway Brands Corp. Common Stock,NYSE
MODN,\"Model N, Inc. Common Stock\",NYSE
MOG.A,Moog Inc. Class A Common Stock,NYSE
MOG.B,Moog Inc. Class B Common Stock,NYSE
MOGU,MOGU Inc. American Depositary Shares (each  representing 25 Class A Ordinary Shares),NYSE
MOH,Molina Healthcare Inc Common Stock,NYSE
MOS,Mosaic Company (The) Common Stock,NYSE
MOV,Movado Group Inc. Common Stock,NYSE
MP,MP Materials Corp. Common Stock,NYSE
MPA,Blackrock MuniYield Pennsylvania Quality Fund Common Stock,NYSE
MPC,Marathon Petroleum Corporation Common Stock,NYSE
MPLN,MultiPlan Corporation Class A Common Stock,NYSE
MPLX,MPLX LP Common Units Representing Limited Partner Interests,NYSE
MPTI,\"M-tron Industries, Inc. Common Stock\",NYSE MKT
MPU,Mega Matrix Corp. Common Stock,NYSE MKT
MPV,Barings Participation Investors Common Stock,NYSE
MPW,\"Medical Properties Trust, Inc. common stock\",NYSE
MPX,Marine Products Corporation Common Stock,NYSE
MQT,\"Blackrock MuniYield Quality Fund II, Inc. Common Stock\",NYSE
MQY,\"Blackrock MuniYield Quality Fund, Inc. Common Stock\",NYSE
MRC,MRC Global Inc. Common Stock,NYSE
MRDB,MariaDB plc Ordinary Shares,NYSE
MRDB.W,\"MariaDB plc Warrants, each whole warrant exercisable for one Ordinary Share at an exercise price of $11.50 per share\",NYSE
MRK,\"Merck & Company, Inc. Common Stock (new)\",NYSE
MRO,Marathon Oil Corporation Common Stock,NYSE
MRT,\"Marti Technologies, Inc. Class A Ordinary Shares\",NYSE MKT
MS,Morgan Stanley Common Stock,NYSE
MS$A,Morgan Stanley Dep Shs repstg 1/1000 Pfd Ser A,NYSE
MS$E,Morgan Stanley DEPOSITARY SHARES REP 1/1000TH SHARES FIXED/FLTG PREFERRED STOCK SERIES E,NYSE
MS$F,Morgan Stanley Dep Shs Rpstg 1/1000th Int Prd Ser F Fxd to Flag,NYSE
MS$I,Morgan Stanley Depository Shares Representing 1/1000th Preferred Series 1 Fixed to Floating Non (Cum),NYSE
MS$K,\"Morgan Stanley Depositary Shares, each representing 1/1,000th of a share of Fixed-to-Floating Rate Non-Cumulative Preferred Stock,  Series K\",NYSE
MS$L,\"Morgan Stanley Depositary Shares, each representing 1/1,000th of a share of 4.875% Non-Cumulative Preferred Stock, Series L\",NYSE
MS$O,\"Morgan Stanley Depositary Shares, each representing 1/1,000th of a share of 4.250% Non-Cumulative Preferred Stock, Series O\",NYSE
MS$P,\"Morgan Stanley Depositary Shares, each representing 1/1,000th of a share of 6.500% Non-Cumulative Preferred Stock, Series P\",NYSE
MSA,MSA Safety Incorporated Common Stock,NYSE
MSB,Mesabi Trust Common Stock,NYSE
MSC,\"Studio City International Holdings Limited American depositary shares, each representing four  Class A ordinary shares\",NYSE
MSCI,MSCI Inc. Common Stock,NYSE
MSD,\"Morgan Stanley Emerging Markets Debt Fund, Inc. Common Stock\",NYSE
MSDL,Morgan Stanley Direct Lending Fund Common Stock,NYSE
MSGE,Madison Square Garden Entertainment Corp. Class A Common Stock,NYSE
MSGS,Madison Square Garden Sports Corp. Class A Common Stock (New),NYSE
MSI,\"Motorola Solutions, Inc. Common Stock\",NYSE
MSM,\"MSC Industrial Direct Company, Inc. Common Stock\",NYSE
MSN,Emerson Radio Corporation Common Stock,NYSE MKT
MT,Arcelor Mittal NY Registry Shares NEW,NYSE
MTA,Metalla Royalty & Streaming Ltd. Common Shares,NYSE MKT
MTAL,Metals Acquisition Limited Ordinary Shares,NYSE
MTAL.W,Metals Acquisition Limited Warrants,NYSE
MTB,M&T Bank Corporation Common Stock,NYSE
MTB$H,\"M&T Bank Corporation Perpetual Fixed-to-Floating Rate Non-Cumulative Preferred Stock, Series H\",NYSE
MTB$J,\"M&T Bank Corporation Depositary Shares each representing a 1/400th ownership interest in a share of Perpetual 7.500% Non-Cumulative Preferred Stock, Series J\",NYSE
MTD,\"Mettler-Toledo International, Inc. Common Stock\",NYSE
MTDR,Matador Resources Company Common Stock,NYSE
MTG,MGIC Investment Corporation Common Stock,NYSE
MTH,Meritage Homes Corporation Common Stock,NYSE
MTN,\"Vail Resorts, Inc. Common Stock\",NYSE
MTNB,\"Matinas Biopharma Holdings, Inc. Common Stock\",NYSE MKT
MTR,Mesa Royalty Trust Common Stock,NYSE
MTRN,Materion Corporation,NYSE
MTUS,Metallus Inc. Common Shares,NYSE
MTW,\"Manitowoc Company, Inc. (The) Common Stock\",NYSE
MTX,Minerals Technologies Inc. Common Stock,NYSE
MTZ,\"MasTec, Inc. Common Stock\",NYSE
MUA,\"Blackrock MuniAssets Fund, Inc Common Stock\",NYSE
MUC,\"Blackrock MuniHoldings California Quality Fund, Inc.  Common Stock\",NYSE
MUE,\"Blackrock MuniHoldings Quality Fund II, Inc. Common Stock\",NYSE
MUFG,\"Mitsubishi UFJ Financial Group, Inc. Common Stock\",NYSE
MUI,\"BlackRock Municipal Income Fund, Inc. Common Stock\",NYSE
MUJ,\"Blackrock MuniHoldings New Jersey Quality Fund, Inc. Common Stock\",NYSE
MUR,Murphy Oil Corporation Common Stock,NYSE
MUSA,Murphy USA Inc. Common Stock,NYSE
MUX,McEwen Mining Inc. Common Stock,NYSE
MVF,\"Blackrock MuniVest Fund, Inc. Common Stock\",NYSE
MVO,MV Oil Trust Units of Beneficial Interests,NYSE
MVT,\"Blackrock MuniVest Fund II, Inc.  Common Stock\",NYSE
MWA,MUELLER WATER PRODUCTS Common Stock,NYSE
MWG,Multi Ways Holdings Limited Ordinary Shares,NYSE MKT
MX,Magnachip Semiconductor Corporation Common Stock,NYSE
MXC,Mexco Energy Corporation Common Stock,NYSE MKT
MXE,\"Mexico Equity and Income Fund, Inc. (The) Common Stock\",NYSE
MXF,\"Mexico Fund, Inc. (The) Common Stock\",NYSE
MYD,\"Blackrock MuniYield Fund, Inc.  Common Stock\",NYSE
MYE,\"Myers Industries, Inc. Common Stock\",NYSE
MYI,\"Blackrock MuniYield Quality Fund III, Inc Common Stock\",NYSE
MYN,\"Blackrock MuniYield New York Quality Fund, Inc.Common Stock\",NYSE
MYND,\"Mynd.ai, Inc. American Depositary Shares\",NYSE MKT
MYO,Myomo Inc. Common Stock,NYSE MKT
MYTE,\"MYT Netherlands Parent B.V. American Depositary Shares, each representing one Ordinary Share\",NYSE
NABL,\"N-able, Inc. Common Stock\",NYSE
NAC,Nuveen California Quality Municipal Income Fund,NYSE
NAD,Nuveen Quality Municipal Income Fund Common Stock,NYSE
NAK,\"Northern Dynasty Minerals, Ltd. Common Stock\",NYSE MKT
NAN,Nuveen New York Quality Municipal Income Fund Common Stock,NYSE
NAPA,\"The Duckhorn Portfolio, Inc. Common Stock\",NYSE
NAT,Nordic American Tankers Limited Common Stock,NYSE
NATL,NCR Atleos Corporation Common Stock,NYSE
NAZ,Nuveen Arizona Quality Municipal Income Fund Common Stock,NYSE
NBB,Nuveen Taxable Municipal Income Fund Common Shares of Beneficial Interest,NYSE
NBH,Neuberger Berman Municipal Fund Inc. Common Stock,NYSE MKT
NBHC,National Bank Holdings Corporation Common Stock,NYSE
NBR,Nabors Industries Ltd.,NYSE
NBXG,Neuberger Berman Next Generation Connectivity Fund Inc. Common Stock,NYSE
NBY,\"NovaBay Pharmaceuticals, Inc. Common Stock\",NYSE MKT
NC,\"NACCO Industries, Inc. Common Stock\",NYSE
NCA,Nuveen California Municipal Value Fund,NYSE
NCDL,Nuveen Churchill Direct Lending Corp. Common Stock,NYSE
NCL,Northann Corp. Common Stock,NYSE MKT
NCLH,Norwegian Cruise Line Holdings Ltd. Ordinary Shares,NYSE
NCV,Virtus Convertible & Income Fund Common Shares of Beneficial Interest,NYSE
NCV$A,Virtus Convertible & Income Fund 5.625% Series A Cumulative Preferred Shares,NYSE
NCZ,Virtus Convertible & Income Fund II Common Shares of Beneficial Interest,NYSE
NCZ$A,Virtus Convertible & Income Fund II 5.50% Series A Cumulative Preferred Shares,NYSE
NDMO,Nuveen Dynamic Municipal Opportunities Fund Common Shares of Beneficial Interest,NYSE
NDP,\"Tortoise Energy Independence Fund, Inc. Common Stock\",NYSE
NE,Noble Corporation plc A Ordinary Shares,NYSE
NE.A,Noble Corporation plc Tranche 2 Warrants,NYSE
NE.W,Noble Corporation plc Tranche 1 Warrants,NYSE
NEA,Nuveen AMT-Free Quality Municipal Income Fund Common Shares of Beneficial Interest Par Value $.01,NYSE
NEE,\"NextEra Energy, Inc. Common Stock\",NYSE
NEE$N,\"NextEra Energy, Inc. Series N Junior Subordinated Debentures due March 1, 2079\",NYSE
NEE$R,\"NextEra Energy, Inc. 6.926% Corporate Units\",NYSE
NEM,Newmont Corporation,NYSE
NEN,New England Realty Associates Limited Partnership Class A Depositary Receipts Evidencing Units of Limited Partnership,NYSE MKT
NEP,\"NextEra Energy Partners, LP Common Units representing limited partner interests\",NYSE
NET,\"Cloudflare, Inc. Class A Common Stock\",NYSE
NEU,NewMarket Corp Common Stock,NYSE
NEUE,\"NeueHealth, Inc. Common Stock\",NYSE
NEWP,New Pacific Metals Corp. Common Shares,NYSE MKT
NEXA,Nexa Resources S.A. Common Shares,NYSE
NFG,National Fuel Gas Company Common Stock,NYSE
NFGC,New Found Gold Corp Common Shares,NYSE MKT
NFJ,\"Virtus Dividend, Interest & Premium Strategy Fund Common Shares of Beneficial Interest\",NYSE
NFYS,Enphys Acquisition Corp. Class A Ordinary Shares,NYSE
NFYS.U,\"Enphys Acquisition Corp. Units, each consisting of one Class A ordinary share and one-half of one redeemable warrant\",NYSE
NFYS.W,\"Enphys Acquisition Corp. Redeemable Warrants, each whole warrant exercisable for one Class A ordinary share at an exercise price of $11.50\",NYSE
NG,Novagold Resources Inc.,NYSE MKT
NGD,New Gold Inc.,NYSE MKT
NGG,\"National Grid Transco, PLC National Grid PLC (NEW) American Depositary Shares\",NYSE
NGL,NGL ENERGY PARTNERS LP Common Units representing Limited Partner Interests,NYSE
NGL$B,NGL ENERGY PARTNERS LP 9.00% Class B Fixed-to-Floating Rate Cumulative Redeemable Perpetual Preferred Units representing limited partnership interests,NYSE
NGL$C,NGL ENERGY PARTNERS LP 9.625% Class C Fixed-to-Floating Rate Cumulative  Redeemable Perpetual Preferred Units representing  limited partner interests,NYSE
NGS,\"Natural Gas Services Group, Inc. Common Stock\",NYSE
NGVC,\"Natural Grocers by Vitamin Cottage, Inc. Common Stock\",NYSE
NGVT,Ingevity Corporation Common Stock ,NYSE
NHC,National HealthCare Corporation Common Stock,NYSE MKT
NHI,\"National Health Investors, Inc. Common Stock\",NYSE
NHS,Neuberger Berman High Yield Strategies Fund,NYSE MKT
NI,NiSource Inc Common Stock,NYSE
NIC,Nicolet Bankshares Inc. Common Stock,NYSE
NIE,Virtus Equity & Convertible Income Fund Common Shares of Beneficial Interest,NYSE
NIM,Nuveen Select Maturities Municipal Fund Common Stock,NYSE
NINE,\"Nine Energy Service, Inc. Common Stock\",NYSE
NIO,\"NIO Inc. American depositary shares, each  representing one Class A ordinary share\",NYSE
NJR,NewJersey Resources Corporation Common Stock,NYSE
NKE,\"Nike, Inc. Common Stock\",NYSE
NKX,Nuveen California AMT-Free Quality Municipal Income Fund,NYSE
NL,\"NL Industries, Inc. Common Stock\",NYSE
NLOP,Net Lease Office Properties Common Shares of Beneficial Interest,NYSE
NLY,Annaly Capital Management Inc. Common Stock,NYSE
NLY$F,Annaly Capital Management Inc 6.95% Series F,NYSE
NLY$G,Annaly Capital Management Inc 6.50% Series G Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock,NYSE
NLY$I,Annaly Capital Management Inc 6.750% Series I Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock,NYSE
NMAI,Nuveen Multi-Asset Income Fund Common Shares of Beneficial Interest,NYSE
NMCO,Nuveen Municipal Credit Opportunities Fund Common Shares,NYSE
NMG,Nouveau Monde Graphite Inc. Common Shares,NYSE
NMI,\"Nuveen Municipal Income Fund, Inc. Common Stock\",NYSE
NML,Neuberger Berman Energy Infrastructure and Income Fund Inc. Common Stock,NYSE MKT
NMM,Navios Maritime Partners LP Common Units Representing Limited Partner Interests,NYSE
NMR,Nomura Holdings Inc ADR American Depositary Shares,NYSE
NMS,Nuveen Minnesota Quality Municipal Income Fund ,NYSE
NMT,Nuveen Massachusetts Quality Municipal Income Fund Common Stock,NYSE
NMZ,\"Nuveen Municipal High Income Opportunity Fund Common Stock, $0.01 par value, per share\",NYSE
NNI,\"Nelnet, Inc. Common Stock\",NYSE
NNN,\"NNN REIT, Inc. Common Stock\",NYSE
NNVC,\"NanoViricides, Inc. Common Stock\",NYSE MKT
NNY,Nuveen New York Municipal Value Fund Common Stock,NYSE
NOA,North American Construction Group Ltd. Common Shares (no par),NYSE
NOAH,Noah Holdings Limited American Depositary Shares,NYSE
NOC,Northrop Grumman Corporation Common Stock,NYSE
NOG,\"Northern Oil and Gas, Inc. Common Stock\",NYSE
NOK,Nokia Corporation Sponsored American Depositary Shares,NYSE
NOM,Nuveen Missouri Quality Municipal Income Fund ,NYSE
NOMD,Nomad Foods Limited Ordinary Shares,NYSE
NOTE,\"FiscalNote Holdings, Inc. Class A common stock\",NYSE
NOTE.W,\"FiscalNote Holdings, Inc. Warrants to purchase one share of Class A common stock, each at an exercise price of $11.50 per share\",NYSE
NOV,NOV Inc. Common Stock,NYSE
NOVA,Sunnova Energy International Inc. Common Stock,NYSE
NOW,\"ServiceNow, Inc. Common Stock\",NYSE
NPCT,Nuveen Core Plus Impact Fund Common Shares of Beneficial Interest,NYSE
NPFD,Nuveen Variable Rate Preferred & Income Fund Common Shares,NYSE
NPK,\"National Presto Industries, Inc. Common Stock\",NYSE
NPO,Enpro Inc. Common Stock,NYSE
NPV,Nuveen Virginia Quality Municipal Income Fund Common Stock,NYSE
NPWR,NET Power Inc. Class A Common Stock,NYSE
NPWR.W,\"NET Power Inc. Redeemable warrants, each whole warrant exercisable for one share of Class A Common Stock at an exercise price of $11.50\",NYSE
NQP,Nuveen Pennsylvania Quality Municipal Income Fund Common Stock,NYSE
NR,\"Newpark Resources, Inc. Common Stock\",NYSE
NRDY,Nerdy Inc. Class A Common Stock,NYSE
NREF,\"NexPoint Real Estate Finance, Inc. Common Stock\",NYSE
NREF$A,\"NexPoint Real Estate Finance, Inc. 8.50% Series A Cumulative Redeemable Preferred Stock\",NYSE
NRG,\"NRG Energy, Inc. Common Stock\",NYSE
NRGD,MicroSectors U.S. Big Oil Index -3X Inverse Leveraged ETN,NYSE ARCA
NRGU,MicroSectors U.S. Big Oil Index 3X Leveraged ETN,NYSE ARCA
NRGV,\"Energy Vault Holdings, Inc. Common Stock\",NYSE
NRK,Nuveen New York AMT-Free Quality Municipal Income Fund ,NYSE
NRO,\"Neuberger Berman Real Estate Securities Income Fund, Inc.\",NYSE MKT
NRP,Natural Resource Partners LP Limited Partnership,NYSE
NRT,North European Oil Royality Trust Common Stock,NYSE
NRUC,National Rural Utilities Cooperative Finance Corporation 5.500% Subordinated Notes due 2064 (Subordinated Deferrable Interest Notes),NYSE
NRXS,\"Neuraxis, Inc. Common Stock\",NYSE MKT
NS$A,Nustar Energy L.P. 8.50% Series A Fixed-to-Floating Rate Cumulative Redeemable Perpetual Preferred Units,NYSE
NS$B,Nustar Energy L.P. 7.625% Series B Fixed-to-Floating Rate Cumulative Redeemable Perpetual Preferred Units representing limited partner interests,NYSE
NS$C,Nustar Energy L.P. 9.00% Series C Fixed-to-Floating Rate Cumulative Redeemable Perpetual Preferred Units,NYSE
NSA,National Storage Affiliates Trust Common Shares of Beneficial Interest,NYSE
NSA$A,National Storage Affiliates Trust 6.000% Series A Cumulative Redeemable Preferred Shares of Beneficial Interest (Liquidation Preference $25.00 per share),NYSE
NSA$B,National Storage Affiliates Trust 6.000% Series B Cumulative Redeemable Preferred Shares of Beneficial Interest,NYSE
NSC,Norfolk Southern Corporation Common Stock,NYSE
NSP,\"Insperity, Inc. Common Stock\",NYSE
NSS,\"NuStar Logistics, L.P. 7.625% Fixed-to-Floating Rate Subordinated Notes due 2043\",NYSE
NTB,Bank of N.T. Butterfield & Son Limited (The) Voting Ordinary Shares,NYSE
NTEST,NYSE Tick Pilot Test Sym-Control,NYSE
NTEST.A,NYSE Tick Pilot Test Sym-G1,NYSE
NTEST.B,NYSE Tick Pilot Test Sym-G2,NYSE
NTEST.C,Tick Pilot Test C Common Stock,NYSE
NTG,\"Tortoise Midstream Energy Fund, Inc. Common Stock\",NYSE
NTIP,\"Network-1 Technologies, Inc. Common Stock\",NYSE MKT
NTR,Nutrien Ltd. Common Shares,NYSE
NTST,NetSTREIT Corp. Common Stock,NYSE
NTZ,\"Natuzzi, S.p.A.\",NYSE
NU,Nu Holdings Ltd. Class A Ordinary Shares,NYSE
NUE,Nucor Corporation Common Stock,NYSE
NUS,\"Nu Skin Enterprises, Inc. Common Stock\",NYSE
NUV,\"Nuveen Municipal Value Fund, Inc. Common Stock\",NYSE
NUVB,Nuvation Bio Inc. Class A Common Stock,NYSE
NUVB.W,Nuvation Bio Inc. Warrants,NYSE
NUW,Nuveen AMT-Free Municipal Value Fund,NYSE
NVG,Nuveen AMT-Free Municipal Credit Income Fund ,NYSE
NVGS,Navigator Holdings Ltd. Ordinary Shares (Marshall Islands),NYSE
NVO,Novo Nordisk A/S Common Stock,NYSE
NVR,\"NVR, Inc. Common Stock\",NYSE
NVRI,Enviri Corporation Common Stock,NYSE
NVRO,Nevro Corp. Common Stock,NYSE
NVS,Novartis AG Common Stock,NYSE
NVST,Envista Holdings Corporation Common Stock,NYSE
NVT,nVent Electric plc Ordinary Shares ,NYSE
NWG,\"NatWest Group plc American Depositary Shares, (each representing two (2) Ordinary Shares)\",NYSE
NWN,Northwest Natural Holding Company Common Stock,NYSE
NX,Quanex Building Products Corporation Common Stock,NYSE
NXC,Nuveen California Select Tax-Free Income Portfolio Common Stock,NYSE
NXDT,NexPoint Diversified Real Estate Trust Common Stock,NYSE
NXDT$A,NexPoint Diversified Real Estate Trust 5.50% Series A Cumulative Preferred Shares ($25.00 liquidation preference per share),NYSE
NXE,Nexgen Energy Ltd. Common Shares,NYSE
NXG,NXG NextGen Infrastructure Income Fund Common Shares of Beneficial Interest,NYSE
NXJ,Nuveen New Jersey Qualified Municipal Fund ,NYSE
NXN,Nuveen New York Select Tax-Free Income Portfolio Common Stock,NYSE
NXP,Nuveen Select Tax Free Income Portfolio Common Stock,NYSE
NXRT,\"NexPoint Residential Trust, Inc. Common Stock\",NYSE
NYC,American Strategic Investment Co. Class A Common Stock,NYSE
NYCB,\"New York Community Bancorp, Inc. Common Stock\",NYSE
NYCB$A,\"New York Community Bancorp, Inc. Depositary shares, each representing a 1/40th interest in a share of Fixed-to-Floating Rate Series A Noncumulative Perpetual Preferred Stock\",NYSE
NYCB$U,\"New York Community Bancorp, Inc. New York Community Capital Tr V (BONUSES)\",NYSE
NYT,New York Times Company (The) Common Stock,NYSE
NZF,Nuveen Municipal Credit Income Fund ,NYSE
O,Realty Income Corporation Common Stock,NYSE
O$,Realty Income Corporation 6.000% Series A Cumulative Redeemable Preferred Stock,NYSE
OAK$A,\"Brookfield Oaktree Holdings, LLC 6.625% Series A Preferred Units\",NYSE
OAK$B,\"Brookfield Oaktree Holdings, LLC 6.550% Series B Preferred Units\",NYSE
OBDC,Blue Owl Capital Corporation Common Stock,NYSE
OBDE,Blue Owl Capital Corporation III Common Stock,NYSE
OBE,Obsidian Energy Ltd. Common Shares,NYSE MKT
OBK,\"Origin Bancorp, Inc. Common Stock\",NYSE
OC,Owens Corning Inc Common Stock New,NYSE
OCFT,\"OneConnect Financial Technology Co., Ltd. American Depositary Shares, each representing thirty ordinary shares\",NYSE
OCN,Ocwen Financial Corporation NEW Common Stock,NYSE
ODC,Oil-Dri Corporation Of America Common Stock,NYSE
ODV,Osisko Development Corp. Common Shares,NYSE
OEC,Orion S.A. Common Shares,NYSE
OFG,OFG Bancorp Common Stock,NYSE
OGCP,\"Empire State Realty OP, L.P. Series 60 Operating Partnership Units Representing Limited Partnership Interests\",NYSE ARCA
OGE,OGE Energy Corp Common Stock,NYSE
OGEN,Oragenics Inc. Common Stock,NYSE MKT
OGN,Organon & Co. Common Stock ,NYSE
OGS,\"ONE Gas, Inc. Common Stock\",NYSE
OHI,\"Omega Healthcare Investors, Inc. Common Stock\",NYSE
OI,\"O-I Glass, Inc. Common Stock\",NYSE
OIA,Invesco Municipal Income Opportunities Trust Common Stock,NYSE
OII,\"Oceaneering International, Inc. Common Stock\",NYSE
OIS,\"Oil States International, Inc. Common Stock\",NYSE
OKE,\"ONEOK, Inc. Common Stock\",NYSE
OKLO,Oklo Inc. Class A common stock,NYSE
OLN,Olin Corporation Common Stock,NYSE
OLO,Olo Inc. Class A Common Stock,NYSE
OLP,\"One Liberty Properties, Inc. Common Stock\",NYSE
OMC,Omnicom Group Inc. Common Stock,NYSE
OMF,\"OneMain Holdings, Inc. Common Stock\",NYSE
OMI,\"Owens & Minor, Inc. Common Stock\",NYSE
ONL,Orion Office REIT Inc. Common Stock,NYSE
ONON,On Holding AG Class A Ordinary Shares,NYSE
ONTF,\"ON24, Inc. Common Stock\",NYSE
ONTO,Onto Innovation Inc. Common Stock,NYSE
OOMA,\"Ooma, Inc. Common Stock\",NYSE
OPAD,Offerpad Solutions Inc. Class A Common Stock,NYSE
OPFI,OppFi Inc. Class A Common Stock,NYSE
OPFI.W,OppFi Inc. Warrants,NYSE
OPP,\"RiverNorth/DoubleLine Strategic Opportunity Fund, Inc. Common Stock\",NYSE
OPP$A,\"RiverNorth/DoubleLine Strategic Opportunity Fund, Inc. 4.375% Series A Cumulative Preferred Stock\",NYSE
OPP$B,\"RiverNorth/DoubleLine Strategic Opportunity Fund, Inc. 4.75% Series B Cumulative Preferred Stock\",NYSE
OPTT,\"Ocean Power Technologies, Inc. Common Stock\",NYSE MKT
OPY,\"Oppenheimer Holdings, Inc. Class A Common Stock (DE)\",NYSE
OR,Osisko Gold Royalties Ltd Common Shares,NYSE
ORA,\"Ormat Technologies, Inc. Common Stock\",NYSE
ORAN,Orange,NYSE
ORC,\"Orchid Island Capital, Inc. Common Stock\",NYSE
ORCL,Oracle Corporation Common Stock,NYSE
ORI,Old Republic International Corporation Common Stock,NYSE
ORLA,Orla Mining Ltd. Common Shares,NYSE MKT
ORN,\"Orion Group Holdings, Inc. Common\",NYSE
OSCR,\"Oscar Health, Inc. Class A Common Stock\",NYSE
OSG,\"Overseas Shipholding Group, Inc. Class A Common Stock\",NYSE
OSK,Oshkosh Corporation (Holding Company)Common Stock,NYSE
OTIS,Otis Worldwide Corporation Common Stock ,NYSE
OUST,\"Ouster, Inc. Common Stock\",NYSE
OUST.A,\"Ouster, Inc. Warrants to purchase Common Stock\",NYSE MKT
OUST.W,\"Ouster, Inc. Warrants\",NYSE
OUT,OUTFRONT Media Inc. Common Stock,NYSE
OVV,Ovintiv Inc. (DE),NYSE
OWL,Blue Owl Capital Inc. Class A Common Stock,NYSE
OWLT,\"Owlet, Inc. Class A Common Stock\",NYSE
OXM,\"Oxford Industries, Inc. Common Stock\",NYSE
OXY,Occidental Petroleum Corporation Common Stock,NYSE
OXY.W,Occidental Petroleum Corporation Warrants ,NYSE
OZ,\"Belpointe PREP, LLC Class A Units\",NYSE MKT
PAAS,Pan American Silver Corp. Common Stock,NYSE
PAC,\"Grupo Aeroportuario Del Pacifico, S.A. B. de C.V. Grupo Aeroportuario Del Pacifico, S.A. de C.V. (each representing 10 Series B shares)\",NYSE
PACK,Ranpak Holdings Corp Class A Common Stock,NYSE
PACS,\"PACS Group, Inc. Common Stock\",NYSE
PAG,\"Penske Automotive Group, Inc. Common Stock\",NYSE
PAGS,PagSeguro Digital Ltd. Class A Common Shares,NYSE
PAI,Western Asset Investment Grade Income Fund Inc.,NYSE
PAM,Pampa Energia S.A.,NYSE
PAPL,Pineapple Financial Inc. Common Stock,NYSE MKT
PAR,PAR Technology Corporation Common Stock,NYSE
PARR,\"Par Pacific Holdings, Inc.  Common Stock\",NYSE
PATH,\"UiPath, Inc. Class A Common Stock\",NYSE
PAXS,PIMCO Access Income Fund Common Shares of Beneficial Interest,NYSE
PAY,\"Paymentus Holdings, Inc. Class A Common Stock\",NYSE
PAYC,\"Paycom Software, Inc. Common Stock\",NYSE
PB,\"Prosperity Bancshares, Inc. Common Stock\",NYSE
PBA,Pembina Pipeline Corp. Ordinary Shares (Canada),NYSE
PBF,PBF Energy Inc. Class A Common Stock,NYSE
PBH,Prestige Consumer Healthcare Inc. Common Stock,NYSE
PBI,Pitney Bowes Inc. Common Stock,NYSE
PBI$B,Pitney Bowes Inc 6.70% Notes Due 2043,NYSE
PBR,Petroleo Brasileiro S.A.- Petrobras Common Stock,NYSE
PBR.A,Petroleo Brasileiro S.A.- Petrobras American Depositary Shares,NYSE
PBT,Permian Basin Royalty Trust Common Stock,NYSE
PCF,High Income Securities Fund Common Stock,NYSE
PCG,Pacific Gas & Electric Co. Common Stock,NYSE
PCG$A,Pacific Gas & Electric Co. 6% Preferred Stock,NYSE MKT
PCG$B,Pacific Gas & Electric Co. 5 1/2% Preferred Stock,NYSE MKT
PCG$C,Pacific Gas & Electric Co. 5% 1st Preferred Stock,NYSE MKT
PCG$D,Pacific Gas & Electric Co. 5% 1st  Red. Preferred Stock,NYSE MKT
PCG$E,Pacific Gas & Electric Co. 5% 1st A Preferred Stock,NYSE MKT
PCG$G,Pacific Gas & Electric Co. 4.80% 1st Preferred Stock,NYSE MKT
PCG$H,Pacific Gas & Electric Co. 4.50% 1st Preferred Stock,NYSE MKT
PCG$I,Pacific Gas & Electric Co. 4.36% 1st Preferred Stock,NYSE MKT
PCK,Pimco California Municipal Income Fund II Common Shares of Beneficial Interest,NYSE
PCM,\"PCM Fund, Inc. Common Stock\",NYSE
PCN,Pimco Corporate & Income Strategy Fund Common Stock,NYSE
PCOR,\"Procore Technologies, Inc. Common Stock\",NYSE
PCQ,PIMCO California Municipal Income Fund Common Stock,NYSE
PD,\"PagerDuty, Inc. Common Stock\",NYSE
PDI,PIMCO Dynamic Income Fund Common Stock,NYSE
PDM,\"Piedmont Office Realty Trust, Inc. Class A Common Stock\",NYSE
PDO,PIMCO Dynamic Income Opportunities Fund Common Shares of Beneficial Interest,NYSE
PDS,Precision Drilling Corporation Common Stock,NYSE
PDT,John Hancock Premium Dividend Fund,NYSE
PDX,PIMCO Dynamic Income Strategy Fund Common Shares of Beneficial Interest,NYSE
PEB,Pebblebrook Hotel Trust Common Shares of Beneficial Interest,NYSE
PEB$E,Pebblebrook Hotel Trust 6.375% Series E Cumulative Redeemable Preferred Shares of Beneficial Interest,NYSE
PEB$F,Pebblebrook Hotel Trust 6.3% Series F Cumulative Redeemable Preferred Shares of Beneficial Interest,NYSE
PEB$G,Pebblebrook Hotel Trust 6.375% Series G Cumulative Redeemable Preferred Shares of Beneficial Interest,NYSE
PEB$H,Pebblebrook Hotel Trust 5.700% Series H Cumulative Redeemable Preferred Shares of Beneficial Interest,NYSE
PED,Pedevco Corp. Common Stock,NYSE MKT
PEG,Public Service Enterprise Group Incorporated Common Stock,NYSE
PEN,\"Penumbra, Inc. Common Stock\",NYSE
PEO,\"Adams Natural Resources Fund, Inc. Common Stock\",NYSE
PERF,Perfect Corp. Class A Ordinary Share,NYSE
PERF.W,\"Perfect Corp. Warrants, each exercisable for one Class A Ordinary Share at a price of $11.50 per share\",NYSE
PFD,Flaherty & Crumrine Preferred and Income Fund Incorporated,NYSE
PFE,\"Pfizer, Inc. Common Stock\",NYSE
PFFL,\"ETRACS 2xMonthly Pay Leveraged Preferred Stock Index ETN due September 25, 2048\",NYSE ARCA
PFGC,Performance Food Group Company Common Stock,NYSE
PFH,\"Prudential Financial, Inc. 4.125% Junior Subordinated Notes due 2060\",NYSE
PFL,PIMCO Income Strategy Fund Shares of Beneficial Interest,NYSE
PFLT,PennantPark Floating Rate Capital Ltd. Common Stock,NYSE
PFN,PIMCO Income Strategy Fund II,NYSE
PFO,Flaherty & Crumrine Preferred and Income Opportunity Fund Incorporated,NYSE
PFS,\"Provident Financial Services, Inc Common Stock\",NYSE
PFSI,\"PennyMac Financial Services, Inc. Common Stock\",NYSE
PG,Procter & Gamble Company (The) Common Stock,NYSE
PGP,Pimco Global StocksPlus & Income Fund Common Shares of Beneficial Interest,NYSE
PGR,Progressive Corporation (The) Common Stock,NYSE
PGRE,\"Paramount Group, Inc. Common Stock\",NYSE
PGRU,PropertyGuru Group Limited Ordinary Shares,NYSE
PGZ,Principal Real Estate Income Fund Common Shares of Beneficial Interest,NYSE
PH,Parker-Hannifin Corporation Common Stock,NYSE
PHD,\"Pioneer Floating Rate Fund, Inc.\",NYSE
PHG,Koninklijke Philips N.V. NY Registry Shares,NYSE
PHGE,BiomX Inc. COmmon Stock,NYSE MKT
PHGE.U,BiomX Inc. Units,NYSE MKT
PHI,PLDT Inc. Sponsored ADR,NYSE
PHIN,PHINIA Inc. Common Stock,NYSE
PHK,Pimco High Income Fund,NYSE
PHM,\"PulteGroup, Inc. Common Stock\",NYSE
PHR,\"Phreesia, Inc. Common Stock\",NYSE
PHT,\"Pioneer High Income Fund, Inc.\",NYSE
PHX,PHX Minerals Inc. Common Stock,NYSE
PHYT,Pyrophyte Acquisition Corp. Class A Ordinary Shares,NYSE
PHYT.U,\"Pyrophyte Acquisition Corp. Units, each consisting of one share of Class A common stock and one-half of one redeemable warrant\",NYSE
PHYT.W,\"Pyrophyte Acquisition Corp. Warrants, each whole warrant exercisable for one Class A ordinary share at an exercise price of $11.50 per share\",NYSE
PII,Polaris Inc. Common Stock,NYSE
PIM,Putnam Master Intermediate Income Trust Common Stock,NYSE
PINE,\"Alpine Income Property Trust, Inc. Common Stock\",NYSE
PINS,\"Pinterest, Inc. Class A Common Stock\",NYSE
PIPR,Piper Sandler Companies Common Stock,NYSE
PJT,PJT Partners Inc. Class A Common Stock,NYSE
PK,Park Hotels & Resorts Inc. Common Stock ,NYSE
PKE,Park Aerospace Corp. Common Stock,NYSE
PKG,Packaging Corporation of America Common Stock,NYSE
PKST,Peakstone Realty Trust Common Shares,NYSE
PKX,POSCO Holdings Inc. American Depositary Shares (Each representing 1/4th of a share of Common Stock),NYSE
PL,Planet Labs PBC Class A Common Stock,NYSE
PL.W,\"Planet Labs PBC Redeemable warrants, each whole warrant exercisable for one share of Class A common stock, each at an exercise price of $11.50 per share\",NYSE
PLAG,Planet Green Holdings Corp. Common Stock,NYSE MKT
PLD,\"Prologis, Inc. Common Stock\",NYSE
PLG,Platinum Group Metals Ltd. Ordinary Shares (Canada),NYSE MKT
PLNT,\"Planet Fitness, Inc. Common Stock\",NYSE
PLOW,\"Douglas Dynamics, Inc. Common Stock\",NYSE
PLTR,Palantir Technologies Inc. Class A Common Stock,NYSE
PLX,\"Protalix BioTherapeutics, Inc. (DE) Common Stock\",NYSE MKT
PLYM,\"Plymouth Industrial REIT, Inc. Common Stock\",NYSE
PM,Philip Morris International Inc Common Stock,NYSE
PMF,PIMCO Municipal Income Fund Common Stock,NYSE
PML,Pimco Municipal Income Fund II Common Shares of Beneficial Interest,NYSE
PMM,Putnam Managed Municipal Income Trust Common Stock,NYSE
PMNT,Perfect Moment Ltd. Common Stock,NYSE MKT
PMO,Putnam Municipal Opportunities Trust Common Stock,NYSE
PMT,PennyMac Mortgage Investment Trust Common Shares of Beneficial Interest,NYSE
PMT$A,PennyMac Mortgage Investment Trust 8.125% Series A Fixed-to-Floating Rate Cumulative Redeemable Preferred Shares of Beneficial Interest,NYSE
PMT$B,PennyMac Mortgage Investment Trust 8.00% Series B Fixed-to-Floating Rate Cumulative Redeemable Preferred Shares of Beneficial Interest,NYSE
PMT$C,PennyMac Mortgage Investment Trust 6.75% Series C Cumulative Redeemable Preferred Shares of Beneficial Interest,NYSE
PMTU,PennyMac Mortgage Investment Trust 8.50% Senior Notes due 2028,NYSE
PMX,PIMCO Municipal Income Fund III Common Shares of Beneficial Interest,NYSE
PNC,\"PNC Financial Services Group, Inc. (The) Common Stock\",NYSE
PNF,PIMCO New York Municipal Income Fund Common Stock,NYSE
PNI,Pimco New York Municipal Income Fund II Common Shares of Beneficial Interest,NYSE
PNM,\"PNM Resources, Inc. (Holding Co.) Common Stock\",NYSE
PNNT,PennantPark Investment Corporation Common Stock,NYSE
PNR,Pentair plc. Ordinary Share,NYSE
PNST,\"Pinstripes Holdings, Inc. Class A Common Stock\",NYSE
PNST.W,\"Pinstripes Holdings, Inc. Warrants, each whole warrant exercisable for one share of Class A common stock at an exercise price of $11.50 per share\",NYSE
PNW,Pinnacle West Capital Corporation Common Stock,NYSE
POR,Portland General Electric Co Common Stock,NYSE
POST,\"Post Holdings, Inc. Common Stock\",NYSE
PPG,\"PPG Industries, Inc. Common Stock\",NYSE
PPL,PPL Corporation Common Stock,NYSE
PPT,Putnam Premier Income Trust Common Stock,NYSE
PR,Permian Resources Corporation Class A Common Stock,NYSE
PRA,ProAssurance Corporation Common Stock,NYSE
PRE$J,\"PartnerRe Ltd. 4.875% Fixed Rate Non-Cumulative Redeemable Preferred Shares, Series J\",NYSE
PRG,\"PROG Holdings, Inc. Common Stock\",NYSE
PRGO,Perrigo Company plc Ordinary Shares,NYSE
PRH,\"Prudential Financial, Inc. 5.950% Junior Subordinated Notes due 2062\",NYSE
PRI,\"Primerica, Inc. Common Stock\",NYSE
PRIF$D,\"Priority Income Fund, Inc. 7.00% Series D Term Preferred Stock due 2029\",NYSE
PRIF$F,\"Priority Income Fund, Inc. 6.625% Series F Term Preferred Stock due 2027\",NYSE
PRIF$G,\"Priority Income Fund, Inc. 6.25% Series G Preferred Stock Due 2026\",NYSE
PRIF$H,\"Priority Income Fund, Inc. 6.00% Series H Term Preferred Stock due 2026\",NYSE
PRIF$I,\"Priority Income Fund, Inc. 6.125% Series I Term Preferred Stock due 2028\",NYSE
PRIF$J,\"Priority Income Fund, Inc. 6.000% Series J Term Preferred Stock due 2028\",NYSE
PRIF$K,\"Priority Income Fund, Inc. 7.000% Series K Cumulative Preferred Stock\",NYSE
PRIF$L,\"Priority Income Fund, Inc. 6.375% Series L Term Preferred Stock Due 2029\",NYSE
PRIM,Primoris Services Corporation Common Stock,NYSE
PRK,Park National Corporation Common Stock,NYSE MKT
PRKS,United Parks & Resorts Inc. Common Stock,NYSE
PRLB,\"Proto Labs, Inc. Common stock\",NYSE
PRM,\"Perimeter Solutions, SA Ordinary Shares\",NYSE
PRMW,Primo Water Corporation Common Stock,NYSE
PRO,\"PROS Holdings, Inc. Common Stock\",NYSE
PRS,\"Prudential Financial, Inc. 5.625% Junior Subordinated Notes due 2058\",NYSE
PRT,PermRock Royalty Trust Trust Units,NYSE
PRU,\"Prudential Financial, Inc. Common Stock\",NYSE
PSA,Public Storage Common Stock,NYSE
PSA$F,\"Public Storage Depositary Shares Each Representing 1/1,000 of a 5.15% Cumulative Preferred Share of Beneficial Interest, Series F, par value $0.01 per share\",NYSE
PSA$G,\"Public Storage Depositary Shares, Each Representing 1/1,000 of a 5.05% Cumulative Preferred Share of Beneficial Interest, Series G\",NYSE
PSA$H,\"Public Storage Depositary Shares Each Representing 1/1,000 of a  5.60% Cumulative Preferred  Share of Beneficial Interest, Series H\",NYSE
PSA$I,\"Public Storage Depositary Shares Each Representing 1/1,000 of a 4.875% Cumulative Preferred Share of Beneficial Interest, Series I, par value $0.01 per share\",NYSE
PSA$J,\"Public Storage Depositary Shares Each Representing 1/1,000 of a 4.700% Cumulative Preferred Share of Beneficial Interest, Series J, par value $0.01 per share\",NYSE
PSA$K,\"Public Storage Depositary Shares Each Representing 1/1,000 of a 4.75% Cumulative Preferred Share of Beneficial Interest, Series K\",NYSE
PSA$L,\"Public Storage Depositary Shares Each Representing 1/1,000 of a 4.625% Cumulative Preferred Share of Beneficial Interest, Series L, par value $0.01 per share\",NYSE
PSA$M,\"Public Storage Depositary Shares Each Representing 1/1,000 of a 4.125% Cumulative Preferred Share of Beneficial Interest, Series M\",NYSE
PSA$N,\"Public Storage Depositary Shares Each Representing 1/1,000 of a 3.875% Cumulative Preferred Share of Beneficial Interest, Series N\",NYSE
PSA$O,\"Public Storage Depositary Shares Each Representing 1/1,000 of a 3.900% Cumulative Preferred Share of Beneficial Interest, Series O\",NYSE
PSA$P,\"Public Storage Depositary Shares Each Representing 1/1,000 of a 4.000% Cumulative Preferred Share of Bene cial Interest, Series P\",NYSE
PSA$Q,\"Public Storage Depositary Shares Each Representing 1/1,000 of a 3.950% Cumulative Preferred Share of Beneficial Interest, Series Q, par value $0.01 per share\",NYSE
PSA$R,\"Public Storage Depositary Shares Each Representing 1/1,000 of a 4.00% Cumulative Preferred Share of Bene cial Interest, Series R\",NYSE
PSA$S,\"Public Storage Depositary Shares Each Representing 1/1,000 of a 4.100% Cumulative Preferred Share of Beneficial Interest, Series S\",NYSE
PSBD,Palmer Square Capital BDC Inc. Common Stock,NYSE
PSEC$A,Prospect Capital Corporation 5.35% Series A Fixed Rate Cumulative Perpetual Preferred Stock,NYSE
PSF,\"Cohen & Steers Select Preferred and Income Fund, Inc. Common Stock\",NYSE
PSFE,Paysafe Limited Common Shares,NYSE
PSFE.W,\"Paysafe Limited Warrants, exercisable for one Common Share of Paysafe Limited at a price of $11.50 per share\",NYSE
PSN,Parsons Corporation Common Stock,NYSE
PSO,\"Pearson, Plc Common Stock\",NYSE
PSQH,\"PSQ Holdings, Inc. Class A Common Stock\",NYSE
PSQH.W,\"PSQ Holdings, Inc. Warrants, each exercisable for one share of Class A common stock at an exercise price of $11.50 per share\",NYSE
PSTG,\"Pure Storage, Inc. Class A Common Stock\",NYSE
PSTL,\"Postal Realty Trust, Inc. Class A Common Stock\",NYSE
PSX,Phillips 66 Common Stock,NYSE
PTA,Cohen & Steers Tax-Advantaged Preferred Securities and Income Fund Common Shares of Beneficial Interest,NYSE
PTN,\"Palatin Technologies, Inc. Common Stock\",NYSE MKT
PTY,Pimco Corporate & Income Opportunity Fund,NYSE
PUK,Prudential Public Limited Company Common Stock,NYSE
PUMP,ProPetro Holding Corp. Common Stock,NYSE
PVH,PVH Corp. Common Stock,NYSE
PVL,Permianville Royalty Trust Trust Units ,NYSE
PW,Power REIT (MD) Common Stock,NYSE MKT
PW$A,Power REIT 7.75% Series A Cumulative Perpetual Preferred Stock,NYSE MKT
PWR,\"Quanta Services, Inc. Common Stock\",NYSE
PWSC,\"PowerSchool Holdings, Inc. Class A Common Stock\",NYSE
PX,\"P10, Inc. Class A Common Stock\",NYSE
PYN,PIMCO New York Municipal Income Fund III Common Shares of Beneficial Interest,NYSE
PYT,PPlus Tr GSC-2 Tr Ctf Fltg Rate,NYSE
PZC,PIMCO California Municipal Income Fund III Common Shares of Beneficial Interest,NYSE
PZG,Paramount Gold Nevada Corp. Common Stock,NYSE MKT
QBTS,D-Wave Quantum Inc. Common Shares,NYSE
QBTS.W,\"D-Wave Quantum Inc. Warrants, each whole warrant exercisable for 1.4541326 Common Shares at an exercise price of $11.50\",NYSE
QD,\"Qudian Inc. American Depositary Shares, each representing one Class A Ordinary Share\",NYSE
QGEN,Qiagen N.V. Common Shares,NYSE
QS,QuantumScape Corporation Class A Common Stock,NYSE
QSR,Restaurant Brands International Inc. Common Shares,NYSE
QTWO,\"Q2 Holdings, Inc. Common Stock\",NYSE
QUAD,\"Quad Graphics, Inc Class A Common Stock\",NYSE
QVCC,\"QVC, Inc. 6.250% Senior Secured Notes due 2068\",NYSE
QVCD,\"QVC, Inc. 6.375% Senior Secured Notes due 2067\",NYSE
R,\"Ryder System, Inc. Common Stock\",NYSE
RA,Brookfield Real Assets Income Fund Inc. Common Stock,NYSE
RACE,Ferrari N.V. Common Shares,NYSE
RAMP,\"LiveRamp Holdings, Inc. Common Stock\",NYSE
RBA,\"RB Global, Inc. Common Stock\",NYSE
RBC,RBC Bearings Incorporated Common Stock,NYSE
RBCP,RBC Bearings Incorporated 5.00% Series A Mandatory Convertible Preferred Stock,NYSE
RBLX,Roblox Corporation Class A Common Stock,NYSE
RBOT,Vicarious Surgical Inc. Class A Common Stock,NYSE
RBOT.W,\"Vicarious Surgical Inc. Warrants, each whole warrant exercisable for one share of Class A Common Stock at an exercise price of $11.50 per share\",NYSE
RBRK,\"Rubrik, Inc. Class A Common Stock\",NYSE
RBT,\"Rubicon Technologies, Inc. Class A Common Stock\",NYSE
RC,Ready Capital Corproation Common Stock,NYSE
RC$C,Ready Capital Corporation 6.25% Series C Cumulative Convertible Preferred Stock,NYSE
RC$E,Ready Capital Corporation 6.50% Series E Cumulative Redeemable Preferred Stock,NYSE
RCB,Ready Capital Corporation 6.20% Senior Notes due 2026,NYSE
RCC,Ready Capital Corporation 5.75% Senior Notes due 2026,NYSE
RCFA,Perception Capital Corp. IV Class A Ordinary Shares,NYSE
RCFA.U,\"Perception Capital Corp. IV Units, each consisting of one Class A ordinary share and one-half of one redeemable warrant\",NYSE
RCFA.W,\"Perception Capital Corp. IV Warrants, each whole warrant exercisable for one Class A ordinary share at an exercise price of $11.50 per share\",NYSE
RCG,\"RENN Fund, Inc Common Stock\",NYSE MKT
RCI,\"Rogers Communication, Inc. Common Stock\",NYSE
RCL,Royal Caribbean Cruises Ltd. Common Stock,NYSE
RCS,\"PIMCO Strategic Income Fund, Inc.\",NYSE
RCUS,\"Arcus Biosciences, Inc. Common Stock\",NYSE
RDDT,\"Reddit, Inc. Class A Common Stock\",NYSE
RDN,Radian Group Inc. Common Stock,NYSE
RDW,Redwire Corporation Common Stock,NYSE
RDW.W,Redwire Corporation Redeemable Warrants,NYSE
RDY,Dr. Reddy's Laboratories Ltd Common Stock,NYSE
REI,\"Ring Energy, Inc. Common Stock\",NYSE MKT
RELX,RELX PLC PLC American Depositary Shares (Each representing One Ordinary Share),NYSE
REPX,\"Riley Exploration Permian, Inc. Common Stock\",NYSE MKT
RERE,ATRenew Inc. American Depositary Shares (every three of which representing two Class A ordinary shares),NYSE
RES,\"RPC, Inc. Common Stock\",NYSE
REVG,\"REV Group, Inc. Common Stock\",NYSE
REX,REX American Resources Corporation,NYSE
REXR,\"Rexford Industrial Realty, Inc. Common Stock\",NYSE
REXR$B,\"Rexford Industrial Realty, Inc. 5.875% Series B Cumulative Redeemable Preferred Stock\",NYSE
REXR$C,\"Rexford Industrial Realty, Inc. 5.625% Series C Cumulative Redeemable Preferred Stock, par value $0.01 per share\",NYSE
REZI,\"Resideo Technologies, Inc. Common Stock \",NYSE
RF,Regions Financial Corporation Common Stock,NYSE
RF$B,Regions Financial Corporation Depositary Shares Representing 1/40th Perpetual Preferred Series B,NYSE
RF$C,\"Regions Financial Corporation Depositary Shares, each Representing a 1/40th Interest in a  Share of 5.700% Fixed-to-Floating Rate Non-Cumulative  Perpetual Preferred Stock, Series C\",NYSE
RF$E,\"Regions Financial Corporation Depositary Shares, Each Representing a 1/40th Interest in a Share of 4.45% Non-Cumulative Perpetual Preferred Stock, Series E\",NYSE
RFI,\"Cohen & Steers Total Return Realty Fund, Inc. Common Stock\",NYSE
RFL,\"Rafael Holdings, Inc. Class B Common Stock\",NYSE
RFM,\"RiverNorth Flexible Municipal Income Fund, Inc. Common Stock\",NYSE
RFMZ,\"RiverNorth Flexible Municipal Income Fund II, Inc. Common Stock\",NYSE
RGA,\"Reinsurance Group of America, Incorporated Common Stock\",NYSE
RGR,\"Sturm, Ruger & Company, Inc. Common Stock\",NYSE
RGT,\"Royce Global Trust, Inc. Common Stock\",NYSE
RH,RH Common Stock,NYSE
RHE,\"Regional Health Properties, Inc. Common Stock\",NYSE MKT
RHE$A,\"Regional Health Properties, Inc. 10.875% Series A Cumulative Redeemable Preferred Stock\",NYSE MKT
RHI,Robert Half Inc. Common Stock,NYSE
RHP,\"Ryman Hospitality Properties, Inc. (REIT)\",NYSE
RIG,Transocean Ltd (Switzerland) Common Stock,NYSE
RIO,Rio Tinto Plc Common Stock,NYSE
RITM,Rithm Capital Corp. Common Stock,NYSE
RITM$A,Rithm Capital Corp. 7.50% Series A Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock,NYSE
RITM$B,Rithm Capital Corp. 7.125% Series B Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock,NYSE
RITM$C,Rithm Capital Corp. 6.375% Series C Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock,NYSE
RITM$D,Rithm Capital Corp. 7.00% Fixed-Rate Reset Series D Cumulative Redeemable Preferred Stock,NYSE
RIV,\"RiverNorth Opportunities Fund, Inc. Common Stock\",NYSE
RIV$A,\"RiverNorth Opportunities Fund, Inc. 6.00% Series A Perpetual Preferred Stock\",NYSE
RJF,\"Raymond James Financial, Inc. Common Stock\",NYSE
RJF$B,\"Raymond James Financial, Inc. Depositary Shares, each representing a 1/40th interest in a share of 6.375% Fixed-to-Floating Rate Series B Non-Cumulative Perpetual Preferred Stock\",NYSE
RKT,\"Rocket Companies, Inc. Class A Common Stock\",NYSE
RL,Ralph Lauren Corporation Common Stock,NYSE
RLGT,\"Radiant Logistics, Inc. Common Stock\",NYSE MKT
RLI,RLI Corp. Common Stock (DE),NYSE
RLJ,RLJ Lodging Trust Common Shares of Beneficial Interest $0.01 par value,NYSE
RLJ$A,RLJ Lodging Trust $1.95 Series A Cumulative Convertible  Preferred Shares,NYSE
RLTY,Cohen & Steers Real Estate Opportunities and Income Fund Common Shares of Beneficial Interest,NYSE
RLX,\"RLX Technology Inc. American Depositary Shares, each representing the right to receive one (1) Class A ordinary share\",NYSE
RM,Regional Management Corp. Common Stock,NYSE
RMAX,\"RE/MAX Holdings, Inc. Class A Common Stock\",NYSE
RMD,ResMed Inc. Common Stock,NYSE
RMI,\"RiverNorth Opportunistic Municipal Income Fund, Inc. Common Stock\",NYSE
RMM,\"RiverNorth Managed Duration Municipal Income Fund, Inc. Common Stock\",NYSE
RMMZ,\"RiverNorth Managed Duration Municipal Income Fund II, Inc. Common Stock\",NYSE
RMPL$,RiverNorth Capital and Income Fund 5.875%% Series A Term Preferred Stock $0.0001 par value per share,NYSE
RMT,\"Royce Micro-Cap Trust, Inc. Common Stock\",NYSE
RNG,\"RingCentral, Inc. Class A Common Stock\",NYSE
RNGR,\"Ranger Energy Services, Inc. Class A Common Stock\",NYSE
RNP,\"Cohen & Steers REIT and Preferred and Income Fund, Inc. Common Shares\",NYSE
RNR,RenaissanceRe Holdings Ltd. Common Stock,NYSE
RNR$F,\"RenaissanceRe Holdings Ltd. Depositary Shares, each Representing a 1/1,000th Interest in a 5.750% Series F Preference Share\",NYSE
RNR$G,\"RenaissanceRe Holdings Ltd. Depositary Shares, each representing a 1/1,000th interest in a share of 4.20% Series G Preference Shares\",NYSE
RNST,Renasant Corporation Common Stock,NYSE
ROG,Rogers Corporation Common Stock,NYSE
ROK,\"Rockwell Automation, Inc. Common Stock\",NYSE
ROL,\"Rollins, Inc. Common Stock\",NYSE
RPM,RPM International Inc. Common Stock,NYSE
RQI,Cohen & Steers Quality Income Realty Fund Inc Common Shares,NYSE
RRAC,Rigel Resource Acquisition Corp. Class A Ordinary Shares,NYSE
RRAC.U,\"Rigel Resource Acquisition Corp. Units, each consisting of one Class A ordinary share and one-half of one redeemable warrant\",NYSE
RRAC.W,\"Rigel Resource Acquisition Corp. Warrants, each whole warrant exercisable for one Class A ordinary share at an exercise price of $11.50 per share\",NYSE
RRC,Range Resources Corporation Common Stock,NYSE
RRX,Regal Rexnord Corporation Common Stock,NYSE
RS,\"Reliance, Inc. Common Stock\",NYSE
RSF,RiverNorth Capital and Income Fund Common Stock,NYSE
RSG,\"Republic Services, Inc. Common Stock\",NYSE
RSI,\"Rush Street Interactive, Inc. Class A Common Stock\",NYSE
RSKD,Riskified Ltd. Class A Ordinary Shares,NYSE
RTO,Rentokil Initial plc American Depositary Shares (each representing five (5) Ordinary Shares),NYSE
RTX,RTX Corporation Common Stock,NYSE
RVLV,\"Revolve Group, Inc. Class A Common Stock\",NYSE
RVP,\"Retractable Technologies, Inc. Common Stock\",NYSE MKT
RVT,\"Royce Small-Cap Trust, Inc. Common Stock\",NYSE
RVTY,\"Revvity, Inc. Common Stock\",NYSE
RWT,\"Redwood Trust, Inc. Common Stock\",NYSE
RWT$A,\"Redwood Trust, Inc. 10.00% Series A Fixed-Rate Reset Cumulative Redeemable Preferred Stock\",NYSE
RWTN,\"Redwood Trust, Inc. 9.125% Senior Notes Due 2029\",NYSE
RXO,\"RXO, Inc. Common Stock\",NYSE
RY,Royal Bank Of Canada Common Stock,NYSE
RYAM,Rayonier Advanced Materials Inc. Common Stock,NYSE
RYAN,\"Ryan Specialty Holdings, Inc. Class A Common Stock\",NYSE
RYDE,Ryde Group Ltd. Class A Ordinary Shares,NYSE MKT
RYI,Ryerson Holding Corporation Common Stock,NYSE
RYN,Rayonier Inc. REIT Common Stock,NYSE
RZB,\"Reinsurance Group of America, Incorporated 5.75% Fixed-To-Floating Rate Subordinated Debentures due 2056\",NYSE
RZC,\"Reinsurance Group of America, Incorporated 7.125% Fixed-Rate Reset Subordinated Debentures due 2052\",NYSE
S,\"SentinelOne, Inc. Class A Common Stock\",NYSE
SA,\"Seabridge Gold, Inc. Ordinary Shares (Canada)\",NYSE
SABA,Saba Capital Income & Opportunities Fund II Shares of Beneficial Interest,NYSE
SACC,Sachem Capital Corp. 6.875% Notes due 2024,NYSE MKT
SACH,Sachem Capital Corp. Common Shares,NYSE MKT
SACH$A,Sachem Capital Corp. 7.75% Series A Cumulative Redeemable Preferred Stock,NYSE MKT
SAFE,Safehold Inc. New Common Stock ,NYSE
SAH,\"Sonic Automotive, Inc. Common Stock\",NYSE
SAJ,Saratoga Investment Corp 8.00% Notes due 2027,NYSE
SAM,\"Boston Beer Company, Inc. (The) Common Stock\",NYSE
SAN,\"Banco Santander, S.A. Sponsored ADR (Spain)\",NYSE
SAND,Sandstorm Gold Ltd. Ordinary Shares (Canada),NYSE
SAP,SAP  SE ADS,NYSE
SAR,Saratoga Investment Corp New,NYSE
SAT,Saratoga Investment Corp 6.00% Notes due 2027,NYSE
SATX,SatixFy Communications Ltd. Ordinary Share,NYSE MKT
SAVE,\"Spirit Airlines, Inc. Common Stock\",NYSE
SAY,Saratoga Investment Corp 8.125% Notes due 2027,NYSE
SAZ,Saratoga Investment Corp 8.50% Notes due 2028,NYSE
SB,\"Safe Bulkers, Inc Common Stock ($0.001 par value)\",NYSE
SB$C,\"Safe Bulkers, Inc Cumulative Redeemable Perpetual Preferred Series C (Marshall Islands)\",NYSE
SB$D,\"Safe Bulkers, Inc Perpetual Preferred Series D (Marshall Islands)\",NYSE
SBBA,Scorpio Tankers Inc. 7.00% Senior Notes due 2025,NYSE
SBEV,\"Splash Beverage Group, Inc. (NV) Common Stock\",NYSE MKT
SBEV.W,\"Splash Beverage Group, Inc. Warrants to purchase one whole share of Common Stock at an exercise price of $4.60\",NYSE MKT
SBH,\"Sally Beauty Holdings, Inc. (Name to be changed from Sally Holdings, Inc.) Common Stock\",NYSE
SBI,Western Asset Intermediate Muni Fund Inc Common Stock,NYSE
SBOW,\"SilverBow Resorces, Inc. Common Stock\",NYSE
SBR,Sabine Royalty Trust Common Stock,NYSE
SBS,Companhia de saneamento Basico Do Estado De Sao Paulo - Sabesp American Depositary Shares (Each repstg 250 Common Shares),NYSE
SBSW,D/B/A Sibanye-Stillwater Limited ADS,NYSE
SBXC,SilverBox Corp III Class A Common Stock,NYSE
SBXC.U,\"SilverBox Corp III Units, each consisting of one Class A Common Stock and one-third of one redeemable warrant\",NYSE
SBXC.W,\"SilverBox Corp III Redeemable warrants, each whole warrant exercisable for one share of Class A Common Stock at an exercise price of $11.50\",NYSE
SCCB,Sachem Capital Corp. 7.125% Notes due 2024,NYSE MKT
SCCC,Sachem Capital Corp. 7.75% Notes due 2025,NYSE MKT
SCCD,Sachem Capital Corp. 6.00% Notes due 2026,NYSE MKT
SCCE,Sachem Capital Corp. 6.00% Notes due 2027,NYSE MKT
SCCF,Sachem Capital Corp. 7.125% Notes due 2027,NYSE MKT
SCCG,Sachem Capital Corp. 8.00% Notes due 2027,NYSE MKT
SCCO,Southern Copper Corporation Common Stock,NYSE
SCD,LMP Capital and Income Fund Inc. Common Stock,NYSE
SCE$G,SCE Trust II Trust Preferred Securities,NYSE
SCE$H,SCE Trust III Fixed/Floating Rate Trust Preference Securities,NYSE
SCE$J,Southern California Edison Company 5.375% Fixed-to-Floating Rate Trust Preference Securities,NYSE
SCE$K,Southern California Edison Company 5.45% Fixed-to-Floating Rate Trust Preference Securities,NYSE
SCE$L,SCE TRUST VI,NYSE
SCE$M,SCE Trust VII 7.50% Trust Preference Securities,NYSE
SCHW,Charles Schwab Corporation (The) Common Stock,NYSE
SCHW$D,\"The Charles Schwab Corporation Depositary Shares each representing 1/40th interest in a share of 5.95% Non-Cumulative Perpetual Preferred Stock, Series D\",NYSE
SCHW$J,\"The Charles Schwab Corporation Depositary Shares, Each Representing a 1/40th Interest in a Share of 4.450% Non-Cumulative Perpetual Preferred Stock, Series J\",NYSE
SCI,Service Corporation International Common Stock,NYSE
SCL,Stepan Company Common Stock,NYSE
SCM,Stellus Capital Investment Corporation Common Stock,NYSE
SCPX,\"Scorpius Holdings, Inc. Common Stock\",NYSE MKT
SCS,Steelcase Inc. Common Stock,NYSE
SCX,L.S. Starrett Company (The) Common Stock,NYSE
SD,\"SandRidge Energy, Inc. Common Stock\",NYSE
SDHC,Smith Douglas Homes Corp. Class A Common Stock,NYSE
SDHY,PGIM Short Duration High Yield Opportunities Fund Common Shares,NYSE
SDPI,\"Superior Drilling Products, Inc. Common Stock\",NYSE MKT
SDRL,Seadrill Limited Common Shares,NYSE
SE,\"Sea Limited American Depositary Shares, each representing one Class A Ordinary Share\",NYSE
SEAL$A,Seapeak LLC 9.00% Series A Cumulative Redeemable Perpetual Preferred Units,NYSE
SEAL$B,Seapeak LLC 8.50% Series B Fixed-to-Floating Rate Cumulative Redeemable Perpetual Preferred Units,NYSE
SEB,Seaboard Corporation Common Stock,NYSE MKT
SEDA,SDCL EDGE Acquisition Corporation Class A Ordinary Shares,NYSE
SEDA.U,\"SDCL EDGE Acquisition Corporation Units, each consisting of one Class A ordinary share and one-half of one redeemable warrant\",NYSE
SEDA.W,\"SDCL EDGE Acquisition Corporation Redeemable warrants, each whole warrant exercisable for one share of Class A ordinary share at an exercise price of $11.50\",NYSE
SEE,Sealed Air Corporation Common Stock,NYSE
SEM,Select Medical Holdings Corporation Common Stock,NYSE
SEMR,\"SEMrush Holdings, Inc. Class A Common Stock\",NYSE
SENS,\"Senseonics Holdings, Inc. Common Stock\",NYSE MKT
SER,\"Serina Therapeutics, Inc. Common Stock\",NYSE MKT
SES,SES AI Corporation Class A Common Stock,NYSE
SES.W,\"SES AI Corporation Warrants, each whole warrant exercisable for one share of Class A Common Stock at an exercise price of $11.50 per share\",NYSE
SF,Stifel Financial Corporation Common Stock,NYSE
SF$B,\"Stifel Financial Corporation Depositary Shares, Each Representing 1/1,000th  Interest in a Share of 6.25% Non-Cumulative  Preferred Stock, Series B\",NYSE
SF$C,\"Stifel Financial Corporation Depositary Shares, Each Representing 1/1,000th Interest in a Share of 6.125% Non Cumulative Preferred Stock, Series C\",NYSE
SF$D,\"Stifel Financial Corporation Depositary Shares, Each Representing 1/1,000th Interest in a Share of 4.50% Non-Cumulative Preferred Stock, Series D\",NYSE
SFB,Stifel Financial Corporation 5.20% Senior Notes due 2047,NYSE
SFBS,\"ServisFirst Bancshares, Inc. Common Stock\",NYSE
SFL,SFL Corporation Ltd,NYSE
SG,\"Sweetgreen, Inc. Class A Common Stock\",NYSE
SGE,\"Strong Global Entertainment, Inc. Class A Common Voting Shares\",NYSE MKT
SGHC,Super Group (SGHC) Limited Ordinary Shares,NYSE
SGN,\"Signing Day Sports, Inc. Common Stock\",NYSE MKT
SGU,Star Group L.P. Common Stock,NYSE
SHAK,\"Shake Shack, Inc. Class A Common Stock\",NYSE
SHCO,Soho House & Co Inc. Class A Common Stock,NYSE
SHEL,Shell PLC American Depositary Shares (each representing two (2) Ordinary Shares) ,NYSE
SHG,Shinhan Financial Group Co Ltd American Depositary Shares,NYSE
SHO,\"Sunstone Hotel Investors, Inc. Sunstone Hotel Investors, Inc. Common Shares\",NYSE
SHO$H,\"Sunstone Hotel Investors, Inc. 6.125% Series H Cumulative Redeemable Preferred Stock\",NYSE
SHO$I,\"Sunstone Hotel Investors, Inc. 5.70% Series I Cumulative Redeemable Preferred Stock\",NYSE
SHOP,Shopify Inc. Class A Subordinate Voting Shares,NYSE
SHW,Sherwin-Williams Company (The) Common Stock,NYSE
SID,Companhia Siderurgica Nacional S.A. Common Stock,NYSE
SIF,\"SIFCO Industries, Inc. Common Stock\",NYSE MKT
SIG,Signet Jewelers Limited Common Shares,NYSE
SII,Sprott Inc. Common Shares,NYSE
SILV,SilverCrest Metals Inc. Common Shares,NYSE MKT
SIM,\"Grupo Simec, S.A.B. de C.V. American Depositary Shares\",NYSE MKT
SING,SinglePoint Inc. Common Stock,BATS
SITC,SITE Centers Corp. Common Stock,NYSE
SITC$A,SITE Centers Corp. 6.375% Class A Preferred Shares,NYSE
SITE,\"SiteOne Landscape Supply, Inc. Common Stock\",NYSE
SIX,Six Flags Entertainment Corporation New Common Stock,NYSE
SJM,The J.M. Smucker Company Common Stock,NYSE
SJT,San Juan Basin Royalty Trust Common Stock,NYSE
SJW,SJW Group Common Stock (DE),NYSE
SKE,Skeena Resources Limited Common Shares,NYSE
SKIL,Skillsoft Corp. Class A Common Stock,NYSE
SKLZ,Skillz Inc. Class A Common Stock,NYSE
SKM,\"SK Telecom Co., Ltd. Common Stock\",NYSE
SKT,Tanger Inc. Common Stock,NYSE
SKX,\"Skechers U.S.A., Inc. Common Stock\",NYSE
SKY,Skyline Champion Corporation Common Stock,NYSE
SKYH,Sky Harbour Group Corporation Class A Common Stock,NYSE MKT
SKYH.W,\"Sky Harbour Group Corporation Warrants, each whole warrant exercisable for one share of Class A common stock at an exercise price of $11.50 per share\",NYSE MKT
SLB,Schlumberger N.V. Common Stock,NYSE
SLCA,\"U.S. Silica Holdings, Inc. Common Stock\",NYSE
SLF,Sun Life Financial Inc. Common Stock,NYSE
SLG,SL Green Realty Corp Common Stock,NYSE
SLG$I,SL Green Realty Corporation Preferred Series I,NYSE
SLGN,Silgan Holdings Inc. Common Stock,NYSE
SLI,Standard Lithium Ltd. Common Shares,NYSE MKT
SLND,\"Southland Holdings, Inc. Common Stock\",NYSE MKT
SLND.W,\"Southland Holdings, Inc. Warrants\",NYSE MKT
SLQT,\"SelectQuote, Inc. Common Stock\",NYSE
SLSR,Solaris Resources Inc. Common Shares,NYSE MKT
SLVM,Sylvamo Corporation Common Stock,NYSE
SM,SM Energy Company Common Stock,NYSE
SMAR,Smartsheet Inc. Class A Common Stock,NYSE
SMBK,\"SmartFinancial, Inc. Common Stock\",NYSE
SMFG,Sumitomo Mitsui Financial Group Inc Unsponsored American Depositary Shares (Japan),NYSE
SMG,Scotts Miracle-Gro Company (The) Common Stock,NYSE
SMHB,ETRACS Monthly Pay 2x Leveraged Small Cap High Dividend ETN Series B,NYSE ARCA
SMHI,SEACOR Marine Holdings Inc. Common Stock ,NYSE
SMLP,\"Summit Midstream Partners, LP Common Units Representing Limited Partner Interests\",NYSE
SMP,\"Standard Motor Products, Inc. Common Stock\",NYSE
SMR,NuScale Power Corporation Class A Common Stock,NYSE
SMR.W,\"NuScale Power Corporation Warrants, each exercisable for one share of Class A Common Stock at an exercise price of $11.50 per share\",NYSE
SMRT,\"SmartRent, Inc. Class A Common Stock\",NYSE
SMWB,Similarweb Ltd. Ordinary Shares,NYSE
SN,\"SharkNinja, Inc. Ordinary Shares\",NYSE
SNA,Snap-On Incorporated Common Stock,NYSE
SNAP,Snap Inc. Class A Common Stock,NYSE
SNDA,\"Sonida Senior Living, Inc. Common Stock\",NYSE
SNDR,\"Schneider National, Inc. Common Stock\",NYSE
SNN,\"Smith & Nephew SNATS, Inc. Common Stock\",NYSE
SNOW,Snowflake Inc. Class A Common Stock,NYSE
SNV,Synovus Financial Corp. Common Stock,NYSE
SNV$D,\"Synovus Financial Corp. Fixed-to-Floating Rate Non-Cumulative Perpetual Preferred Stock, Series D Liquation Preference $25.00 per Share\",NYSE
SNV$E,\"Synovus Financial Corp. 5.875% Fixed-Rate Reset Non-Cumulative Perpetual Preferred Stock, Series E\",NYSE
SNX,TD SYNNEX Corporation Common Stock,NYSE
SO,Southern Company (The) Common Stock,NYSE
SOAR,\"Volato Group, Inc. Class A Common Stock\",NYSE MKT
SOAR.W,\"Volato Group, Inc. Redeemable Warrants, each whole warrant exercisable for one share of Class A common stock at an exercise price of $11.50\",NYSE MKT
SOC,Sable Offshore Corp. Common Stock,NYSE
SOC.W,\"Sable Offshore Corp. Warrants, each whole warrant exercisable for one share of Common Stock at an exercise price of $11.50\",NYSE
SOI,\"Solaris Oilfield Infrastructure, Inc. Class A Common Stock\",NYSE
SOJC,\"Southern Company (The) Series 2017B 5.25% Junior Subordinated Notes due December 1, 2077\",NYSE
SOJD,\"Southern Company (The) Series 2020A 4.95% Junior Subordinated Notes due January 30, 2080\",NYSE
SOJE,\"Southern Company (The) Series 2020C 4.20% Junior Subordinated Notes due October 15, 2060\",NYSE
SOL,\"Emeren Group Ltd American Depositary Shares, each representing 10 shares\",NYSE
SOLV,Solventum Corporation Common Stock,NYSE
SON,Sonoco Products Company Common Stock,NYSE
SONY,Sony Group Corporation American Depositary Shares ,NYSE
SOR,\"Source Capital, Inc. Common Stock\",NYSE
SOS,SOS Limited American Depositary Shares,NYSE
SPB,\"Spectrum Brands Holdings, Inc. Common Stock\",NYSE
SPCE,\"Virgin Galactic Holdings, Inc. Common Stock\",NYSE
SPE,\"Special Opportunities Fund, Inc Common Stock\",NYSE
SPE$C,\"Special Opportunities Fund Inc. 2.75% Convertible Preferred Stock, Series C\",NYSE
SPG,\"Simon Property Group, Inc. Common Stock\",NYSE
SPG$J,\"Simon Property Group, Inc. Simon Property Group 8 3/8% Series J Cumulative Redeemable Preferred Stock\",NYSE
SPGI,S&P Global Inc. Common Stock,NYSE
SPH,\"Suburban Propane Partners, L.P. Common Stock\",NYSE
SPHR,Sphere Entertainment Co. Class A Common Stock,NYSE
SPIR,\"Spire Global, Inc. Class A Common Stock\",NYSE
SPLP,Steel Partners Holdings LP LTD PARTNERSHIP UNIT,NYSE
SPLP$A,\"Steel Partners Holdings LP 6.0% Series A Preferred Units, no par value\",NYSE
SPNT,SiriusPoint Ltd. Common Shares,NYSE
SPNT$B,\"SiriusPoint Ltd. 8.00% Resettable Fixed Rate Preference Shares, Series B, $25.00 liquidation preference per share\",NYSE
SPOT,Spotify Technology S.A. Ordinary Shares,NYSE
SPR,\"Spirit Aerosystems Holdings, Inc. Common Stock\",NYSE
SPRU,Spruce Power Holding Corporation Class A Common Stock,NYSE
SPXC,\"SPX Technologies, Inc. Common Stock\",NYSE
SPXX,Nuveen S&P 500 Dynamic Overwrite Fund,NYSE
SQ,\"Block, Inc. Class A Common Stock,\",NYSE
SQM,Sociedad Quimica y Minera S.A. Common Stock,NYSE
SQNS,Sequans Communications S.A. American Depositary Shares,NYSE
SQSP,\"Squarespace, Inc. Class A Common Stock\",NYSE
SR,Spire Inc. Common Stock,NYSE
SR$A,\"Spire Inc. Depositary Shares, each representing a 1/1,000th interest in a share of 5.90% Series A Cumulative Redeemable Perpetual Preferred Stock\",NYSE
SRE,DBA Sempra Common Stock,NYSE
SREA,DBA Sempra 5.750% Junior Subordinated Notes due 2079,NYSE
SRFM,Surf Air Mobility Inc. Common Stock,NYSE
SRG,Seritage Growth Properties Class A Common Stock,NYSE
SRG$A,Seritage Growth Properties 7.00% Series A Cumulative Redeemable Preferred Shares of Beneficial Interest,NYSE
SRI,\"Stoneridge, Inc. Common Stock\",NYSE
SRL,Scully Royalty Ltd.,NYSE
SRV,NXG Cushing Midstream Energy Fund Common Shares of Beneficial Interest,NYSE
SSB,SouthState Corporation Common Stock,NYSE
SSD,\"Simpson Manufacturing Company, Inc. Common Stock\",NYSE
SSL,Sasol Ltd. American Depositary Shares,NYSE
SST,\"System1, Inc. Class A Common Stock\",NYSE
SST.W,\"System1, Inc. Warrants, each whole warrant exercisable for one share of Class A Common Stock at an exercise price of $11.50 per share\",NYSE
SSTK,\"Shutterstock, Inc. Common Stock\",NYSE
SSY,\"SunLink Health Systems, Inc. Common Stock\",NYSE MKT
ST,Sensata Technologies Holding plc Ordinary Shares,NYSE
STAG,\"Stag Industrial, Inc. Common Stock\",NYSE
STC,Stewart Information Services Corporation Common Stock,NYSE
STE,STERIS plc (Ireland) Ordinary Shares,NYSE
STEL,\"Stellar Bancorp, Inc. Common Stock\",NYSE
STEM,\"Stem, Inc. Class A Common Stock\",NYSE
STEW,\"SRH Total Return Fund, Inc. Common Stock\",NYSE
STG,\"Sunlands Technology Group American Depositary Shares, representing Class A ordinary shares\",NYSE
STK,Columbia Seligman Premium Technology Growth Fund Inc,NYSE
STLA,Stellantis N.V. Common Shares,NYSE
STM,STMicroelectronics N.V. Common Stock,NYSE
STN,Stantec Inc Common Stock,NYSE
STNG,Scorpio Tankers Inc. Common Shares,NYSE
STR,Sitio Royalties Corp. Class A Common Stock,NYSE
STRW,\"Strawberry Fields REIT, Inc. Common Stock\",NYSE MKT
STT,State Street Corporation Common Stock,NYSE
STT$G,\"State Street Corporation Depositary shares, each representing a 1/4,000th ownership interest in a share of Fixed-to-Floating Rate Non-Cumulative\",NYSE
STVN,Stevanato Group S.p.A. Ordinary Shares,NYSE
STWD,\"STARWOOD PROPERTY TRUST, INC. Starwood Property Trust Inc.\",NYSE
STXS,\"Stereotaxis, Inc. Common Stock\",NYSE MKT
STZ,\"Constellation Brands, Inc. Common Stock\",NYSE
SU,Suncor Energy  Inc. Common Stock,NYSE
SUI,\"Sun Communities, Inc. Common Stock\",NYSE
SUM,\"Summit Materials, Inc. Class A Common Stock\",NYSE
SUN,Sunoco LP Common Units representing limited partner interests,NYSE
SUP,\"Superior Industries International, Inc. Common Stock (DE)\",NYSE
SUPV,Grupo Supervielle S.A. American Depositary Shares each Representing five Class B shares,NYSE
SUZ,Suzano S.A. American Depositary Shares (each representing One Ordinary Share),NYSE
SVM,Silvercorp Metals Inc. Common Shares,NYSE MKT
SVT,\"Servotronics, Inc. Common Stock\",NYSE MKT
SVV,\"Savers Value Village, Inc. Common Stock\",NYSE
SWI,SolarWinds Corporation Common Stock,NYSE
SWK,\"Stanley Black & Decker, Inc. Common Stock\",NYSE
SWN,Southwestern Energy Company Common Stock,NYSE
SWX,\"Southwest Gas Holdings, Inc. Common Stock (DE)\",NYSE
SWZ,\"Swiss Helvetia Fund, Inc. (The) Common Stock\",NYSE
SXC,\"SunCoke Energy, Inc. Common Stock\",NYSE
SXI,Standex International Corporation Common Stock,NYSE
SXT,Sensient Technologies Corporation Common Stock,NYSE
SYF,Synchrony Financial Common Stock,NYSE
SYF$A,\"Synchrony Financial Depositary Shares, each Representing a 1/40th Interest in a Share of 5.625% Fixed Rate Non-Cumulative Perpetual Preferred Stock, Series A\",NYSE
SYF$B,\"Synchrony Financial Depositary Shares Each Representing a 1/40th Interest in a Share of 8.250% Fixed Rate Reset Non-Cumulative Perpetual Preferred Stock, Series B\",NYSE
SYK,Stryker Corporation Common Stock,NYSE
SYNX,Silynxcom Ltd. Ordinary Shares,NYSE MKT
SYY,Sysco Corporation Common Stock,NYSE
T,AT&T Inc.,NYSE
T$A,\"AT&T Inc. Depositary Shares, each representing a 1/1,000th interest in a share of 5.000% Perpetual Preferred Stock, Series A\",NYSE
T$C,\"AT&T Inc. Depositary Shares, each representing a 1/1,000th interest in a share of 4.750% Perpetual Preferred Stock, Series C\",NYSE
TAC,TransAlta Corporation Ordinary Shares,NYSE
TAK,Takeda Pharmaceutical Company Limited American Depositary Shares (each representing 1/2 of a share of Common Stock),NYSE
TAL,TAL Education Group American Depositary Shares,NYSE
TALO,\"Talos Energy, Inc. Common Stock\",NYSE
TAP,Molson Coors Beverage Company Class B Common Stock,NYSE
TAP.A,Molson Coors Beverage Company Class A Common Stock,NYSE
TARO,Taro Pharmaceutical Industries Ltd. Ordinary Shares,NYSE
TBB,AT&T Inc. 5.350% Global Notes due 2066,NYSE
TBBB,BBB Foods Inc. Class A Common Shares,NYSE
TBC,AT&T Inc. 5.625% Global Notes due 2067,NYSE
TBI,\"TrueBlue, Inc. Common Stock\",NYSE
TCI,\"Transcontinental Realty Investors, Inc. Common Stock\",NYSE
TCOA,Zalatoris Acquisition Corp. Class A Common Stock,NYSE
TCOA.U,\"Zalatoris Acquisition Corp. Units, each consisting of one share of Class A common stock and one-half of one redeemable public warrant\",NYSE
TCOA.W,\"Zalatoris Acquisition Corp. Public warrants, each whole public warrant exercisable for one share of Class A Common Stock at an exercise price of $11.50 per share\",NYSE
TCS,Container Store (The) Common Stock,NYSE
TD,Toronto Dominion Bank (The) Common Stock,NYSE
TDC,Teradata Corporation Common Stock,NYSE
TDCX,\"TDCX Inc. American Depositary Shares, each representing one Class A ordinary share\",NYSE
TDF,\"Templeton Dragon Fund, Inc. Common Stock\",NYSE
TDG,Transdigm Group Incorporated Common Stock,NYSE
TDOC,\"Teladoc Health, Inc. Common Stock\",NYSE
TDS,\"Telephone and Data Systems, Inc. Common Shares\",NYSE
TDS$U,\"Telephone and Data Systems, Inc. Depositary Shares, Each Representing a 1/1,000th Interest in a 6.625% Series UU Cumulative Redeemable Perpetual Preferred Stock\",NYSE
TDS$V,\"Telephone and Data Systems, Inc. Depositary Shares, Each Representing a 1/1,000th Interest in a 6.000% Series VV Cumulative Redeemable Perpetual Preferred Stock\",NYSE
TDW,Tidewater Inc. Common Stock,NYSE
TDW.W,Tidewater Inc. Warrant,NYSE MKT
TDY,Teledyne Technologies Incorporated Common Stock,NYSE
TEAF,Ecofin Sustainable and Social Impact Term Fund,NYSE
TECK,Teck Resources Ltd Ordinary Shares,NYSE
TEF,Telefonica SA Common Stock,NYSE
TEI,\"Templeton Emerging Markets Income Fund, Inc. Common Stock\",NYSE
TEL,TE Connectivity Ltd. New Switzerland Registered Shares,NYSE
TELL,Tellurian Inc. Common Stock,NYSE MKT
TELZ,Tellurian Inc. 8.25% Senior Notes due 2028,NYSE MKT
TEO,Telecom Argentina SA,NYSE
TEVA,Teva Pharmaceutical Industries Limited American Depositary Shares,NYSE
TEX,Terex Corporation Common Stock,NYSE
TFC,Truist Financial Corporation Common Stock,NYSE
TFC$I,Truist Financial Corporation Depositary Shares,NYSE
TFC$O,\"Truist Financial Corporation Depositary Shares, Each Representing a 1/1,000th Interest in a Share of Series O Non-Cumulative Perpetual Preferred Stock\",NYSE
TFC$R,\"Truist Financial Corporation Depositary Shares, each representing 1/1,000th interest in a share of Series R Non-Cumulative Perpetual Preferred Stock\",NYSE
TFII,TFI International Inc. Common Shares,NYSE
TFPM,Triple Flag Precious Metals Corp. Common Shares,NYSE
TFSA,\"Terra Income Fund 6, LLC 7.00% Notes due 2026\",NYSE
TFX,Teleflex Incorporated Common Stock,NYSE
TG,Tredegar Corporation Common Stock,NYSE
TGB,\"Taseko Mines, Ltd. Common Stock\",NYSE MKT
TGI,\"Triumph Group, Inc. Common Stock\",NYSE
TGLS,Tecnoglass Inc. Ordinary Shares,NYSE
TGNA,TEGNA Inc,NYSE
TGS,Transportadora de Gas del Sur SA TGS Common Stock,NYSE
TGT,Target Corporation Common Stock,NYSE
THC,Tenet Healthcare Corporation Common Stock,NYSE
THG,Hanover Insurance Group Inc,NYSE
THM,\"International Tower Hill Mines, Ltd. Ordinary Shares (Canada)\",NYSE MKT
THO,\"Thor Industries, Inc. Common Stock\",NYSE
THQ,abrdn Healthcare Opportunities Fund Shares of Beneficial Interest,NYSE
THR,\"Thermon Group Holdings, Inc. Common Stock\",NYSE
THS,\"Treehouse Foods, Inc. Common Stock\",NYSE
THW,abrdn World Healthcare Fund Shares of Beneficial Interest,NYSE
TIMB,TIM S.A. American Depositary Shares (Each representing 5 Common Shares) ,NYSE
TISI,\"Team, Inc. Common Stock\",NYSE
TIXT,TELUS International (Cda) Inc. Subordinate Voting Shares,NYSE
TJX,\"TJX Companies, Inc. (The) Common Stock\",NYSE
TK,Teekay Corporation Common Stock,NYSE
TKC,Turkcell Iletisim Hizmetleri AS Common Stock,NYSE
TKO,\"TKO Group Holdings, Inc. Class A Common Stock\",NYSE
TKR,Timken Company (The) Common Stock,NYSE
TLK,\"PT Telekomunikasi Indonesia, Tbk\",NYSE
TLYS,\"Tilly's, Inc. Common Stock\",NYSE
TM,Toyota Motor Corporation Common Stock,NYSE
TME,\"Tencent Music Entertainment Group American Depositary Shares, each representing two Class A Ordinary Shares\",NYSE
TMHC,Taylor Morrison Home Corporation Common Stock,NYSE
TMO,Thermo Fisher Scientific Inc Common Stock,NYSE
TMP,Tompkins Financial Corporation Common Stock,NYSE MKT
TMQ,Trilogy Metals Inc. Common Stock,NYSE MKT
TNC,Tennant Company Common Stock,NYSE
TNET,\"TriNet Group, Inc. Common Stock\",NYSE
TNK,Teekay Tankers Ltd.,NYSE
TNL,Travel   Leisure Co. Common  Stock,NYSE
TNP,Tsakos Energy Navigation Ltd Common Shares,NYSE
TNP$E,\"Tsakos Energy Navigation Ltd Series E Fixed-to-Floating Rate Cumulative Redeemable Perpetual Preferred Shares, par value $1.00\",NYSE
TNP$F,\"Tsakos Energy Navigation Ltd Series F Fixed-to-Floating Rate Cumulative Redeemable Perpetual Preferred Shares, par value $1.00\",NYSE
TOL,\"Toll Brothers, Inc. Common Stock\",NYSE
TOON,\"Kartoon Studios, Inc. Common Stock\",NYSE MKT
TOPS,\"TOP Ships, Inc. Common Stock\",NYSE MKT
TOST,\"Toast, Inc. Class A Common Stock\",NYSE
TOVX,\"Theriva Biologics, Inc. Common Stock\",NYSE MKT
TPB,\"Turning Point Brands, Inc. Common Stock\",NYSE
TPC,Tutor Perini Corporation Common Stock,NYSE
TPET,Trio Petroleum Corp. Common Stock,NYSE MKT
TPH,\"Tri Pointe Homes, Inc. Common Stock\",NYSE
TPHS,Trinity Place Holdings Inc. Common Stock,NYSE MKT
TPL,Texas Pacific Land Corporation Common Stock,NYSE
TPR,\"Tapestry, Inc. Common Stock\",NYSE
TPTA,\"Terra Property Trust, Inc. 6.00% Notes due 2026\",NYSE
TPVG,TriplePoint Venture Growth BDC Corp. Common Stock,NYSE
TPX,\"Tempur Sealy International, Inc. Common Stock\",NYSE
TPZ,\"Tortoise Power and Energy Infrastructure Fund, Inc Common Stock\",NYSE
TR,\"Tootsie Roll Industries, Inc. Common Stock\",NYSE
TRAK,\"ReposiTrak, Inc. Common Stock\",NYSE
TRC,Tejon Ranch Co Common Stock,NYSE
TREX,\"Trex Company, Inc. Common Stock\",NYSE
TRGP,\"Targa Resources, Inc. Common Stock\",NYSE
TRI,Thomson Reuters Corp Common Shares,NYSE
TRIS,Tristar Acquisition I Corp. Class A Ordinary Shares,NYSE
TRIS.U,\"Tristar Acquisition I Corp. Units, each consisting of one Class A Ordinary Share and one-half of one Redeemable Warrant to purchase one Class A Ordinary Share\",NYSE
TRIS.W,\"Tristar Acquisition I Corp. Redeemable Warrants, each exercisable for one Class A Ordinary Share at an exercise price of $11.50 per share\",NYSE
TRN,\"Trinity Industries, Inc. Common Stock\",NYSE
TRNO,Terreno Realty Corporation Common Stock,NYSE
TROX,Tronox Holdings plc Ordinary Shares (UK),NYSE
TRP,TC Energy Corporation Common Stock,NYSE
TRT,Trio-Tech International Common Stock,NYSE MKT
TRTL,TortoiseEcofin Acquisition Corp. III Class A Ordinary Shares,NYSE
TRTL.U,\"TortoiseEcofin Acquisition Corp. III Units each consisting of one Class A Ordinary Share, and one-fourth of one redeemable warrant\",NYSE
TRTL.W,\"TortoiseEcofin Acquisition Corp. III Warrant, entitles to purchase one Class A Ordinary Share at a price of $11.50 per share\",NYSE
TRTN$A,Triton International Limited 8.50% Series A Cumulative Redeemable Perpetual  Preference Shares,NYSE
TRTN$B,Triton International Limited 8.00% Series B Cumulative Redeemable Perpetual Preference Shares,NYSE
TRTN$C,Triton International Limited 7.375% Series C Cumulative Redeemable Perpetual Preference Shares,NYSE
TRTN$D,Triton International Limited 6.875% Series D Cumulative Redeemable Perpetual Preference Shares,NYSE
TRTN$E,Triton International Limited 5.75% Series E Cumulative Redeemable Perpetual Preference Shares,NYSE
TRTX,\"TPG RE Finance Trust, Inc. Common Stock\",NYSE
TRTX$C,\"TPG RE Finance Trust, Inc. 6.25% Series C Cumulative Redeemable Preferred Stock, $0.001 par value per share\",NYSE
TRU,TransUnion Common Stock,NYSE
TRV,\"The Travelers Companies, Inc. Common Stock\",NYSE
TRX,TRX Gold Corporation Common Stock,NYSE MKT
TS,Tenaris S.A. American Depositary Shares,NYSE
TSE,Trinseo PLC Ordinary Shares ,NYSE
TSI,\"TCW Strategic Income Fund, Inc. Common Stock\",NYSE
TSLX,\"Sixth Street Specialty Lending, Inc. Common Stock\",NYSE
TSM,Taiwan Semiconductor Manufacturing Company Ltd.,NYSE
TSN,\"Tyson Foods, Inc. Common Stock\",NYSE
TSQ,\"Townsquare Media, Inc. Class A Common Stock\",NYSE
TT,Trane Technologies plc,NYSE
TTC,Toro Company (The) Common Stock,NYSE
TTE,TotalEnergies SE,NYSE
TTI,\"Tetra Technologies, Inc. Common Stock\",NYSE
TTP,\"Tortoise Pipeline & Energy Fund, Inc. Common Stock\",NYSE
TU,Telus Corporation Ordinary Shares,NYSE
TUP,Tupperware Brands Corporation Common Stock,NYSE
TUYA,\"Tuya Inc. American Depositary Shares, each representing one Class A Ordinary Share\",NYSE
TV,Grupo Televisa S.A.B. Common Stock,NYSE
TVC,Tennessee Valley Authority Common Stock,NYSE
TVE,Tennessee Valley Authority,NYSE
TWI,\"Titan International, Inc. (DE) Common Stock\",NYSE
TWLO,Twilio Inc. Class A Common Stock,NYSE
TWN,\"Taiwan Fund, Inc. (The) Common Stock\",NYSE
TWO,Two Harbors Investment Corp,NYSE
TWO$A,Two Harbors Investments Corp 8.125% Series A Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock ($25.00 liquidation preference per share),NYSE
TWO$B,Two Harbors Investments Corp 7.625% Series B Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock,NYSE
TWO$C,Two Harbors Investments Corp 7.25% Series C Fixed-to-Floating Rate Cumulative Redeemable Preferred Stock,NYSE
TX,\"Ternium S.A. Ternium S.A. American Depositary Shares (each representing ten shares, USD1.00 par value)\",NYSE
TXO,\"TXO Partners, L.P. Common Units Representing Limited Partner Interests\",NYSE
TXT,Textron Inc. Common Stock,NYSE
TY,Tri Continental Corporation Common Stock,NYSE
TY$,Tri Continental Corporation Preferred Stock,NYSE
TYG,Tortoise Energy Infrastructure Corporation Common Stock,NYSE
TYL,\"Tyler Technologies, Inc. Common Stock\",NYSE
U,Unity Software Inc. Common Stock,NYSE
UA,\"Under Armour, Inc. Class C Common Stock\",NYSE
UAA,\"Under Armour, Inc. Class A Common Stock\",NYSE
UAMY,United States Antimony Corporation Common Stock,NYSE MKT
UAN,\"CVR Partners, LP Common Units representing Limited Partner Interests\",NYSE
UAVS,\"AgEagle Aerial Systems, Inc. Common Stock\",NYSE MKT
UBER,\"Uber Technologies, Inc. Common Stock\",NYSE
UBS,UBS Group AG Registered Ordinary Shares,NYSE
UCIB,\"ETRACS UBS Bloomberg Constant Maturity Commodity Index (CMCI) Total Return ETN Series B due April 5, 2038\",NYSE ARCA
UDR,\"UDR, Inc. Common Stock\",NYSE
UE,Urban Edge Properties Common Shares of Beneficial Interest,NYSE
UEC,Uranium Energy Corp. Common Stock,NYSE MKT
UFI,\"Unifi, Inc. New Common Stock\",NYSE
UGI,UGI Corporation Common Stock,NYSE
UGIC,UGI Corporation Corporate Units,NYSE
UGP,Ultrapar Participacoes S.A. (New) American Depositary Shares (Each representing one Common Share),NYSE
UHAL,U-Haul Holding Company Common Stock,NYSE
UHAL.B,U-Haul Holding Company Series N Non-Voting Common Stock ,NYSE
UHS,\"Universal Health Services, Inc. Common Stock\",NYSE
UHT,Universal Health Realty Income Trust Common Stock,NYSE
UI,Ubiquiti Inc. Common Stock,NYSE
UIS,Unisys Corporation New Common Stock,NYSE
UL,Unilever PLC Common Stock,NYSE
ULS,UL Solutions Inc. Class A Common Stock,NYSE
UMAC,\"Unusual Machines, Inc. Common Stock\",NYSE MKT
UMC,United Microelectronics Corporation (NEW) Common Stock,NYSE
UMH,\"UMH Properties, Inc. Common Stock\",NYSE
UMH$D,\"UMH Properties, Inc. 6.375% Series D Cumulative Redeemable Preferred Stock, Liquidation Preference $25 per share\",NYSE
UNF,Unifirst Corporation Common Stock,NYSE
UNFI,\"United Natural Foods, Inc. Common Stock\",NYSE
UNH,UnitedHealth Group Incorporated Common Stock (DE),NYSE
UNM,Unum Group Common Stock,NYSE
UNMA,Unum Group 6.250% Junior Subordinated Notes due 2058,NYSE
UNP,Union Pacific Corporation Common Stock,NYSE
UP,Wheels Up Experience Inc. Class A Common Stock,NYSE
UPS,\"United Parcel Service, Inc. Common Stock\",NYSE
URG,Ur Energy Inc Common Shares (Canada),NYSE MKT
URI,\"United Rentals, Inc. Common Stock\",NYSE
USA,Liberty All-Star Equity Fund Common Stock,NYSE
USAC,\"USA Compression Partners, LP Common Units Representing Limited Partner Interests\",NYSE
USAS,\"Americas Gold and Silver Corporation Common Shares, no par value\",NYSE MKT
USB,U.S. Bancorp Common Stock,NYSE
USB$A,\"U.S. Bancorp Depositary Shares, Each representing a 1/100th interest in a share of Series A Non-CumulativePerpetual Pfd Stock\",NYSE
USB$H,U.S. Bancorp Depositary Shares repstg 1/1000th Pfd Ser B,NYSE
USB$P,\"U.S. Bancorp Depositary Shares each representing a 1/1,000th interest in a share of Series K Non-Cumulative Perpetual Preferred Stock\",NYSE
USB$Q,\"U.S. Bancorp Depositary Shares, Each Representing a 1/1,000th Interest in a Share of Series L Non-Cumulative Perpetual Preferred Stock\",NYSE
USB$R,\"U.S. Bancorp Depositary Shares, Each Representing a 1/1,000th Interest in a Share of Series M Non-Cumulative Perpetual Preferred Stock\",NYSE
USB$S,\"U.S. Bancorp Depositary Shares, each representing a 1/1,000th interest in a share of Series O Non-Cumulative Perpetual Preferred Stock\",NYSE
USFD,US Foods Holding Corp. Common Stock,NYSE
USM,United States Cellular Corporation Common Stock,NYSE
USNA,\"USANA Health Sciences, Inc. Common Stock\",NYSE
USPH,\"U.S. Physical Therapy, Inc. Common Stock\",NYSE
UTF,\"Cohen & Steers Infrastructure Fund, Inc Common Stock\",NYSE
UTG,Reaves Utility Income Fund Common Shares of Beneficial Interest,NYSE MKT
UTI,Universal Technical Institute Inc Common Stock,NYSE
UTL,UNITIL Corporation Common Stock,NYSE
UTZ,Utz Brands Inc Class A Common Stock ,NYSE
UUU,\"Universal Security Instruments, Inc. Common Stock\",NYSE MKT
UUUU,Energy Fuels Inc Ordinary Shares (Canada),NYSE MKT
UVE,UNIVERSAL INSURANCE HOLDINGS INC Common Stock,NYSE
UVV,Universal Corporation Common Stock,NYSE
UWMC,UWM Holdings Corporation Class A Common Stock,NYSE
UWMC.W,UWM Holdings Corporation Warrant,NYSE
UZD,United States Cellular Corporation 6.250% Senior Notes due 2069,NYSE
UZE,United States Cellular Corporation 5.500% Senior Notes due 2070,NYSE
UZF,United States Cellular Corporation 5.500% Senior Notes due 2070,NYSE
V,Visa Inc.,NYSE
VAC,Marriott Vacations Worldwide Corporation Common Stock,NYSE
VAL,Valaris Limited Common Shares,NYSE
VAL.W,Valaris Limited Warrants,NYSE
VALE,VALE S.A.  American Depositary Shares Each Representing one common share,NYSE
VATE,INNOVATE Corp. Common Stock,NYSE
VBF,Invesco Bond Fund Common Stock,NYSE
VCV,Invesco California Value Municipal Income Trust Common Stock,NYSE
VCXB,10X Capital Venture Acquisition Corp. III Class A Ordinary Shares,NYSE MKT
VCXB.U,\"10X Capital Venture Acquisition Corp. III Units, each consisting of one Class A ordinary share and one-half of one redeemable warrant\",NYSE MKT
VCXB.W,\"10X Capital Venture Acquisition Corp. III Redeemable warrants, each whole warrant exercisable for one Class A ordinary share, each at an exercise price of $11.50 per share\",NYSE MKT
VEEV,Veeva Systems Inc. Class A Common Stock,NYSE
VEL,\"Velocity Financial, Inc. Common Stock\",NYSE
VET,Vermilion Energy Inc. Common (Canada),NYSE
VFC,V.F. Corporation Common Stock,NYSE
VFL,abrdn National Municipal Income Fund Common Stock,NYSE MKT
VGI,Virtus Global Multi-Sector Income Fund Common Shares of Beneficial Interest,NYSE
VGM,Invesco Trust for Investment Grade Municipals Common Stock (DE),NYSE
VGR,Vector Group Ltd. Common Stock,NYSE
VGZ,Vista Gold Corp Common Stock,NYSE MKT
VHAI,Vocodia Holdings Corp. Common Stock,BATS
VHAI.A,Vocodia Holdings Corp. Warrants Series A,BATS
VHAI.B,Vocodia Holdings Corp. Warrants Series B,BATS
VHC,VirnetX Holding Corp Common Stock,NYSE
VHI,\"Valhi, Inc. Common Stock\",NYSE
VICI,VICI Properties Inc. Common Stock,NYSE
VIK,Viking Holdings Ltd Ordinary Shares,NYSE
VINE,\"Fresh Vine Wine, Inc. Common Stock\",NYSE MKT
VIPS,\"Vipshop Holdings Limited American Depositary Shares, each representing two ordinary shares\",NYSE
VIST,\"Vista Energy S.A.B. de C.V. American Depositary Shares, each representing one series A share, with no par value\",NYSE
VIV,Telefonica Brasil S.A. American Depositary Shares (Each representing One Common Share),NYSE
VKI,Invesco Advantage Municipal Income Trust II Common Shares of Beneficial Interest (DE),NYSE MKT
VKQ,Invesco Municipal Trust Common Stock,NYSE
VLD,\"Velo3D, Inc. Common Stock\",NYSE
VLD.W,\"Velo3D, Inc. Redeemable warrants, each whole warrant exercisable for one share of Common Stock at an exercise price of $11.50\",NYSE
VLN,Valens Semiconductor Ltd. Ordinary Shares,NYSE
VLN.W,\"Valens Semiconductor Ltd. Warrants, each warrant to purchase one-half of one Ordinary Share\",NYSE
VLO,Valero Energy Corporation Common Stock,NYSE
VLRS,\"Controladora Vuela Compania de Aviacion, S.A.B. de C.V. American Depositary Shares, each representing ten (10) Ordinary Participation Certificates\",NYSE
VLT,Invesco High Income Trust II,NYSE
VLTO,Veralto Corp Common Stock ,NYSE
VMC,Vulcan Materials Company (Holding Company) Common Stock,NYSE
VMI,\"Valmont Industries, Inc. Common Stock\",NYSE
VMO,Invesco Municipal Opportunity Trust Common Stock,NYSE
VNCE,Vince Holding Corp. Common Stock,NYSE
VNO,Vornado Realty Trust Common Stock,NYSE
VNO$L,Vornado Realty Trust Pfd Ser L %,NYSE
VNO$M,\"Vornado Realty Trust 5.25% Series M Cumulative Redeemable Preferred Shares of Beneficial Interest, liquidation preference $25.00 per share, no par value per share\",NYSE
VNO$N,\"Vornado Realty Trust 5.25% Series N Cumulative Redeemable Preferred Shares of Beneficial Interest, liquidation preference $25.00 per share\",NYSE
VNO$O,\"Vornado Realty Trust 4.45% Series O Cumulative Redeemable Preferred Shares, Liquidation Preference $25.00 Per Share\",NYSE
VNRX,VolitionRX Limited Common Stock,NYSE MKT
VNT,Vontier Corporation Common Stock ,NYSE
VOC,VOC Energy Trust Units of Beneficial Interest,NYSE
VOYA,\"Voya Financial, Inc. Common Stock\",NYSE
VOYA$B,\"Voya Financial, Inc. Depositary Shares, each representing a 1/40th interest in a share of 5.35% Fixed-Rate Reset Non-Cumulative Preferred Stock, Series B\",NYSE
VPG,\"Vishay Precision Group, Inc. Common Stock\",NYSE
VPV,Invesco Pennsylvania Value Municipal Income Trust Common Stock (DE),NYSE
VRE,\"Veris Residential, Inc. Common Stock\",NYSE
VRN,Veren Inc. Common shares,NYSE
VRT,\"Vertiv Holdings, LLC Class A Common Stock\",NYSE
VRTS,\"Virtus Investment Partners, Inc. Common Stock\",NYSE
VSCO,Victorias Secret & Co. Common Stock ,NYSE
VSH,\"Vishay Intertechnology, Inc. Common Stock\",NYSE
VST,Vistra Corp. Common Stock,NYSE
VSTO,Vista Outdoor Inc. Common Stock,NYSE
VSTS,Vestis Corporation Common Stock,NYSE
VTAK,\"Catheter Precision, Inc. Common Stock\",NYSE MKT
VTEX,VTEX Class A Common Shares,NYSE
VTLE,\"Vital Energy, Inc. Common Stock, par value $0.01 per share\",NYSE
VTMX,\"Corporacion Inmobiliaria Vesta, S.A.B de C.V. American Depositary Shares, each representing ten (10) Common Shares\",NYSE
VTN,Invesco Trust for Investment Grade New York Municipals Common Stock,NYSE
VTOL,\"Bristow Group, Inc. Common Stock\",NYSE
VTR,\"Ventas, Inc. Common Stock\",NYSE
VTS,\"Vitesse Energy, Inc. Common Stock\",NYSE
VVI,Viad Corp Common Stock,NYSE
VVR,Invesco Senior Income Trust Common Stock (DE),NYSE
VVV,Valvoline Inc. Common Stock,NYSE
VVX,\"V2X, Inc. Common Stock\",NYSE
VXX,iPath Series B S&P 500 VIX Short-Term Futures ETN,BATS
VXZ,iPath Series B S&P 500 VIX Mid-Term Futures ETN,BATS
VYX,NCR Voyix Corporation Common Stock,NYSE
VZ,Verizon Communications Inc. Common Stock,NYSE
VZIO,VIZIO Holding Corp. Class A Common Stock,NYSE
VZLA,Vizsla Silver Corp. Common Shares,NYSE MKT
W,Wayfair Inc. Class A Common Stock,NYSE
WAB,Westinghouse Air Brake Technologies Corporation Common Stock,NYSE
WAL,Western Alliance Bancorporation Common Stock (DE),NYSE
WAL$A,\"Western Alliance Bancorporation Depositary Shares, Each Representing a 1/400th Interest in a Share of 4.250% Fixed-Rate Non-Cumulative Perpetual Preferred Stock, Series A\",NYSE
WAT,Waters Corporation Common Stock,NYSE
WBS,Webster Financial Corporation Common Stock,NYSE
WBS$F,\"Webster Financial Corporation Depositary Shares, Each Representing 1/1,000th Interest in a Share of 5.25% Series F Non-Cumulative Perpetual Preferred Stock\",NYSE
WBS$G,\"Webster Financial Corporation Depositary Shares, each representing a 1/40th interest in a share of 6.50% Series G non-cumulative perpetual preferred stock\",NYSE
WBX,Wallbox N.V. Class A Ordinary Shares,NYSE
WBX.W,\"Wallbox N.V. Warrants, each warrant to purchase one Class A Ordinary Share\",NYSE
WCC,\"WESCO International, Inc. Common Stock\",NYSE
WCC$A,\"WESCO International, Inc. Depositary Shares each representing 1/1,000th interest in a share of Series A Fixed-Rate Reset Cumulative Perpetual Preferred Stock\",NYSE
WCN,\"Waste Connections, Inc. Common Shares\",NYSE
WD,\"Walker & Dunlop, Inc Common Stock\",NYSE
WDH,Waterdrop Inc. American Depositary Shares (each representing the right to receive 10 Class A Ordinary Shares),NYSE
WDI,Western Asset Diversified Income Fund Common Shares of Beneficial Interest,NYSE
WDS,\"Woodside Energy Group Limited American Depositary Shares, each representing one Ordinary Share\",NYSE
WEA,Western Asset Bond Fund Share of Beneficial Interest,NYSE
WEAV,\"Weave Communications, Inc. Common Stock\",NYSE
WEC,\"WEC Energy Group, Inc. Common Stock\",NYSE
WEL,Integrated Wellness Acquisition Corp Class A Ordinary Shares,NYSE
WEL.U,\"Integrated Wellness Acquisition Corp Units, each consisting of one Class A ordinary share and one-half of one redeemable warrant(\",NYSE
WEL.W,Integrated Wellness Acquisition Corp Redeemable Warrants,NYSE
WELL,Welltower Inc. Common Stock,NYSE
WES,\"Western Midstream Partners, LP Common Units Representing Limited Partner Interests\",NYSE
WEX,WEX Inc. common stock,NYSE
WF,Woori Financial Group Inc. American Depositary Shares (each representing three (3) shares of Common Stock),NYSE
WFC,Wells Fargo & Company Common Stock,NYSE
WFC$A,\"Wells Fargo & Company Depositary Shares, each representing a 1/1,000th interest in a share of Non-Cumulative Perpetual Class A Preferred Stock, Series AA\",NYSE
WFC$C,\"Wells Fargo & Company Depositary Shares, each representing a 1/1,000th interest in a share of Non-Cumulative Perpetual Class A Preferred Stock, Series CC\",NYSE
WFC$D,\"Wells Fargo & Company Depositary Shares, each representing a 1/1,000th interest in  a share of Non-Cumulative Perpetual Class A Preferred Stock, Series DD\",NYSE
WFC$L,\"Wells Fargo & Company 7.50% Non-Cumulative Perpetual Convertible Class A Preferred Stock, Series L\",NYSE
WFC$Y,\"Wells Fargo & Company Depositary Shares, each representing a 1/1,000th interest in a share of Non-Cumulative Perpetual Class A Preferred Stock, Series Y\",NYSE
WFC$Z,\"Wells Fargo & Company Depositary Shares, each representing a 1/1,000th interest in a share of Non-Cumulative Perpetual Class A Preferred Stock, Series Z\",NYSE
WFG,West Fraser Timber Co. Ltd Common stock,NYSE
WGO,\"Winnebago Industries, Inc. Common Stock\",NYSE
WH,\"Wyndham Hotels & Resorts, Inc. Common Stock \",NYSE
WHD,\"Cactus, Inc. Class A Common Stock\",NYSE
WHG,Westwood Holdings Group Inc Common Stock,NYSE
WHR,Whirlpool Corporation Common Stock,NYSE
WIA,Western Asset Inflation-Linked Income Fund,NYSE
WIT,Wipro Limited Common Stock,NYSE
WIW,Western Asset Inflation-Linked Opportunities & Income Fund,NYSE
WK,Workiva Inc. Class A Common Stock,NYSE
WKC,World Kinect Corporation Common Stock,NYSE
WLK,Westlake Corporation Common Stock,NYSE
WLKP,Westlake Chemical Partners LP Common Units representing limited partner interests,NYSE
WLY,\"John Wiley & Sons, Inc. Common Stock\",NYSE
WLYB,\"John Wiley & Sons, Inc. Common Stock\",NYSE
WM,\"Waste Management, Inc. Common Stock\",NYSE
WMB,\"Williams Companies, Inc. (The) Common Stock\",NYSE
WMK,\"Weis Markets, Inc. Common Stock\",NYSE
WMS,\"Advanced Drainage Systems, Inc. Common Stock\",NYSE
WMT,Walmart Inc. Common Stock,NYSE
WNC,Wabash National Corporation Common Stock,NYSE
WNS,WNS (Holdings) Limited Ordinary Shares,NYSE
WOLF,\"Wolfspeed, Inc. Common Stock\",NYSE
WOR,\"Worthington Enterprises, Inc. Common Shares\",NYSE
WOW,\"WideOpenWest, Inc. Common Stock\",NYSE
WPC,W. P. Carey Inc. REIT,NYSE
WPM,Wheaton Precious Metals Corp Common Shares (Canada),NYSE
WPP,WPP plc American Depositary Shares,NYSE
WRB,W.R. Berkley Corporation Common Stock,NYSE
WRB$E,W.R. Berkley Corporation 5.70% Subordinated Debentures due 2058,NYSE
WRB$F,W.R. Berkley Corporation 5.10% Subordinated Debentures due 2059,NYSE
WRB$G,W.R. Berkley Corporation 4.25% Subordinated Debentures due 2060,NYSE
WRB$H,W.R. Berkley Corporation 4.125% Subordinated Debentures due 2061,NYSE
WRBY,Warby Parker Inc. Class A Common Stock,NYSE
WRK,Westrock Company Common Stock,NYSE
WRN,Western Copper and Gold Corporation Common Stock,NYSE MKT
WS,\"Worthington Steel, Inc. Common Shares\",NYSE
WSM,\"Williams-Sonoma, Inc. Common Stock (DE)\",NYSE
WSO,\"Watsco, Inc. Common Stock\",NYSE
WSO.B,\"Watsco, Inc. Class B Common Stock\",NYSE
WSR,Whitestone REIT Common Shares,NYSE
WST,\"West Pharmaceutical Services, Inc. Common Stock\",NYSE
WT,\"WisdomTree, Inc. Common Stock\",NYSE
WTI,\"W&T Offshore, Inc. Common Stock\",NYSE
WTM,\"White Mountains Insurance Group, Ltd. Common Stock\",NYSE
WTRG,\"Essential Utilities, Inc. Common Stock\",NYSE
WTS,\"Watts Water Technologies, Inc. Class A Common Stock\",NYSE
WTTR,\"Select Water Solutions, Inc. Class A common stock\",NYSE
WU,Western Union Company (The) Common Stock,NYSE
WUCT,UBS AG ETRACS Whitney U.S. Critical Technologies Index ETN,NYSE ARCA
WWR,\"Westwater Resources, Inc. Common Stock\",NYSE MKT
WWW,\"Wolverine World Wide, Inc. Common Stock\",NYSE
WY,Weyerhaeuser Company Common Stock,NYSE
WYY,WidePoint Corporation Common Stock,NYSE MKT
X,United States Steel Corporation Common Stock,NYSE
XFLT,XAI Octagon Floating Rate & Alternative Income Trust Common Shares of Beneficial Interest,NYSE
XFLT$A,\"XAI Octagon Floating Rate & Alternative Income Trust 6.50% Series 2026 Term Preferred Shares, (Liquidation Preference $25.00)\",NYSE
XHR,\"Xenia Hotels & Resorts, Inc. Common Stock\",NYSE
XIN,Xinyuan Real Estate Co Ltd American Depositary Shares,NYSE
XOM,Exxon Mobil Corporation Common Stock,NYSE
XPER,Xperi Inc. Common Stock ,NYSE
XPEV,\"XPeng Inc. American depositary shares, each representing two Class A ordinary shares\",NYSE
XPL,Solitario Resources Corp. Common Stock,NYSE MKT
XPO,\"XPO, Inc. Common Stock\",NYSE
XPOF,\"Xponential Fitness, Inc. Class A Common Stock\",NYSE
XPRO,Expro Group Holdings N.V. Common Stock,NYSE
XTNT,\"Xtant Medical Holdings, Inc. Common Stock\",NYSE MKT
XYF,\"X Financial American Depositary Shares, each representing six Class A Ordinary Shares\",NYSE
XYL,Xylem Inc. Common Stock New,NYSE
YALA,\"Yalla Group Limited American Depositary Shares, each representing one Class A Ordinary Share\",NYSE
YCBD,\"cbdMD, Inc. Common Stock\",NYSE MKT
YCBD$A,\"cbdMD, Inc. 8.0% Series A Cumulative Convertible Preferred Stock\",NYSE MKT
YELP,Yelp Inc. Common Stock,NYSE
YETI,\"YETI Holdings, Inc. Common Stock\",NYSE
YEXT,\"Yext, Inc. Common Stock\",NYSE
YMM,Full Truck Alliance Co. Ltd. American Depositary Shares (each representing 20 Class A Ordinary Shares),NYSE
YOU,\"Clear Secure, Inc. Class A Common Stock\",NYSE
YPF,YPF Sociedad Anonima Common Stock,NYSE
YRD,\"Yiren Digital Ltd. American Depositary Shares, each representing two ordinary shares\",NYSE
YSG,\"Yatsen Holding Limited American Depositary Shares, each representing twenty (20) Class A Ordinary Shares\",NYSE
YUM,\"Yum! Brands, Inc.\",NYSE
YUMC,\"Yum China Holdings, Inc. Common Stock\",NYSE
ZBH,\"Zimmer Biomet Holdings, Inc. Common Stock\",NYSE
ZBZX,BATS BZX Exchange test issue,BATS
ZDGE,\"Zedge, Inc. Class B Common Stock \",NYSE MKT
ZEPP,Zepp Health Corporation American Depositary Shares,NYSE
ZETA,Zeta Global Holdings Corp. Class A Common Stock,NYSE
ZEXIT,IEX Test Company Test Symbol Two for IEX,IEXG
ZGN,Ermenegildo Zegna N.V. Ordinary Shares,NYSE
ZH,\"Zhihu Inc. American Depositary Shares, each representing three (3) Class A Ordinary Shares\",NYSE
ZIEXT,IEX Test Company Test Symbol One for IEX,IEXG
ZIM,ZIM Integrated Shipping Services Ltd. Ordinary Shares,NYSE
ZIP,\"ZipRecruiter, Inc. Class A Common Stock\",NYSE
ZK,ZEEKR Intelligent Technology Holding Limited American Depositary Shares (each representing ten (10) Ordinary Shares),NYSE
ZKH,\"ZKH Group Limited American Depositary Shares, each representing thirty-five (35) Class A Ordinary Shares\",NYSE
ZOM,Zomedica Corp. Common Shares,NYSE MKT
ZONE,CleanCore Solutions Inc. Class B Common Stock,NYSE MKT
ZTEST,BATS BZX Exchange Common Stock (test issue),BATS
ZTO,\"ZTO Express (Cayman) Inc. American Depositary Shares, each representing one Class A ordinary share.\",NYSE
ZTR,Virtus Total Return Fund Inc.,NYSE
ZTS,Zoetis Inc. Class A Common Stock,NYSE
ZUO,\"Zuora, Inc. Class A Common Stock\",NYSE
ZVIA,Zevia PBC Class A Common Stock,NYSE
ZVV,NYSE ARCA test stock,NYSE ARCA
ZWS,Zurn Elkay Water Solutions Corporation Common Stock,NYSE
ZXIET,IEX Test Company Test Symbol Three for IEX,IEXG
";