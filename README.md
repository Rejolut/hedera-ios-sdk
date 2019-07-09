# Hedera iOS SDK by Rejolut

# About Rejolut

Rejolut is an award winning company building innovative digital product for the fast moving companies using emerging technology like Machine/Artificial/Deep Learning, Blockchain, IoT, Augmented/Virtual Reality.

# Why iOS SDK despite [hedera-wallet-iOS](https://github.com/hashgraph/hedera-wallet-ios)?

The Hedera Wallet iOS app relies on interacting with the Hedera API protobufs direclty. There is also no existing Hedera SDK support for smart contract deployments.

# Is this SDK developed from scratch?

No, we have taken [hedera-rust-sdk developed by launchbadge](https://github.com/launchbadge/hedera-sdk-rust) and built on top of it.

# How to build this SDK?
1)Clone the above repositories<br>
2)Cd path to the root folder<br> 
3)Run this command cd <b>hedera-sdk-iOS cargo lipo --release --targets aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios</b>

# How to run this into the iOS?
The above steps will generate the target folder. Go to target-> universal->release
It contains libhedera.a which needs to be included in the build phases of iOS project.

# Is this modified SDK controlled or managed by Hedera Hashgraph?
No, this SDK is fully managed by Rejolut and Rejolut is responsible for the existing modification.

# Is this SDK free to use?
Yes, it's free to use with an Apache 2 license – as is the SDK it is developed on top of [Launchbadge-Rust-SDK](https://github.com/launchbadge/hedera-sdk-rust).

<b>Feel free to contact amit@rejolut.com for any Hedera related development</b>


