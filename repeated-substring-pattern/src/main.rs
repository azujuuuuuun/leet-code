struct Solution {}

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        for i in 0..s.len() / 2 {
            let substring = &s[0..i + 1];
            if s.len() % substring.len() != 0 {
                continue;
            }
            let mut str = "".to_string();
            for _ in 0..s.len() / substring.len() {
                str += substring;
            }
            if str == s {
                return true;
            }
        }
        false
    }
}

fn main() {
    println!(
        "{} {}",
        Solution::repeated_substring_pattern("abab".to_string()),
        true
    );
    println!(
        "{} {}",
        Solution::repeated_substring_pattern("aba".to_string()),
        false
    );
    println!(
        "{} {}",
        Solution::repeated_substring_pattern("abcabcabcabc".to_string()),
        true
    );
    println!(
        "{} {}",
        Solution::repeated_substring_pattern("gcdzzvyyqqrcfxjneckbdfjgeaztbdknqaanfrcxrrbbvqlknuybtghqirazuzuwbollodhebmkvjvcuarhmczfzbnavmhppjlivahjnwcyzgzptyrjjlbbkejjekworvdbsgnisalrhnzbwywadgbxcexjetwyqrgdxhszdwqcplkydsncctpplhrbhewyevgykzikrdcuznnfpngskxseoikwkbsnfscaxknkhvppcrswvawphuqdazitsczglhfykvqclmyipzbnbsifniuohgcrkgjrzjioipewwbgkuvksomjqgigsnsndsqjtvlwhgyqchkroscetciiivjjpwiblwiygcrwzbvrjxuocjwyrgtsrwwggauybxzjzfckqfhrwlxrldflxmvpnvloqbowxobgkadmaigmrcerkxppfnkzmcrdrzogqbengfeaulhkwprigrssxnvxvparcckpgveqqlljfopngrnfmjpmkcuxnjozdeeieuxxutudqzzogufklymjqbcuxoiupsshvetyylpgjcfctdqizvxecscwmcvevcauofvahhbdjxgxgstwvfyqesjabtushtdvxtvpaiefcqhaizabrjfcbjefulxwfliccezhzdljgffkkkcwxyiymoizuvmnebnsiksqnjfbhxfwwkioobeycgrlyjzusiwhnwyfiecoyqxwgsfbrortiwjijkybpxyxdgkepbsvgfdrhmqpsofhmdyzxdhukndasyhjpzsrggldhfsrwntzoudasorkkklgdqeqslocoyqqzxpkegfachwvwdlfepeiqapdtuxjvbhuktbeoigbpujijdvnzjzlvdrpmuuhynyjnvxnywalrppabfgokdpgkrejpsecqdvsajztallucbhiwkiafqytehcrophrpharrlcwpcwipcngbzihxqzxsdmlvmblfqqxhvxgwlhrdtngwikjffqcjzmnitszpxohdxlrisjadqacptzdvdbsuocxtwtsapvheebjqresbmhpspkeosfsoarbffpyoetohcauevbtdvaiviqsnvyfumltrxjorrydsqfnjsdtghcvutyvswmfdijmrsekksuiaivholmqadchznmiwpfezyhttavmljoaqmlrtxsiyvongfscjfdbhzhruzpmhfzduvjvhgtiiabyqcmyijnynyhutsfxyrafyduqhcqbmigqlgzaiocypgsjflrlcwbvgafpekfkxtbrvgouprkzieqvcpmbcwljrlpeakgxcufxnwdsnqgzqiqpslhmozsafrkwyiynmbennqtmxpuilpkvsvlfaishylytxbsiligzifmyhpuiuabonlrtpihzzmxpxxdoftbciqmanynnqugbauaxkzffrkzflbtpczecbsufdrijrhquuitvfepnklccvtlabupupeljccrvibwncellcnjhugbmmhxeeqrhghmsslemrdhmwyczgaohtfmemrbzsohfbdxchwnotbqfxehkrluinlgoimlbvqkzliyrhyyonhiyjkpsrylzsiyqndsuldvkhjwgeivaoombyskwxcakaeasrwkwcxzkqkdncevgyhaxfuyjjdqlkpglpxuorlxohnpeeuzwifejhqmzhvirachcsaavzbqsbheiycwqramnnvnthdsvnpwdqlfdvgyaaeyvfazsosisbctjehtethdlinresiauqwlciheneshdtexddiicntnylccedcordbdnsamzzctyhovvqowlaegqaezwqbivzkynhuukdduzafyfadstpkqevgnvvxvtcatzezkhualbqaysegqtqsqagpsnyntravascpcqqozrbofcoepqlnsotbqqekwffhviulfggwkkqubbtypwnbnvplqwuxxiccsacoqvzwlfttrkkglrpohxfhuhaznoiaefbykdfzujknmwxetbutyrmgyngpdqhjpxvyfasgiwolzaxxlxujthexbnxbehrdrdkohbxatpfqywtbumqksdkgpopidxaihrdqluopvjtjanprzdulmgfmftrtaxyisstbwqdbgslzrumxjxrtzsgbknnjhsyzzjrdpcxqajhrgvnibjhpztgfhmmpyxxaboelsdtaicmtiigyccybubxlllnpbsfrhqygupszvdlrnzxtjfvlpcrpdxgshuoyhkykpuqmobbexzebqrowgqctkggarqxgdljeoxviieyoydtzpydrwidqujcwgptppvyrtfnthchefbwnyknouthlyswcfzhogxrsrkkwrqxfwfftomhwedpaceillrfnnudatzlppimcvwponqdrqrihqubrkphcwfinpzoukbeibtylvqpssdikbpczjeprytsjjwtjhnjingwptfpgrbeolrjjohuazawobhadduyzsvhnmpbthcdpskhhrsnjzugshemigdhsesuaztbctmprsobistxghinlssegojmzouualrvpoxptvzlfjgtfzaklkoyvklzbaetxhspireyuwxwjxnprjvdghoiynxcalcsycxffcrjyssygrcdmlpwiltwwspvpmyhcbmavneckchhtmhtdxkrznegbpvprlkssrasqggubefsfnkdctmbebjacrfganbczajnfuwkyctuujevtypluoyshvandnptmcszbrhieioxpsoozbcdhhlmdibedjvdrrrybcksxzdcrjxjqowcovyzprxhsuxjpcufmrxsrnpsbtlpiblpbiacdtxgznlozzvlhxknyvzwdpzbqqtexwmdzirpnavkkvdlwcodfqwfuuywlstipvlwzqurezstkdumxmnvhnglrvotkxuhfingbplqoapoytwztahmvfalewmxgbvmyngkadsgmuwbfvlyrgzmndvvbegixruffkmodbvqmjfeuyzpgvemfbphzemoclrshgmcbjcppoydyojvfyzxajewnipkdrxmgdstpamxlmtxbunngnymdvngiqjimkettqysbexxljojmwjkhplsgvidgyuotigpraetcquwldmezfwovokhwzyczsstxpnqzrhckmuwveykuiuderjzsevcrflpqvqbzsnjxaacqvbisghwfvscqtpthqowqzbhrgkomhyaddphkjryfzgwodsghsiwhqwvkiiqlfpjfxpcvextjnrbbyqnhzrdlqfjhavvjhyxifxuthkogexfmfbcnimesoyjvhyvfmlooajmedzitmdkxdiobvvfdlnsnmgzdfkhgpaspmyoabjeqgfrwqgeuuhwazdcdnvzeyizivkoraeltoudxtgyeznedcsixehgkcadazebxzbjvcszbjjtgmahfnnoghpmfbaphmrtovqojfrorhduoftngxczmnpakpvyukwzdeqvivakwmgjgefkgccmepewhdavbmviqmgfmwqywhpscezazpasosyifruciakidbefuhptjrgyjrjfmzkolzmbtnhkzpskpyxpbadosnuglpnhwjsmmdujjyyloknsghwqorjxiuxknbagewwdlvnrrtlhoduufxlknqvfdltksparmtrixqwxzqzslcaytfnfitinuguedbjmjylicamwvrgxolyuhhxcbviwbfdufhjqzcrzxyyllxqbigjklbhafpmrvaygedmjqxeqmhpagpisuavzbizvilzbukqemjzqnkzsqcxhbqtlxqhzateftdywdqcscjuckhlmizpizjjyjldvmhtdqzwuajrwvtswiirlhbkscjyqvzyxpfomzsqzdosuhhbjstjexokfujhcimqmxodnhmezbsoutoufgaohbynsbemzkmgtboicgjapqkkchysylisflwrkfmmnwjenhpwgjujfbxlamhvjorawpytbdusrlgephcbgbmunqppmeezykhxgihwvvvnhwewvlsrevrgaqgyhsedrnqgsbguwzgkudigfqekldckontfcnesncexenocioxtymisotvxyelevwkmikftudalpqbqzozbudsxdfojjivpfrsefjvtyhmevwtgpnrzzhsjxyxbmrtbkmytshrlqilnrxtngwujljctnalsnwsymdyhdwicrnqjqjsvxowagbunjjvbcfstnacdmjohpttasngjjdsijnaexfohrnpeyvikqrybmvzjhrhjifnbtbyvzvoqlzqwmzobdmsuhnwdffravrqudpetaeronqzzyuecxlvywphpcbhdaxbhlxaatfrdlurzicxjqmjrpddkuiwlelamuikvrxpthxzjzwjabpxaskmcczeprqbmegykxqgkfjybxlfcgtvznpszzxxsituyjblfiyzcmxpnyvwedebmqguviiigzrxzdoozcsessogougvpmriaurueirfjuewhzrixbuuohkgigqqcxcformwykhfzfmlssspqgdicewmtppfvsjhtneubvjusmbccelbryzvuomiqkgcfovherggkmmtghjyfsszfsiwhtytyhpuaenqzoiepabdzvhlxaenuoleihwvkuzkjjbbzozfqhzmkkhezvbnwamzvgbltptzexdzudkqvyctpvagwmttywcuzyngqwxnbdpakuvreifbwyikuzrnzcntkunzwrbsbzhiclsqqkclaetqkiapdmbfqyfpkihpfembsxyrtuqupmqrgthtnkghuucqhnxyuucthbczxbxftdaoyirdfxszcumitfzzzukxfetovixeoypijkhxzeldpybsnkmrhjpmczflpvzxdevufovbdkbnfqemljfyaxrrwpwzbhwwiwgvmjanmoznyxjnfmfdopcaiuvksjmmnrvdjtywjtfalxypxwgzarsaggjfibxhcowpsfvkwmiagorfpnlluycelnngrpgnywdpfyxtczyifiyihfugwkikmnvubjzefufneiiwilfazezulwepvmtvygcoduaxmeehgexprxtwcmfqpwystrbmvzdlflmjhcocyiqlfsjydhtpwpogqmtemmlpfhjyfhcfvovhdjrjijmzszfgxiinutlfsnbiqnpdikztysnspvusyhqqqbfetlwrrboamkxccdhwhwrmnginsaeefrlwodbczyutpapqtiruscyymbpcgycytfnkxxpcxinhgiuvrurrveuekknfyvmjhivpbfgdikqikmnohduaxcmndqqazusugaaewlgujbjyklkcobxscaivpnnyqnfkafxdmoqfinojztevofjmjwurwmqduhklymwoxoetyxibhrmpegyiwmxmaqorpykjladzppythuwcgtsivwjceghnylwizbtpdhqogpuwyqmkvndyiqbmydfvgazabquwftghwesasejxgsaaxvtdfwpxrbyyplvsigwfcmsdkmkcdkgrpuheoxbdgruitsodxshnaxizebwlfuvpkukypsaiozivbjxlnyuwcwvggnjlsnblrzsgjtsjdyadqemxcqkncljtmygdbzrfpapsgfbtyibjxgsqtsxtxicgrxdhlawlwzdlnxgjtojyejtkkpqdjysgyuperosctuyuhcwmheoeadqrofsrrjtfksxwcbsliyvylkgyfmhxwldmyieybkfxfxugxlneikswlnicqzwxnkkweardfzyrxmkqcnmarroedzqhyuyinsjsfbqbcncoopxpmktljdyoibsgrtthnsntipnntdswggfzmenapjqdlqcvskfodegqrvuiuncqbksnivfaygxatfdqcvukflbcmelomvfpicohivzdglhzeabvlddgidytlrocvrbhfivalqlrzhbhcuptltnckjgnrohjwkqoinfgycnayrqrqwkqzmuoqxhcjankevwhlpwibjvfvdnfbaeiqgtyufgchxehzaleiicwbbminjxbilrungfjdygasvztrxiedvdlfcgbgutqqbuvpovuvvqcristdobriozyvylhhonxsastorkruuvtjluhvwypbtbcxuysrzznuuwmnrpkafnvzeewsibovfimthqigrkjibesianqwskwdritimugwovjkpuahnfmhsfljvrltjnbotjpwjzxnetxhljtwucvlwakazrorzrsbvfhadpqpotqyjqhwvyvwndqdysuuotfsoejokortfpgqybpwlmpjwydydjczdedvkuacyrnstazrrzaincaiahdnyizvdrmudfwlhmwxcsctrbqljvtekmcjcrtgyhtfgpaloqgfwuuezlcjsfneyhwaonekhneiqfcpjvrpphrvtzotpajhsevjeyewjutwpnpzhulbulussazygaiprppogjfainxqliqwxxfzelpjijcwzhhewznzyuerxguctggyrzitwrigvbjtreametaiaossuvtofeqgubmgucdgjmipilupqirkpiinczfhgeqtywxfrdpupgomdqcamdjwsmwtrpkhkwghizsfanlbfljsumcbwbzdtpasmiejjjcacykihkevsgzlslavsowfsfasnnsfhuundiyghflprbpnjjovikkmeitiymwmdlhtdicbxrpqeacexeriwlntqudtwymcoktnxoayytglijxxvrtwyyyxxsqgqkjhgfhcjqhqchwfqemiheddfqrnvqsgiukfseligagbchtkscnmbmrbvaecuahxwzffbvzmnktnzxvembdnjumzhcyxtappiqpnoipmpazjimxdofrwyxtetunxhdtuqvnygqsgkgerunawbhllyxirmbftbkzhjwelomogmzimlpjklhgzrvxjerzkgwkddgdxfwypxkmjleirdaacbxcasgiuwzgjzsfmeitetmozvtvukglclkvafaywacryevzjjvdmiwfpmprtsoanypmojtekfduumpuufkrfeeakwwphvqjgkvjsthnrhjxhpmpracaruxvsrdwbfhabwwhazosfqgriykenckywhqxyaqtkgdvuvjnvlpjxfmguxtgyhypvlhqnscfqbmcsalavqrgutbeejiulbcrwrmphbgenntnimmimzyvcvopinntmnqdakbhuzxexsrmnqlgxyupcjoeohpqpsgbbqjxyeoevpbovlelwwmdrikudjtmzstgkojvgikuupeeagzaqnsmuxiscqydocsodevsyiflybaqvcszgqxzsbtcsmzjdnwjoxcekytjtzskpytvqvtagibgaxgauxjedibdeijpmeedqupyswmptexhdbsqvuuhqqkpwfqpgncedzomsxiyvbiurykgjcgltsbawqdnbxcrryflmzgqgyjuwxlktzdxogtxmwczdefzxaztrmhughjtoyilliaghqqnlynvtgguinyquxtpuuycpzfofqxmmlnnmnjigidirzqgmjmkrsxgrjrquyfeyrfuuqdamomxxgghzynnlemcowhqpuuetusxwvjnitorwjztmykwhscowstsfojslrclnwoelmoguqtfmknvgjvezgmiliqabftvmxvrjeixqjvddlyqzlxdwplnxcomsvmiixlotkbstxhfkfwxilblujxropyybqvpvfpceehdbvvfzcnqnceogkgiqjrmsfdmauurxpvmgihqqyjxpifitgwokbgxvhfmvagfegbevtqztmoaaktdvinimilioihdugvifhcyenvfldosukxxulutuezajmwfskuggrumeuzklfnxtqmcyerdhobrrollwsinxfiiwcemxrpzqqpkyddfihubhgaydfzgxrcuojcfqiigxcysdhmrvgaxxdwywsykowozpgdvbhcyrxqsrzkodzadojhbynxgfczwkziqgbrpucmbduiblwtfizncsaoxptltfuamramamoyckdisptuzdzhprfltzbqmodytuyqvqdyqnpzkyggwfznzcrthstgasjphzzrzksbvkzneafsfitxcssaihvlgkhvbkogheqdqzrscgrykoxtctccsidkuetlymldxdlmzyixcxcdekpfhelspcjvcmbmvybbeusrnwtxhsxnztjxvgwnxtfixbvhzkjaqrusrinmpdhcuvykxlkibocstdbitibilfkjeupswavaplmqgoiumccurmgipmppknpgrtowqudhbpnvfwzzojilkoeywrqdqgfjjnvrvxfrvplsqrgfihyqizudqlhtluugzqlorxbwdxqklsnqbrbvymzevyoteqouvvlwexhyuwkfdmcgaykubecejepmdxfvypuhlbmytxmwcpfnwwpdggvgsdmghdxfpqhhwkcyqslewpzubdymifegfakqdhjhwekwpfqwqagdqzvfnkkwtlfnlvmpmwssrmgquwhcqpodutmjeerbcpysobphetitiugdriqnrjsdsxkgqyczfozjotakgemuczogjbfwylqqdbtvzktvwhcanyqvxqyttwrzatjomjjorbipjvpvtxdlopgajcielcqjypmvljovozgbpdhhtaeehjahcvvmhwudggpjhxhjdxvzixzjvjooosceqfcyebqmodfdtnjpvymvdjpygbaamsjsjrtwinsrafgbjsrslrcazpumbsmuuaisdiacjlysrhdlfjbqxlkdtaihdfbpzixgrvprgtesxwgpgmhxvnemykoiqrlwzclghcmywcghltqhfziokgkbvynlmckiznavxgmywgkkfacuetrlqgmufeqvflunxlnwyawzkmoconujmovkgnksabofkytdmtubriyxvhkltgnddfiejyjrziwfqwcgarqhxicutzekyhwsgphmhmiyttxenpbgbqfuixzypanchvzqovsjanjsrjatdmugavtneptptdqejharuckmcgirvnrdahjlhvacsfsozyecgsmhtoggjmdaeejmznmnatqwtoljrzcivxcjfdikymjmycabrzhdlzjxmnqgzobbszkqhobvhrutiaftetzixxwqzpmdefvacupqjeimtmtefdnxtzqmocqveljxfukjivzwupfxivviuecxihshaxyuvitjflimwbdtstxiutljvjskajqicpfndrpwlqxduelxrxxz".to_string()),
        false
    );
}