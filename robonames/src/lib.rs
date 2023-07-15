mod dicts;

use dicts::{ADJECTIVES, NOUNS};

pub fn print_words() {
    println! {"{}",ADJECTIVES[1000]};
    println! {"{}", NOUNS[1000]};
}

pub fn generate_nickname(_hex: &str) -> &str {
    "nickname"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjective_dictionary() {
        // Checks a few randomly selected adjectives
        let adjective_4 = "Sweeping";
        let adjective_1683 = "Atomic";
        let adjective_4774 = "Satoshi";

        assert_eq!(dicts::ADJECTIVES[4], adjective_4);
        assert_eq!(dicts::ADJECTIVES[1683], adjective_1683);
        assert_eq!(dicts::ADJECTIVES[4774], adjective_4774);
    }

    #[test]
    fn test_nouns_dictionary() {
        // Checks a few randomly selected nouns
        let noun_118 = "Address";
        let noun_6540 = "Null";
        let noun_10937 = "Zombie";

        assert_eq!(dicts::NOUNS[118], noun_118);
        assert_eq!(dicts::NOUNS[6540], noun_6540);
        assert_eq!(dicts::NOUNS[10937], noun_10937);
    }

    #[test]
    #[ignore]
    fn test_nickname_generator() {
        assert_eq!(
            generate_nickname("23d022aa5dc633f2f115e48fc1f393f051ebdec3dfae41cfcd01bdac3577017f"),
            "EntertainedWin410"
        );
        assert_eq!(
            generate_nickname("e16b82646823b51bdda76a2eb2147afbe57b2100af46a61adc0671820a6dc991"),
            "FrankVitro385"
        );
        assert_eq!(
            generate_nickname("242bb123c13a7cb24708f16000fbabe1c4eae06b6912b47e6b0c2fa08307ec0a"),
            "VernacularBoxer512"
        );
        assert_eq!(
            generate_nickname("29c7e1d03d109bcb6af4057c84670702710a9261e16ca6b77a21f5f950644133"),
            "SwimmingPuzzle724"
        );
        assert_eq!(
            generate_nickname("70e2b8e0cc3daa4d2a0c622c46f30a5507b4f66feef3893e9eaccfce3f8ff0fc"),
            "IrritatingLimp990"
        );
        assert_eq!(
            generate_nickname("2f26321bf6cf99dea51a1664fc812ee3d3123313b9b7abcc79d4bbbaf6b9a470"),
            "UnknownMap519"
        );
        assert_eq!(
            generate_nickname("8d9531f05fde6f2e44961b113141e1625f20d17e25101c35071660f8e6a705f0"),
            "UnworthyHymn429"
        );
        assert_eq!(
            generate_nickname("29e6baf2d7442963517e93f2b4fe6a326a0474410ce916778ce800e7cb6524af"),
            "LegibleTract390"
        );
        assert_eq!(
            generate_nickname("672e48fd4a7d2a28923cb637ff9aca48ad398ff058ce39d48d1463ef5396ebd3"),
            "FidgetyOverhaul213"
        );
        assert_eq!(
            generate_nickname("f387da94931be5545e8b618640a27ec2cfc2fec99570f477a3f0057d7c8de597"),
            "GeographicZeal110"
        );
        assert_eq!(
            generate_nickname("0704bc96c5909c47d2461ad161db271f43649aa5331b514b7b484c0cecfd72f9"),
            "HumorousOriginal57"
        );
        assert_eq!(
            generate_nickname("44623105ab0e9b17674cf0aa4df81b5a8acba636f1cf2f4bf03c516b53853be3"),
            "IrritatedApex167"
        );
        assert_eq!(
            generate_nickname("72b1e55e5afce59d9f59f115d5ffcfebbb623d17fb4ed14793655fba854b26a2"),
            "ComicalBicycle160"
        );
        assert_eq!(
            generate_nickname("2e5fb21fdc3c628c29707d89b08b8cfa0f1ced10cf90aa3c42ad528cee0eca45"),
            "StinkyFormation19"
        );
        assert_eq!(
            generate_nickname("1057a3d72d96b643ebb924859705f1b385371210cfbfdf78eedc8e5f9c34c137"),
            "InaptMajesty747"
        );
        assert_eq!(
            generate_nickname("1e7ad4d15162c3f813c542d813ffae5f027aaa0c174a7cf3a9840fda20881824"),
            "SolitaryWear892"
        );
        assert_eq!(
            generate_nickname("5881365a9e19ed77f479d6aa28bbc0a4b2c879a8b75debaf87b327af377acc35"),
            "ChivalrousOlive8"
        );
        assert_eq!(
            generate_nickname("63c2b815132c79ef3a874f4138b6786c19acd94556d62f1170d55af40b3def92"),
            "SurgicalStink272"
        );
        assert_eq!(
            generate_nickname("682258198870f90832f0854a8bee08eb587438a1dd5b12ada901676907347d38"),
            "ScarredOrganism49"
        );
        assert_eq!(
            generate_nickname("efa4068f4e6825526ced61d8b77f6bd237f2f9d001672cad272e2fec8474b213"),
            "SatinDiffidence476"
        );
        assert_eq!(
            generate_nickname("735f64aa04f66be52aa32ad39344150a704b6142a03bb0248b8198a9e2374e1b"),
            "TactualExhibit853"
        );
        assert_eq!(
            generate_nickname("015a4b8ded8107c4bb37763d25b22f0e7be64be5587b7609744b76403ee963fa"),
            "UnhealthyBuild495"
        );
        assert_eq!(
            generate_nickname("006833a6d33c4e4d9d3785505f6d68a29b5e5e9b40d6767a5ae017dc6b3872d0"),
            "OnerousMast982"
        );
        assert_eq!(
            generate_nickname("d3f5e6c547af1c268bfc00bc92bbdbbbf80df2e30a8eaf5f2a199a07e0320cda"),
            "PertToner606"
        );
        assert_eq!(
            generate_nickname("3ee5dd464116bb1cbe225a07d4577b459cc49da215db0dec7e832d8cec3a6ec2"),
            "BloomingProduce238"
        );
        assert_eq!(
            generate_nickname("0c007605495eb709f5572fcdef6acec89e3fcccf3cd0d919ed305904771c0b4d"),
            "CuriousAdhesive448"
        );
        assert_eq!(
            generate_nickname("5716513f0e782df1fa1c7b4b2bfc5b74416a4e52251c8cecaf890413d32e3624"),
            "ConvolutedOnset713"
        );
        assert_eq!(
            generate_nickname("84424fd8f3b996448737b7b7e1649b8856d042fb63e739fd22c6e0a1a4d47a23"),
            "GregariousMix774"
        );
        assert_eq!(
            generate_nickname("3544ad1a5ce79496d3920c26aa5666f6c77222b18458a679543041c54c309ee7"),
            "BrawnyEtiquette688"
        );
        assert_eq!(
            generate_nickname("e1b9ebf74c15f82f9b94d9cc1b5de69000f61a98d3d456086436cf0a04ade2d1"),
            "AdaptableBushel935"
        );
        assert_eq!(
            generate_nickname("72914bdd87104d8edf5d375d1b2aa64aac4a0d930da925d65bc91dd9a0ec89a5"),
            "FilteredMale710"
        );
        assert_eq!(
            generate_nickname("da3764eb861e39c5c092b7c18c2f7b64dd29afe8b1ba7c0ba66d5d344709d5b1"),
            "DetachedXylyl936"
        );
        assert_eq!(
            generate_nickname("40dde35f9e54a568783152088dcb8867c4201aaef6282e8eac358600577fbcb7"),
            "ThisHick743"
        );
        assert_eq!(
            generate_nickname("92c64aa3ad5474072b5428f1900c0ecf7a404e7db824d789e68124d3124ddfc8"),
            "FastHousehold302"
        );
        assert_eq!(
            generate_nickname("023d51ba37e49ba8fd3c0d2256d52f8f24049923a7a87fa25d4051d9e8c0db65"),
            "QuietExcise380"
        );
        assert_eq!(
            generate_nickname("703d70855f2362a851dc50675b0f1e33b22cdf5550c4a47093b8ecad5084ef04"),
            "VisceralSunspot235"
        );
        assert_eq!(
            generate_nickname("344c8af920e3daea139b6b4f557ba2dc88b8c8e780751c50eec2bfc34cef9fb4"),
            "TrainableStripe521"
        );
        assert_eq!(
            generate_nickname("a5b28e037299321af4acca4b4800ff96812085610a9396daa2e72630831ca954"),
            "ShriekingOzone191"
        );
        assert_eq!(
            generate_nickname("6f8f86121107866dc1993d31901320425041edb22ae819086a72f513cec0a026"),
            "EuphemisticRope564"
        );
        assert_eq!(
            generate_nickname("9004e73f27dc2dbc2e7e2f94d1c1adb8f9e18774de9e384e6f3658678af86d49"),
            "UntaintedHill937"
        );
        assert_eq!(
            generate_nickname("e2c7a42525878575087b8bbb6315d9c171925dbe6322272e55e631ada1bb458f"),
            "LeftHook809"
        );
        assert_eq!(
            generate_nickname("52efa730b1259e501831fbe0e6f3d0544a6181d5ce39f16a484d347ca1eb6aa7"),
            "MalleablePorch278"
        );
    }
}
