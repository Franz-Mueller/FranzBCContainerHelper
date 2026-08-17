use crate::docker::image::BcImage;
use bollard::config::ContainerCreateBody;
use bollard::query_parameters::{CreateContainerOptions, ListImagesOptionsBuilder};

pub struct BcContainer {
    id: String,
}

pub struct ContainerBuilder {
    docker: bollard::Docker,
}

impl ContainerBuilder {
    pub fn new() -> Self {
        ContainerBuilder {
            docker: bollard::Docker::connect_with_defaults().unwrap(),
        }
    }

    pub async fn build(&self, image: &BcImage) -> Result<BcContainer, ContainerError> {
        let options = ListImagesOptionsBuilder::default().all(true).build();
        let images = self.docker.list_images(Some(options)).await?;
        let image_ids: Vec<String> = images.iter().map(|i| i.id.clone()).collect(); // TODO redo
        if !image_ids.contains(&image.id().to_string()) {
            return Err(ContainerError::ImageNotFound(image.id().to_string()));
        }
        self.execute_build(image).await?;

        Ok(BcContainer {
            id: String::from("LOL"),
        })
    }

    pub async fn execute_build(&self, image: &BcImage) -> Result<String, ContainerError> {
        let options = CreateContainerOptions::default();
        let mut config = ContainerCreateBody::default();
        config.image = Some(image.id().to_string());
        let container = self.docker.create_container(Some(options), config).await?;

        Ok(container.id)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ContainerError {
    #[error("docker causes an error: {0}")]
    BollardOperation(#[from] bollard::errors::Error),

    #[error("could not find image {0} in docker")]
    ImageNotFound(String),
}
