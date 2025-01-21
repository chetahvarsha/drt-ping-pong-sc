// Code generated by the dharitri-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use dharitri_sc::proxy_imports::*;

pub struct PingPongProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for PingPongProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = PingPongProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        PingPongProxyMethods { wrapped_tx: tx }
    }
}

pub struct PingPongProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> PingPongProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    /// Necessary configuration when deploying: 
    /// `ping_amount` - the exact amount that needs to be sent when `ping`-ing.   
    /// `duration_in_seconds` - how much time (in seconds) until `pong` can be called after the initial `ping` call   
    /// `token_id` - Optional. The Token Identifier of the token that is going to be used. Default is "EGLD". 
    pub fn init<
        Arg0: ProxyArg<BigUint<Env::Api>>,
        Arg1: ProxyArg<u64>,
        Arg2: ProxyArg<OptionalValue<EgldOrEsdtTokenIdentifier<Env::Api>>>,
    >(
        self,
        ping_amount: Arg0,
        duration_in_seconds: Arg1,
        opt_token_id: Arg2,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .argument(&ping_amount)
            .argument(&duration_in_seconds)
            .argument(&opt_token_id)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> PingPongProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn upgrade<
        Arg0: ProxyArg<BigUint<Env::Api>>,
        Arg1: ProxyArg<u64>,
    >(
        self,
        ping_amount: Arg0,
        duration_in_seconds: Arg1,
    ) -> TxTypedUpgrade<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_upgrade()
            .argument(&ping_amount)
            .argument(&duration_in_seconds)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> PingPongProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    /// User sends some tokens to be locked in the contract for a period of time. 
    pub fn ping(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("ping")
            .original_result()
    }

    /// User can take back funds from the contract. 
    /// Can only be called after expiration. 
    pub fn pong(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("pong")
            .original_result()
    }

    pub fn did_user_ping<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("didUserPing")
            .argument(&address)
            .original_result()
    }

    pub fn get_pong_enable_timestamp<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getPongEnableTimestamp")
            .argument(&address)
            .original_result()
    }

    pub fn get_time_to_pong<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, OptionalValue<u64>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getTimeToPong")
            .argument(&address)
            .original_result()
    }

    pub fn accepted_payment_token_id(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, EgldOrEsdtTokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getAcceptedPaymentToken")
            .original_result()
    }

    pub fn ping_amount(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getPingAmount")
            .original_result()
    }

    pub fn duration_in_seconds(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getDurationTimestamp")
            .original_result()
    }

    pub fn user_ping_timestamp<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        address: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getUserPingTimestamp")
            .argument(&address)
            .original_result()
    }
}
