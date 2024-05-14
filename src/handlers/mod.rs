pub async fn event_loop(mut swarm: libp2p::Swarm<crate::protocols::L2MediatorBehaviour>) {
    loop {
        let next_event = libp2p::futures::StreamExt::select_next_some(&mut swarm).await;

        crate::info!("{next_event:?}");
    }
}
