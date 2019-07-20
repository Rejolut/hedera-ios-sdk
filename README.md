# Hedera iOS SDK by Rejolut

# About Rejolut

Rejolut is an Award winning company building an innovative digital product for fast-moving companies using emerging technology like Machine/Artificial/Deep Learning, Blockchain, IoT, Augmented/Virtual Reality.

# Why iOS sdk despite [hedera-wallet-iOS-sdk](https://github.com/hashgraph/hedera-wallet-ios)?

Hedera sdk does not have the smart contract deployment capabilities.

# Is this sdk developed from Sratch?

No, we have taken [hedera-rust-sdk developed by launchbadge](https://github.com/launchbadge/hedera-sdk-rust) and built on top of it.

# How to build this sdk?
1)Clone the above repositories<br>
2)Cd path to the root folder<br> 
3)Run this command cd <b>hedera-sdk-iOS cargo lipo --release --targets aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios</b>

# How to run this into the iOS?
The above steps will generate the target folder. Go to target-> universal->release
It contains libhedera.a which needs to be included in the build phases of iOS project.

# Is this modified SDK controlled or managed by Hedera Hashgraph?
No, this sdk is totally managed by Rejolut and Rejolut is totally responsible for the existing modification.

# Is this SDK free to use?
Yes its free to use and its developed on top of [Launchbadge-Rust-SDK](https://github.com/launchbadge/hedera-sdk-rust) so all the licensing applicable to it will also applicable to Rejolut iOS sdk as it is developed on top of it.

<b>Feel free to contact amit@rejolut.com for any Hedera related Development</b>


