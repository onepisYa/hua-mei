use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! colors {
    ($($x:expr => $r:expr, $g:expr, $b:expr);+) => {
        {
            let mut output = Vec::new();
            $(
                output.push(Colour::new(String::from($x), String::from(""), $r, $g, $b));
            )+
            output
        }
    };
    ($($x:expr, $y:expr => $r:expr, $g:expr, $b:expr);+) => {
        {
            let mut output = Vec::new();
            $(
                output.push(Colour::new(String::from($x), String::from($y), $r, $g, $b));
            )+
            output
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl std::fmt::Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb({}, {}, {})", self.0, self.1, self.2)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Colour {
    pub name: String,
    pub alias: String,
    pub rgb: Vec<u8>,
    pub hex: String,
    pub hex2: String,
}

impl Colour {
    pub fn new(name: String, alias: String, r: u8, g: u8, b: u8) -> Self {
        Self {
            name,
            alias,
            rgb: vec![r, g, b],
            hex: format!("#{:02x}{:02x}{:02x}", r, g, b),
            hex2: format!("{:02x}{:02x}{:02x}", r, g, b),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tradition {
    pub colors: Vec<Colour>,
}

impl Tradition {
    pub fn new() -> Self {
        Self {
            colors: colors![
                "人籁", "renlai" => 158, 188, 25;
                "青粲", "qincan" => 195, 217, 78;
                "翠缥", "cuipiao" => 183, 211, 50;
                "水龙吟", "shuilongyin" => 132, 167, 41;
                "碧山", "bishan" => 119, 150, 73;
                "石发", "shifa" => 106, 141, 82;
                "菉竹", "luzhu" => 105, 142, 106;
                "庭芜绿", "tingwulv" => 104, 148, 92;
                "葱倩", "congqing" => 108, 134, 80;
                "漆姑", "qigu" => 93, 131, 81;
                "翠微", "cuiwei" => 76, 128, 69;
                "芰荷", "zhihe" => 79, 121, 74;
                "青青", "qinqin" => 79, 111, 70;
                "翠虬", "cuiqiu" => 68, 106, 55;
                "官绿", "guanlv" => 42, 110, 63;
                "油绿", "youlv" => 93, 114, 89;
                "莓莓", "meimei" => 78, 101, 72;
                "螺青", "luoqin" => 63, 80, 59;
                "春辰", "chunchen" => 169, 190, 123;
                "麴尘", "quchen" => 192, 208, 157;
                "欧碧", "oubi" => 192, 214, 149;
                "苍葭", "cangjia" => 168, 191, 143;
                "兰苕", "lanzhao" => 168, 183, 140;
                "青玉案", "qingyuan" => 168, 176, 146;
                "碧滋", "bizi" => 144, 160, 125;
                "瓷秘", "cimi" => 191, 192, 150;
                "筠雾", "junwu" => 213, 209, 174;
                "行香子", "xingxiangzi" => 191, 185, 156;
                "鸣珂", "mingke" => 179, 181, 156;
                "琬琰", "wanyan" => 169, 168, 134;
                "出岫", "chuyou" => 169, 167, 115;
                "王刍", "wangzou" => 169, 159, 112;
                "春碧", "chunbi" => 157, 157, 130;
                "执大象", "zhidaxiang" => 145, 145, 119;
                "青圭", "qinggui" => 146, 144, 93;
                "绿沈", "chunsheng" => 147, 143, 76;
                "风入松", "fengrusong" => 134, 140, 78;
                "荩箧", "jingxia" => 135, 125, 82;
                "绞衣", "jiaoyi" => 127, 117, 76;
                "素綦", "suxi" => 89, 83, 51;
                "苍筤", "cangliang" => 153, 188, 172;
                "天缥", "tianpiao" => 213, 235, 225;
                "卵色", "luanse" => 213, 227, 212;
                "沧浪", "canglang" => 177, 213, 200;
                "葭菼", "jiayan" => 202, 215, 197;
                "山岚", "shanlan" => 190, 210, 187;
                "冰台", "bintai" => 190, 202, 183;
                "青古", "qinggu" => 178, 189, 169;
                "醽醁", "linglu" => 166, 186, 177; // ling lu
                "渌波", "lubo" => 155, 180, 150;
                "青楸", "qingqiu" => 129, 163, 128;
                "缥碧", "piaobi" => 128, 164, 146;
                "翠涛", "cuitao" => 129, 157, 142;
                "青梅", "qingmei" => 119, 138, 119;
                "雀梅", "quemei" => 119, 138, 111;
                "苔古", "taigu" => 121, 131, 108;
                "蕉月",  "jiaoyue" => 134, 144, 138;
                "千山翠", "qianshancui" => 107, 125, 115;
                "翕赩", "xixi" => 95, 118, 106;
                "结绿", "jiejv" => 85, 95, 77;
                "绿云", "lvyun" => 69, 73, 61;
                // 深红
                "丹罽", "danji" => 230, 0, 18;
                "黄丹", "huangdan" => 234, 85, 20;
                "檎丹", "qindan" => 233, 72, 41;
                "银朱", "yinzhu" => 209, 41, 32;
                "洛神珠", "luoshenzhu" => 210, 57, 24;
                "珊瑚赫", "shanhuhe" => 193, 44, 31;
                "朱孔阳", "zhukongyang" => 184, 26, 53;
                "丹雘", "danhuo" => 200, 22, 29;
                "水华朱", "shuihuazhu" => 167, 33, 38;
                "胭脂虫", "yanzhichong" => 171, 29, 34;
                "朱樱", "zhuying" => 143, 29, 34;
                "大繎", "daran" => 130, 35, 39;
                "顺圣", "shunsheng" => 124, 25, 30;
                "爵头", "juetou" => 99, 18, 22;
                "麒麟竭", "qilinjie" => 76, 30, 26;
                "苕荣", "tiaorong" => 237, 109, 61;
                "扶光", "fuguang" => 240, 194, 162;
                "十样锦", "shiyangjin" => 248, 198, 181;
                "海天霞", "haitianxia" => 243, 166, 148;
                "骍刚", "xinggang" => 245, 176, 135;
                "朱颜酡", "zhuyantuo"=> 242, 154, 118;
                "赪霞", "chengxia" => 241, 143, 96;
                "赪尾", "chengwei" => 239, 132, 93;
                "缙云", "jinyun" => 238, 121, 89;
                "小红", "xiaohong" => 230, 119, 90;
                "琼琚", "qiongju" => 215, 127, 102;
                "岱赭", "daizhe" => 221, 107, 79;
                "朱柿", "zhushi" => 237, 109, 70;
                "赩炽", "xichi" => 203, 82,62;
                "鹤顶红", "hedinghong" => 210, 71, 53;
                "纁黄", "xunhuang" => 186, 81, 64;
                "棠梨", "tangli" => 177, 90, 67;
                "朱殷", "zhuyin" => 185, 58, 38;
                "石榴裙", "shiliuqun" => 177, 59, 46;
                "朱草", "zhucao" => 166, 84, 54;
                "赤灵", "chiling" => 149, 64, 36;
                "佛赤", "fochi" => 143, 61, 44;
                "綪筏", "qingfa" => 158, 42, 34;
                "朱湛", "zhuzhan" => 149, 48, 46;
                "丹秫", "danshu" => 135, 52, 36;
                "木兰", "mulan" => 102, 43, 31;
                // 粉红
                "杨妃", "yangfei" => 240, 145, 160;
                "盈盈", "yingying" => 249, 211, 227;
                "银红", "yinhong" => 231, 202, 211;
                "粉米", "fenmi" => 239, 196, 206;
                "桃夭", "taoyao" => 246, 190, 200;
                "水红", "shuihong" => 236, 176, 193;
                "夕岚", "xilan" => 227, 173, 185;
                "彤管", "tongguan" => 226, 162, 172;
                "咸池", "xianchi" => 218, 169, 169;
                "莲红", "lianhong" => 217, 160, 179;
                "雌霓", "cini" => 207, 146, 158;
                "縓缘", "quanyuan" => 206, 136, 146;
                "长春", "changchun" => 220 ,107, 130;
                "渥赭", "wozhe" => 221, 107, 123;
                "红䵂","hongzhi" => 205, 115,114;
                "紫梅", "zimei" => 187, 122, 140;
                "绛纱", "jiangsha" => 178, 119, 119;
                "如藘", "rulu" => 163, 95, 101;
                "美人祭", "meirenji" => 195, 92, 106;
                "唇脂", "chunzhi" => 194, 81, 96;
                "鞓红", "tinghong" => 176, 69, 82;
                "葡萄褐", "putaohe" => 158, 105, 109;
                "蚩尤旗", "chiyouqi" => 168, 88, 88;
                "紫矿", "zikuang" => 158, 78, 86;
                "紫诰", "zigao" => 118, 85, 93;
                "苏方", "sufang" => 129, 71, 76;
                "霁红", "jihong" => 124, 68, 73;
                "蜜褐", "mihe" => 104, 54, 50;
                "福色", "fuse" => 102, 43, 47;
                // 紫色
                "黪紫", "canzi" => 204, 115, 160;
                "龙膏烛", "longgaozhu" => 222, 130, 167;
                "苏梅", "sumei" => 221, 118, 148;
                "琅玕紫", "langanzi" => 203, 92, 131;
                "胭脂水", "yanzhishui" => 185, 90, 137;
                "紫茎屏风", "zijingpingfeng" => 167, 98, 131;
                "红踯躅", "hongzhizhu" => 184, 53, 112;
                "胭脂紫", "yanzhizi" => 176, 67, 111;
                "魏红", "weihong" => 167, 55, 102;
                "紫府", "zifu" => 153, 93, 127;
                "魏紫", "weizi" => 144, 55, 84;
                "地血", "dixue" => 129, 70, 98;
                "芥拾紫", "jieshizi" => 96, 38, 65;
                "油紫", "youzi" => 66, 11, 47;
                "紫薄汗", "zibohan" => 187, 161, 203;
                "退红", "tuihong" => 240, 207, 227;
                "昌荣", "changrong" => 220, 199, 225;
                "樱花", "yinhua" => 228, 184, 213;
                "丁香", "dingxiang" => 206, 147, 191;
                "木槿", "mujing" => 186, 121, 177;
                "茈藐", "zimiao" => 166, 126, 183;
                "紫蒲", "zipu" => 166, 85, 157;
                "紫紶", "ziqu" => 125, 82, 132;
                "拂紫绵", "fuzimian" => 126, 82, 127;
                "頳紫", "chengzi" => 138, 24, 116;
                "三公子", "sangongzi" => 102, 61, 116;
                "齐紫", "qizi" => 108, 33, 109;
                "凝夜紫", "ningyezi" => 66, 34, 86
            ],
        }
    }
}