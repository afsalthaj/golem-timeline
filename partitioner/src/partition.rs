use timeline::GolemEvent;
use timeline::GolemEventValue;

trait Paritioner {
    fn partition(&self, golem: GolemEvent<GolemEventValue>) -> Vec<Vec<usize>>;
}


struct DefaultPartitioner {
    num_workers: usize,
}

impl Paritioner for DefaultPartitioner {
    fn partition(&self, golem: GolemEvent<GolemEventValue>) -> Vec<Vec<usize>> {
        let mut partitions = vec![vec![]; self.num_workers];
        for (i, _) in golem.event.iter().enumerate() {
            partitions[i % self.num_workers].push(i);
        }
        partitions
    }
}
